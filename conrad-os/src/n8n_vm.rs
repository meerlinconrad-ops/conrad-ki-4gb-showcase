//! Eingebettete QuickJS-Sandbox fuer n8n-Expressions (`={{ ... }}`) und JS-Code-Nodes.
//!
//! Laedt einmalig pro VM-Instanz `luxon.min.js`, `jmespath.js` und `prelude.js` und stellt
//! `$json`, `$node`, `$()`, `$input`, `$items`, `$now`, `$today`, `$vars`, `$env`, `$execution`,
//! Luxon `DateTime`, `$jmespath` sowie sichere `Function`/`eval`-Sperren bereit.

use chrono::{Offset, TimeZone, Utc};
use chrono_tz::Tz;
use hmac::{Hmac, Mac};
use md5::{Digest as _, Md5};
use rquickjs::{function::Opt, Context, Function, Runtime, Value as JsValue};
use serde_json::{json, Map, Value};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, OnceLock,
};
use std::time::{Duration, Instant};
use thiserror::Error;

const LUXON_JS: &str = include_str!("n8n_js/luxon.min.js");
const JMESPATH_JS: &str = include_str!("n8n_js/jmespath.js");
const PRELUDE_JS: &str = include_str!("n8n_js/prelude.js");

const POOL_MAX: usize = 16;

#[derive(Debug, Error)]
pub enum VmError {
    #[error("JavaScript-Laufzeitfehler: {0}")]
    Js(String),
    #[error("Zeitlimit oder Speicherlimit in der JS-Sandbox ueberschritten")]
    Interrupted,
    #[error("Ungueltige Serialisierung: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct VmOptions {
    pub memory_limit: usize,
    pub stack_limit: usize,
    pub timeout: Duration,
    pub timezone: String,
    pub freeze_epoch_ms: Option<i64>,
    pub harden: bool,
}

impl Default for VmOptions {
    fn default() -> Self {
        Self {
            memory_limit: 32 * 1024 * 1024,
            stack_limit: 512 * 1024,
            timeout: Duration::from_millis(500),
            timezone: "Europe/Berlin".to_string(),
            freeze_epoch_ms: None,
            harden: true,
        }
    }
}

/// Berechnet MD5/SHA1/SHA256/SHA512 oder HMAC-Varianten fuer `__r8r_hash`.
pub fn digest(algo: &str, data: &str, secret: Option<&str>) -> Result<String, String> {
    let norm = algo.to_ascii_lowercase();
    if let Some(key) = secret {
        match norm.as_str() {
            "sha256" | "hmac-sha256" => {
                let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key.as_bytes())
                    .map_err(|e| e.to_string())?;
                mac.update(data.as_bytes());
                return Ok(hex::encode(mac.finalize().into_bytes()));
            }
            "sha512" | "hmac-sha512" => {
                let mut mac = <Hmac<Sha512> as Mac>::new_from_slice(key.as_bytes())
                    .map_err(|e| e.to_string())?;
                mac.update(data.as_bytes());
                return Ok(hex::encode(mac.finalize().into_bytes()));
            }
            "sha1" | "hmac-sha1" => {
                let mut mac = <Hmac<Sha1> as Mac>::new_from_slice(key.as_bytes())
                    .map_err(|e| e.to_string())?;
                mac.update(data.as_bytes());
                return Ok(hex::encode(mac.finalize().into_bytes()));
            }
            "md5" | "hmac-md5" => {
                let mut mac = <Hmac<Md5> as Mac>::new_from_slice(key.as_bytes())
                    .map_err(|e| e.to_string())?;
                mac.update(data.as_bytes());
                return Ok(hex::encode(mac.finalize().into_bytes()));
            }
            other => return Err(format!("Unbekannter HMAC-Algorithmus: {other}")),
        }
    }
    match norm.as_str() {
        "md5" => Ok(hex::encode(Md5::digest(data.as_bytes()))),
        "sha1" => Ok(hex::encode(Sha1::digest(data.as_bytes()))),
        "sha256" => Ok(hex::encode(Sha256::digest(data.as_bytes()))),
        "sha512" => Ok(hex::encode(Sha512::digest(data.as_bytes()))),
        other => Err(format!("Unbekannter Hash-Algorithmus: {other}")),
    }
}

fn tz_offset_minutes(tz_name: &str, epoch_ms: f64) -> i32 {
    if tz_name.eq_ignore_ascii_case("utc") || tz_name.is_empty() {
        return 0;
    }
    let Ok(tz) = tz_name.parse::<Tz>() else {
        return 0;
    };
    let secs = (epoch_ms / 1000.0).floor() as i64;
    let Some(utc_dt) = Utc.timestamp_opt(secs, 0).single() else {
        return 0;
    };
    utc_dt.with_timezone(&tz).offset().fix().local_minus_utc() / 60
}

fn tz_abbr_name(tz_name: &str, epoch_ms: f64) -> String {
    if tz_name.eq_ignore_ascii_case("utc") || tz_name.is_empty() {
        return "UTC".to_string();
    }
    let Ok(tz) = tz_name.parse::<Tz>() else {
        return "UTC".to_string();
    };
    let secs = (epoch_ms / 1000.0).floor() as i64;
    let Some(utc_dt) = Utc.timestamp_opt(secs, 0).single() else {
        return "UTC".to_string();
    };
    format!("{}", utc_dt.with_timezone(&tz).format("%Z"))
}

fn tz_is_valid(tz_name: &str) -> bool {
    tz_name.eq_ignore_ascii_case("utc") || tz_name.parse::<Tz>().is_ok()
}

pub struct Vm {
    rt: Runtime,
    ctx: Context,
    deadline: Arc<Mutex<Option<Instant>>>,
    timed_out: Arc<AtomicBool>,
    timeout: Duration,
    timezone: String,
    task_map: Arc<Mutex<HashMap<String, Vec<Value>>>>,
}

impl Vm {
    pub fn new(opts: VmOptions) -> Result<Self, VmError> {
        let rt = Runtime::new().map_err(|e| VmError::Js(e.to_string()))?;
        rt.set_memory_limit(opts.memory_limit);
        rt.set_max_stack_size(opts.stack_limit);

        let deadline = Arc::new(Mutex::new(None::<Instant>));
        let timed_out = Arc::new(AtomicBool::new(false));
        let dl_clone = deadline.clone();
        let to_clone = timed_out.clone();
        rt.set_interrupt_handler(Some(Box::new(move || {
            if let Ok(guard) = dl_clone.try_lock() {
                if let Some(limit) = *guard {
                    if Instant::now() > limit {
                        to_clone.store(true, Ordering::Relaxed);
                        return true;
                    }
                }
            }
            false
        })));

        let ctx = Context::full(&rt).map_err(|e| VmError::Js(e.to_string()))?;
        let task_map = Arc::new(Mutex::new(HashMap::<String, Vec<Value>>::new()));
        let tm_get = task_map.clone();
        let tm_cnt = task_map.clone();

        let tz_init = opts.timezone.clone();
        let freeze_init = opts.freeze_epoch_ms;

        ctx.with(|ctx| -> Result<(), VmError> {
            let g = ctx.globals();
            g.set("__r8r_timezone", tz_init.as_str())
                .map_err(|e| format_js_error(&ctx, "set_tz", e))?;
            match freeze_init {
                Some(ms) => g.set("__r8r_freeze", ms as f64),
                None => g.set("__r8r_freeze", JsValue::new_null(ctx.clone())),
            }
            .map_err(|e| format_js_error(&ctx, "set_freeze_init", e))?;

            g.set(
                "__r8r_tz_offset",
                Function::new(ctx.clone(), |tz: String, ms: f64| -> i32 {
                    tz_offset_minutes(&tz, ms)
                })
                .map_err(|e| format_js_error(&ctx, "fn_tz_offset", e))?,
            )
            .map_err(|e| format_js_error(&ctx, "set_tz_offset", e))?;

            g.set(
                "__r8r_tz_abbr",
                Function::new(ctx.clone(), |tz: String, ms: f64| -> String {
                    tz_abbr_name(&tz, ms)
                })
                .map_err(|e| format_js_error(&ctx, "fn_tz_abbr", e))?,
            )
            .map_err(|e| format_js_error(&ctx, "set_tz_abbr", e))?;

            g.set(
                "__r8r_tz_valid",
                Function::new(ctx.clone(), |tz: String| -> bool { tz_is_valid(&tz) })
                    .map_err(|e| format_js_error(&ctx, "fn_tz_valid", e))?,
            )
            .map_err(|e| format_js_error(&ctx, "set_tz_valid", e))?;

            g.set(
                "__r8r_hash",
                Function::new(
                    ctx.clone(),
                    |algo: String, data: String, secret: Opt<String>| -> String {
                        digest(&algo, &data, secret.0.as_deref()).unwrap_or_default()
                    },
                )
                .map_err(|e| format_js_error(&ctx, "fn_hash", e))?,
            )
            .map_err(|e| format_js_error(&ctx, "set_hash", e))?;

            g.set(
                "__r8r_task",
                Function::new(ctx.clone(), move |name: String, idx: Opt<i32>| -> Option<String> {
                    let guard = tm_get.lock().unwrap_or_else(|e| e.into_inner());
                    let items = guard.get(&name)?;
                    let _ = idx;
                    let wrapped: Vec<Value> = items
                        .iter()
                        .map(|it| if it.get("json").is_some() { it.clone() } else { json!({ "json": it }) })
                        .collect();
                    Some(json!({ "data": { "main": [wrapped] } }).to_string())
                })
                .map_err(|e| format_js_error(&ctx, "fn_task", e))?,
            )
            .map_err(|e| format_js_error(&ctx, "set_task", e))?;

            g.set(
                "__r8r_task_count",
                Function::new(ctx.clone(), move |name: String| -> i32 {
                    let guard = tm_cnt.lock().unwrap_or_else(|e| e.into_inner());
                    if guard.contains_key(&name) { 1 } else { -1 }
                })
                .map_err(|e| format_js_error(&ctx, "fn_task_count", e))?,
            )
            .map_err(|e| format_js_error(&ctx, "set_task_count", e))?;

            ctx.eval::<(), _>(LUXON_JS)
                .map_err(|e| format_js_error(&ctx, "luxon", e))?;
            ctx.eval::<(), _>(JMESPATH_JS)
                .map_err(|e| format_js_error(&ctx, "jmespath", e))?;
            ctx.eval::<(), _>(PRELUDE_JS)
                .map_err(|e| format_js_error(&ctx, "prelude", e))?;

            if opts.harden {
                let _ = ctx.eval::<(), _>("if (typeof __r8r_harden === 'function') __r8r_harden();");
            }
            Ok(())
        })?;

        Ok(Self {
            rt,
            ctx,
            deadline,
            timed_out,
            timeout: opts.timeout,
            timezone: opts.timezone,
            task_map,
        })
    }

    pub fn set_context(
        &mut self,
        node_outputs: &HashMap<String, Vec<Value>>,
        current_node: &str,
        item_index: usize,
        vars: &Value,
        execution_meta: &Value,
        freeze_epoch_ms: Option<i64>,
    ) -> Result<(), VmError> {
        {
            let mut map = self.task_map.lock().unwrap_or_else(|e| e.into_inner());
            map.clear();
            for (k, v) in node_outputs {
                map.insert(k.clone(), v.clone());
            }
        }
        let items = node_outputs
            .get(current_node)
            .cloned()
            .unwrap_or_else(|| vec![json!({})]);
        let wrapped_input: Vec<Value> = items
            .iter()
            .map(|it| {
                if it.get("json").is_some() {
                    it.clone()
                } else {
                    json!({ "json": it })
                }
            })
            .collect();
        let node_names: Vec<String> = node_outputs.keys().cloned().collect();
        let data_obj = json!({
            "input": wrapped_input,
            "inputs": [wrapped_input],
            "node": { "name": current_node },
            "nodeNames": node_names,
            "vars": vars,
            "env": {},
            "execution": execution_meta,
            "workflow": { "id": "conrad-os", "name": "Conrad Workflow" },
        });
        let data_json = serde_json::to_string(&data_obj)?;
        let tz_lit = serde_json::to_string(&self.timezone)?;
        let _ = freeze_epoch_ms;
        self.ctx.with(|ctx| -> Result<(), VmError> {
            let script = format!(
                r#"
                if (typeof __r8r_reset === 'function') __r8r_reset({tz_lit});
                if (typeof __r8r_set_data === 'function') {{
                    __r8r_set_data(JSON.parse({data_lit}));
                }}
                if (typeof __r8r_set_item === 'function') {{
                    __r8r_set_item({idx});
                }}
                "#,
                tz_lit = tz_lit,
                data_lit = serde_json::to_string(&data_json).unwrap_or_else(|_| "\"{}\"".into()),
                idx = item_index,
            );
            ctx.eval::<(), _>(script)
                .map_err(|e| format_js_error(&ctx, "set_context", e))?;
            Ok(())
        })
    }

    pub fn eval_expression(&mut self, expr: &str) -> Result<Value, VmError> {
        let _guard = TimeoutGuard::arm(&self.deadline, &self.timed_out, self.timeout);
        let out_str: Result<String, VmError> = self.ctx.with(|ctx| {
            let code = format!("__r8r_run(() => (\n{expr}\n))");
            ctx.eval::<String, _>(code)
                .map_err(|e| format_js_error(&ctx, "eval_expression", e))
        });

        if self.timed_out.load(Ordering::Relaxed) {
            return Err(VmError::Interrupted);
        }
        decode_r8r_envelope(&out_str?)
    }

    /// Fuehrt beliebigen Async/Sync Body fuer n8n JavaScript Code Nodes aus (`items`, `$input`, `$json`, `$node`, `$()`, `$vars`).
    pub fn eval_code_body(&mut self, body: &str, _mode: &str, _all_items: &[Value]) -> Result<Value, VmError> {
        let _guard = TimeoutGuard::arm(&self.deadline, &self.timed_out, Duration::from_millis(1200));

        let out_str: Result<String, VmError> = self
            .ctx
            .with(|ctx| -> Result<(), VmError> {
                let code = format!(
                    "__r8r_run_async(async () => {{\n  const items = $input.all();\n  const $items = (n, o, r) => $input.all();\n  {body}\n}})"
                );
                let _ = ctx
                    .eval::<String, _>(code)
                    .map_err(|e| format_js_error(&ctx, "eval_code_body", e))?;
                Ok(())
            })
            .and_then(|()| {
                while self.rt.is_job_pending() {
                    if self.timed_out.load(Ordering::Relaxed) {
                        return Err(VmError::Interrupted);
                    }
                    if let Err(e) = self.rt.execute_pending_job() {
                        return Err(VmError::Js(format!("job: {e:?}")));
                    }
                }
                self.ctx.with(|ctx| {
                    let s = ctx
                        .globals()
                        .get::<_, String>("__r8r_async_result")
                        .map_err(|e| format_js_error(&ctx, "eval_code_collect", e))?;
                    Ok(s)
                })
            });

        if self.timed_out.load(Ordering::Relaxed) {
            return Err(VmError::Interrupted);
        }
        decode_r8r_envelope(&out_str?)
    }
}

fn decode_r8r_envelope(s: &str) -> Result<Value, VmError> {
    let parsed: Value = serde_json::from_str(s)?;
    if let Some(err_obj) = parsed.get("e") {
        let msg = err_obj
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Unbekannter JS-Fehler");
        let name = err_obj
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Error");
        return Err(VmError::Js(format!("{name}: {msg}")));
    }
    if parsed.get("u").is_some() {
        return Ok(Value::Null);
    }
    Ok(parsed.get("v").cloned().unwrap_or(Value::Null))
}

struct TimeoutGuard {
    deadline: Arc<Mutex<Option<Instant>>>,
}

impl TimeoutGuard {
    fn arm(deadline: &Arc<Mutex<Option<Instant>>>, timed_out: &Arc<AtomicBool>, dur: Duration) -> Self {
        timed_out.store(false, Ordering::Relaxed);
        if let Ok(mut g) = deadline.lock() {
            *g = Some(Instant::now() + dur);
        }
        Self {
            deadline: deadline.clone(),
        }
    }
}

impl Drop for TimeoutGuard {
    fn drop(&mut self) {
        if let Ok(mut g) = self.deadline.lock() {
            *g = None;
        }
    }
}

fn format_js_error(ctx: &rquickjs::Ctx<'_>, stage: &str, err: rquickjs::Error) -> VmError {
    if matches!(err, rquickjs::Error::Exception) {
        let exc = ctx.catch();
        if let Some(ex) = exc.as_exception() {
            let msg = ex.message().unwrap_or_default();
            let stack = ex.stack().unwrap_or_default();
            return VmError::Js(format!("[{stage}] {msg} | {stack}"));
        }
        if let Some(obj) = exc.as_object() {
            let msg: String = obj.get("message").unwrap_or_default();
            let stack: String = obj.get("stack").unwrap_or_default();
            if !msg.is_empty() || !stack.is_empty() {
                return VmError::Js(format!("[{stage}] {msg} | {stack}"));
            }
        }
        if let Ok(s) = exc.get::<String>() {
            return VmError::Js(format!("[{stage}] {s}"));
        }
        return VmError::Js(format!("[{stage}] exc={exc:?} err={err}"));
    }
    VmError::Js(format!("[{stage}] {err}"))
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment<'a> {
    Literal(&'a str),
    Expr(&'a str),
}

/// Zerlegt `Hello {{ $json.name }}!` in Literale und Expression-Segmente unter Beachtung von Strings & verschachtelten `{}`.
pub fn split_template(input: &str) -> Vec<Segment<'_>> {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let mut pos = 0;
    let mut lit_start = 0;

    while pos + 1 < bytes.len() {
        if bytes[pos] == b'{' && bytes[pos + 1] == b'{' {
            if pos > lit_start {
                out.push(Segment::Literal(&input[lit_start..pos]));
            }
            let expr_start = pos + 2;
            let mut i = expr_start;
            let mut brace_depth = 0i32;
            let mut quote: Option<u8> = None;
            let mut escaped = false;
            let mut closed = None;

            while i < bytes.len() {
                let b = bytes[i];
                if let Some(q) = quote {
                    if escaped {
                        escaped = false;
                    } else if b == b'\\' {
                        escaped = true;
                    } else if b == q {
                        quote = None;
                    }
                    i += 1;
                    continue;
                }
                match b {
                    b'\'' | b'"' | b'`' => {
                        quote = Some(b);
                        i += 1;
                    }
                    b'{' => {
                        brace_depth += 1;
                        i += 1;
                    }
                    b'}' => {
                        if brace_depth > 0 {
                            brace_depth -= 1;
                            i += 1;
                        } else if i + 1 < bytes.len() && bytes[i + 1] == b'}' {
                            closed = Some(i);
                            i += 2;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    _ => i += 1,
                }
            }
            if let Some(end_idx) = closed {
                out.push(Segment::Expr(input[expr_start..end_idx].trim()));
                lit_start = i;
                pos = i;
            } else {
                out.push(Segment::Literal(&input[pos..]));
                return out;
            }
        } else {
            pos += 1;
        }
    }
    if lit_start < input.len() {
        out.push(Segment::Literal(&input[lit_start..]));
    }
    out
}

pub fn js_value_to_string(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

/// Thread-lokaler Pool gehaerteter VMs fuer schnelles Auswerten von `={{ ... }}`-Expressions.
static SHARED_POOL: OnceLock<Mutex<Vec<Vm>>> = OnceLock::new();

fn with_pooled_vm<F, R>(f: F) -> Result<R, VmError>
where
    F: FnOnce(&mut Vm) -> Result<R, VmError>,
{
    thread_local! {
        static LOCAL_VM: RefCell<Option<Vm>> = const { RefCell::new(None) };
    }
    // Check shared pool first for multi-thread Tokio reuse, fallback to fresh VM if empty
    let pool = SHARED_POOL.get_or_init(|| Mutex::new(Vec::with_capacity(POOL_MAX)));
    let mut vm = {
        let mut g = pool.lock().unwrap_or_else(|e| e.into_inner());
        g.pop()
    };
    if vm.is_none() {
        vm = Some(Vm::new(VmOptions::default())?);
    }
    let mut instance = vm.unwrap();
    let res = f(&mut instance);
    if res.is_ok() {
        if let Ok(mut g) = pool.lock() {
            if g.len() < POOL_MAX {
                g.push(instance);
            }
        }
    }
    res
}

/// Wertet einen n8n Expression-String (`={{ $json.a + 1 }}` oder `Hallo {{ $json.name }}`) mit QuickJS aus.
pub fn eval_n8n_template(
    raw: &str,
    node_outputs: &HashMap<String, Vec<Value>>,
    current_node: &str,
    item_index: usize,
    vars: &Value,
    execution_meta: &Value,
) -> Result<Value, VmError> {
    let expr_raw = match raw.strip_prefix('=') {
        Some(rest) => rest,
        None => return Ok(Value::String(raw.to_string())),
    };
    let segments = split_template(expr_raw);
    if segments.is_empty() {
        return Ok(Value::String(String::new()));
    }
    with_pooled_vm(|vm| {
        vm.set_context(node_outputs, current_node, item_index, vars, execution_meta, None)?;
        if segments.len() == 1 {
            if let Segment::Expr(code) = segments[0] {
                return vm.eval_expression(code);
            }
        }
        let mut out = String::new();
        for seg in segments {
            match seg {
                Segment::Literal(s) => out.push_str(s),
                Segment::Expr(code) => {
                    let v = vm.eval_expression(code)?;
                    out.push_str(&js_value_to_string(&v));
                }
            }
        }
        Ok(Value::String(out))
    })
}

/// Wertet rekursiv jedes JSON-Feld mit `=`-Praefix via QuickJS aus.
pub fn eval_n8n_json_value(
    val: &Value,
    node_outputs: &HashMap<String, Vec<Value>>,
    current_node: &str,
    item_index: usize,
    vars: &Value,
    execution_meta: &Value,
) -> Value {
    match val {
        Value::String(s) if s.starts_with('=') => {
            match eval_n8n_template(s, node_outputs, current_node, item_index, vars, execution_meta) {
                Ok(v) => v,
                Err(_) => Value::String(s.clone()),
            }
        }
        Value::Array(arr) => Value::Array(
            arr.iter()
                .map(|x| eval_n8n_json_value(x, node_outputs, current_node, item_index, vars, execution_meta))
                .collect(),
        ),
        Value::Object(map) => {
            let mut next = Map::new();
            for (k, v) in map {
                next.insert(
                    k.clone(),
                    eval_n8n_json_value(v, node_outputs, current_node, item_index, vars, execution_meta),
                );
            }
            Value::Object(next)
        }
        other => other.clone(),
    }
}

/// Normalisiert n8n Code-Node Rueckgabewerte auf `Vec<Value>` (Items im `{ json: ... }` bzw. rohen Objektformat).
pub fn eval_js_code_node(
    code: &str,
    mode: &str,
    items: &[Value],
    node_outputs: &HashMap<String, Vec<Value>>,
    current_node: &str,
    vars: &Value,
    execution_meta: &Value,
) -> Result<Vec<Value>, VmError> {
    with_pooled_vm(|vm| {
        if mode == "runOnceForEachItem" {
            let mut out = Vec::with_capacity(items.len());
            for (idx, it) in items.iter().enumerate() {
                vm.set_context(node_outputs, current_node, idx, vars, execution_meta, None)?;
                let wrapped = if it.get("json").is_some() {
                    it.clone()
                } else {
                    json!({ "json": it })
                };
                let res = vm.eval_code_body(code, mode, &[wrapped])?;
                collect_code_items(res, &mut out);
            }
            return Ok(out);
        }

        vm.set_context(node_outputs, current_node, 0, vars, execution_meta, None)?;
        let wrapped_items: Vec<Value> = items
            .iter()
            .map(|it| {
                if it.get("json").is_some() {
                    it.clone()
                } else {
                    json!({ "json": it })
                }
            })
            .collect();
        let res = vm.eval_code_body(code, mode, &wrapped_items)?;
        let mut out = Vec::new();
        collect_code_items(res, &mut out);
        Ok(out)
    })
}

fn collect_code_items(res: Value, out: &mut Vec<Value>) {
    match res {
        Value::Null => {}
        Value::Array(arr) => {
            for elem in arr {
                if let Some(j) = elem.get("json") {
                    out.push(j.clone());
                } else {
                    out.push(elem);
                }
            }
        }
        Value::Object(mut obj) => {
            if let Some(j) = obj.remove("json") {
                out.push(j);
            } else {
                out.push(Value::Object(obj));
            }
        }
        other => out.push(json!({ "result": other })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quickjs_expression_and_luxon_and_jmespath() {
        let mut outputs = HashMap::new();
        outputs.insert(
            "Trigger".to_string(),
            vec![json!({"price": 19.5, "qty": 4, "email": "Max@Example.de", "tags": [{"name": "vip"}]})],
        );
        let vars = json!({"company": "Conrad KI"});
        let exec = json!({"id": "exec-1", "mode": "manual"});

        let v = eval_n8n_template(
            "={{ $json.price * $json.qty }}",
            &outputs,
            "Trigger",
            0,
            &vars,
            &exec,
        )
        .unwrap();
        assert_eq!(v, json!(78));

        let t = eval_n8n_template(
            "=Hallo {{ $json.email.toLowerCase() }} - {{ $vars.company }}",
            &outputs,
            "Trigger",
            0,
            &vars,
            &exec,
        )
        .unwrap();
        assert_eq!(t, json!("Hallo max@example.de - Conrad KI"));

        let iso = eval_n8n_template("={{ $now.toISODate() }}", &outputs, "Trigger", 0, &vars, &exec).unwrap();
        assert!(iso.as_str().unwrap_or("").len() == 10);

        let jmes = eval_n8n_template("={{ $jmespath($json, 'tags[0].name') }}", &outputs, "Trigger", 0, &vars, &exec).unwrap();
        assert_eq!(jmes, json!("vip"));

        let escaped = eval_n8n_template("={{ eval('1+1') }}", &outputs, "Trigger", 0, &vars, &exec);
        assert!(escaped.is_err());
    }

    #[test]
    fn quickjs_code_node_runs_real_js() {
        let items = vec![json!({"name": "Alpha", "mrr": 99}), json!({"name": "Beta", "mrr": 49})];
        let mut outputs = HashMap::new();
        outputs.insert("Code".to_string(), items.clone());
        let out = eval_js_code_node(
            "return $input.all().map(i => ({ json: { ...i.json, total: i.json.mrr * 12 } }));",
            "runOnceForAllItems",
            &items,
            &outputs,
            "Code",
            &json!({}),
            &json!({}),
        )
        .unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0]["total"], json!(1188));
        assert_eq!(out[1]["total"], json!(588));
    }
}
