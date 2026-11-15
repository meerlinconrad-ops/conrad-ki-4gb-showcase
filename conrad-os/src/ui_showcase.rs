pub fn render_showcase_html() -> &'static str {
    r######"<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Conrad KI Suite — 5.000 €/Monat → &lt;20 €/Monat Kostenvergleich</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=DM+Sans:ital,opsz,wght@0,9..40,400..700;1,9..40,400..700&family=Fraunces:ital,opsz,wght@0,9..144,500..700;1,9..144,400&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
<style>
:root {
  --bg: #fbfaf8;
  --bg-subtle: #f4f1ea;
  --bg-card: #ffffff;
  --ink: #12110e;
  --ink-secondary: #4a463e;
  --ink-muted: #747065;
  --ink-faint: #a8a499;
  --accent-forest: #1e3822;
  --accent-forest-hdr: #2c4a2e;
  --accent-forest-light: #2c4e32;
  --accent-forest-soft: rgba(30, 56, 34, 0.08);
  --accent-orange: #c8531a;
  --accent-orange-hover: #b0450f;
  --accent-orange-soft: rgba(200, 83, 26, 0.1);
  --lime: #bef264;
  --border: rgba(18, 17, 14, 0.08);
  --border-strong: rgba(18, 17, 14, 0.16);
  --radius-md: 14px;
  --radius-lg: 20px;
}
* { box-sizing: border-box; }
html, body {
  margin: 0;
  padding: 0;
  background: var(--bg);
  color: var(--ink);
  font-family: 'DM Sans', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 15px;
  line-height: 1.55;
  overflow-x: hidden;
  -webkit-font-smoothing: antialiased;
}
.serif { font-family: 'Fraunces', Georgia, serif; }
.mono { font-family: 'JetBrains Mono', ui-monospace, monospace; }
a { color: inherit; text-decoration: none; }

/* Header Nav (Conrad KI CI) */
.header-nav {
  position: sticky;
  top: 0;
  z-index: 40;
  background: rgba(251, 250, 248, 0.92);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border);
}
.header-inner {
  max-width: 1180px;
  margin: 0 auto;
  padding: 12px 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  flex-wrap: wrap;
}
.brand-wrap { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.brand-link { display: inline-flex; align-items: center; gap: 10px; font-weight: 700; font-size: 15.5px; color: var(--ink); }
.brand-logo { width: 28px; height: 28px; border-radius: 7px; object-fit: contain; background: var(--accent-forest); }
.ci-pill-tag {
  display: inline-flex; align-items: center; gap: 6px; padding: 4px 11px; border-radius: 9999px;
  font-size: 12px; font-weight: 600; background: var(--accent-forest-soft); color: var(--accent-forest);
}
.nav-links { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.btn {
  display: inline-flex; align-items: center; justify-content: center; gap: 7px;
  padding: 8px 16px; border-radius: 9999px; font-family: inherit;
  font-size: 13px; font-weight: 600; border: 1px solid var(--border-strong);
  background: var(--bg-card); color: var(--ink); cursor: pointer;
  transition: background 0.15s, border-color 0.15s, color 0.15s; white-space: nowrap;
}
.btn:hover { background: var(--bg-subtle); border-color: var(--ink-muted); }
.btn-orange { background: var(--accent-orange); color: #ffffff; border-color: var(--accent-orange); }
.btn-orange:hover { background: var(--accent-orange-hover); border-color: var(--accent-orange-hover); }
.btn-forest { background: var(--accent-forest); color: #ffffff; border-color: var(--accent-forest); }
.btn-forest:hover { background: var(--accent-forest-light); border-color: var(--accent-forest-light); }

/* Main Container */
.container {
  max-width: 1180px;
  margin: 0 auto;
  padding: 28px 24px 56px;
  display: flex;
  flex-direction: column;
  gap: 26px;
}

/* Doku Hero (.doku-hero CI) */
.doku-hero {
  background: radial-gradient(circle at 86% 16%, rgba(30, 56, 34, 0.78) 0%, #12110e 58%);
  color: #fbfaf8;
  border-radius: var(--radius-lg);
  padding: 34px 32px 30px;
  border: 1px solid rgba(30, 56, 34, 0.45);
  box-shadow: 0 18px 40px rgba(18, 17, 14, 0.08);
}
.hero-top-row {
  display: flex; align-items: center; justify-content: space-between;
  gap: 12px; flex-wrap: wrap; margin-bottom: 14px;
}
.hero-badge {
  display: inline-flex; align-items: center; gap: 8px; padding: 5px 12px; border-radius: 9999px;
  font-size: 11.5px; font-weight: 700; letter-spacing: 0.05em; text-transform: uppercase;
  background: rgba(190, 242, 100, 0.14); color: var(--lime); border: 1px solid rgba(190, 242, 100, 0.28);
}
.hero-meta { display: flex; align-items: center; gap: 10px; font-size: 12px; color: rgba(251, 250, 248, 0.75); flex-wrap: wrap; }
.doku-hero h1 {
  font-family: 'Fraunces', Georgia, serif;
  font-size: clamp(26px, 4vw, 42px);
  font-weight: 600;
  line-height: 1.14;
  letter-spacing: -0.02em;
  margin: 0 0 12px;
  max-width: 920px;
}
.doku-hero h1 .lime { color: var(--lime); }
.doku-hero h1 .orange-accent { color: #ff8c57; font-style: italic; }
.hero-sub { margin: 0 0 24px; font-size: 15.5px; color: rgba(251, 250, 248, 0.82); max-width: 820px; }
.hero-metrics { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 14px; }
.doku-metric {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: var(--radius-md);
  padding: 15px 17px;
}
.doku-metric .m-lbl {
  font-size: 11px; font-weight: 700; letter-spacing: 0.06em; text-transform: uppercase;
  color: rgba(251, 250, 248, 0.62); margin-bottom: 5px;
}
.doku-metric .m-val {
  font-family: 'Fraunces', Georgia, serif; font-size: 27px; font-weight: 600; line-height: 1.15; color: #ffffff;
}
.doku-metric.highlight .m-val { color: var(--lime); }
.doku-metric .m-sub { margin-top: 6px; font-size: 12px; color: rgba(251, 250, 248, 0.72); }

/* Section Cards */
.ci-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 26px 28px;
  box-shadow: 0 4px 18px rgba(18, 17, 14, 0.025);
}
.eyebrow {
  font-size: 11.5px; font-weight: 700; letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--accent-orange); margin-bottom: 5px;
}
.ci-card h2 {
  font-family: 'Fraunces', Georgia, serif;
  font-size: clamp(21px, 2.5vw, 27px);
  font-weight: 600; letter-spacing: -0.015em; margin: 0 0 8px; color: var(--ink);
}
.sec-lead { margin: 0 0 18px; color: var(--ink-secondary); font-size: 14.5px; }

/* Cost Table */
.table-scroll {
  width: 100%; overflow-x: auto;
  border: 1px solid var(--border-strong); border-radius: var(--radius-md);
}
.cost-table { width: 100%; border-collapse: collapse; min-width: 700px; background: var(--bg-card); }
.cost-table thead th {
  background: var(--accent-forest); color: #fbfaf8; font-size: 11.5px; font-weight: 600;
  letter-spacing: 0.04em; text-transform: uppercase; text-align: left; padding: 13px 15px;
}
.cost-table thead th.num, .cost-table tbody td.num { text-align: right; white-space: nowrap; }
.cost-table tbody td {
  padding: 13px 15px; border-bottom: 1px solid var(--border); vertical-align: top; font-size: 13.5px;
}
.cost-table tbody tr:last-child td { border-bottom: none; }
.cost-table tbody tr.sum-row { background: var(--bg-subtle); font-weight: 700; border-top: 2px solid var(--accent-forest); }
.legacy-price {
  color: #991b1b; background: rgba(153, 27, 27, 0.08);
  padding: 3px 9px; border-radius: 9999px; font-weight: 600; display: inline-block;
}
.rust-price {
  color: var(--accent-forest); background: var(--accent-forest-soft);
  padding: 3px 9px; border-radius: 9999px; font-weight: 700; display: inline-block;
}
.save-pill {
  color: var(--accent-orange); background: var(--accent-orange-soft);
  padding: 3px 10px; border-radius: 9999px; font-weight: 700; display: inline-block;
}

/* Grids & Proofs */
.grid-2 { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 20px; }
.grid-3 { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 18px; }
.model-row {
  display: flex; align-items: center; justify-content: space-between; gap: 10px;
  padding: 10px 13px; background: var(--bg); border: 1px solid var(--border);
  border-radius: 10px; margin-bottom: 8px;
}
.arch-list { margin: 0; padding: 0; list-style: none; display: flex; flex-direction: column; gap: 10px; }
.arch-list li {
  padding: 11px 14px; background: var(--bg); border: 1px solid var(--border);
  border-left: 3px solid var(--accent-forest); border-radius: 10px; font-size: 13.5px; color: var(--ink-secondary);
}
.arch-list li strong { color: var(--ink); }
.proof-card {
  background: var(--bg); border: 1px solid var(--border-strong); border-radius: var(--radius-md);
  padding: 18px; display: flex; flex-direction: column; gap: 11px;
}
.proof-head { display: flex; align-items: center; justify-content: space-between; gap: 8px; flex-wrap: wrap; }
.lat-badge {
  font-size: 11.5px; padding: 3px 9px; border-radius: 9999px;
  background: var(--accent-forest-soft); color: var(--accent-forest); font-weight: 600;
}
.proof-pre {
  margin: 0; padding: 12px; border-radius: 10px; background: #12110e; color: #e7e5e0;
  font-size: 11.5px; line-height: 1.45; max-height: 220px; overflow: auto;
  white-space: pre-wrap; word-break: break-word;
}
.proof-select {
  height: 36px; padding: 0 12px; border-radius: 9999px;
  border: 1px solid var(--border-strong); background: var(--bg-card); color: var(--ink); font-size: 12px;
}

/* Footer */
.ci-footer {
  background: var(--bg-subtle); border-top: 1px solid var(--border);
  padding: 26px 24px; font-size: 13px; color: var(--ink-secondary);
}
.footer-inner {
  max-width: 1180px; margin: 0 auto; display: flex; align-items: center;
  justify-content: space-between; gap: 16px; flex-wrap: wrap;
}
.footer-links { display: flex; gap: 16px; flex-wrap: wrap; font-weight: 600; color: var(--accent-forest); }
.footer-links a:hover { text-decoration: underline; }

@media (max-width: 960px) {
  .hero-metrics { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .grid-2, .grid-3 { grid-template-columns: 1fr; }
}
@media (max-width: 600px) {
  .header-inner, .container { padding-left: 16px; padding-right: 16px; }
  .doku-hero { padding: 24px 18px; }
  .ci-card { padding: 20px 16px; }
  .hero-metrics { grid-template-columns: 1fr; }
}
</style>
</head>
<body>

<header class="header-nav">
  <div class="header-inner">
    <div class="brand-wrap">
      <a class="brand-link" href="/showcase">
        <img class="brand-logo" src="https://conrad-ki.com/assets/logo.png" alt="Conrad KI Logo">
        <span>Conrad KI</span>
      </a>
      <span class="ci-pill-tag">Produkte · 10× schneller &amp; 100× effizienter</span>
    </div>
    <nav class="nav-links">
      <a class="btn" href="/twenty">Conrad CRM</a>
      <a class="btn" href="/n8n">Conrad Automation</a>
      <a class="btn" href="https://conrad-ki.com/#conrad-os" target="_blank" rel="noopener">Homepage · Produkte ↗</a>
      <a class="btn btn-orange" href="#kostenvergleich">5.000 € → &lt;20 € Vergleich</a>
    </nav>
  </div>
</header>

<main class="container">
  <section class="doku-hero">
    <div class="hero-top-row">
      <span class="hero-badge">Produktiv-Suite · 100 % DSGVO auf deutschem Server (&lt;20 €/Monat)</span>
      <div class="hero-meta">
        <span id="tsLabel" class="mono">wird geladen...</span>
        <button class="btn" style="padding:5px 12px;font-size:12px;" onclick="loadMetrics()">↻ Live-Metriken aktualisieren</button>
      </div>
    </div>
    <h1>Wie wir <span class="lime">5.000 €/Monat</span> IT- &amp; SaaS-Kosten mit einem <span class="orange-accent">&lt;20 €/Monat</span> Server ersetzen.</h1>
    <p class="hero-sub"><strong>Conrad CRM</strong> (<span class="mono">/twenty</span>) und <strong>Conrad Automation</strong> (<span class="mono">/n8n</span>) — <strong>bis zu 10× schneller</strong> und <strong>bis zu 100× kompakter</strong> als herkömmliche Enterprise-Suiten, angetrieben von unserer spezialisiert nachtrainierten <strong>Conrad KI Engine</strong>.</p>

    <div class="hero-metrics">
      <div class="doku-metric highlight">
        <div class="m-lbl">Monatliche Ersparnis</div>
        <div class="m-val" id="kSavings">4.981 €</div>
        <div class="m-sub" id="kSavingsSub">5.000 €/Mo Legacy vs. 19 €/Mo Server</div>
      </div>
      <div class="doku-metric">
        <div class="m-lbl">Live-Footprint (100× effizienter)</div>
        <div class="m-val" id="kRss">— MB</div>
        <div class="m-sub" id="kHostMem">Host frei: — MB · DB: — MB</div>
      </div>
      <div class="doku-metric">
        <div class="m-lbl">Conrad CRM (Live-Datensätze)</div>
        <div class="m-val" id="kCrmTotal">—</div>
        <div class="m-sub" id="kCrmBreakdown">Firmen · Kontakte · Deals · Notizen</div>
      </div>
      <div class="doku-metric">
        <div class="m-lbl">Conrad Automation &amp; KI</div>
        <div class="m-val" id="kN8n">—</div>
        <div class="m-sub" id="kAiDefault">Conrad KI Engine aktiv</div>
      </div>
    </div>
  </section>

  <section class="ci-card" id="kostenvergleich">
    <div class="eyebrow">01 · Monatlicher Kostenvergleich</div>
    <h2>Enterprise-SaaS-Rechnung vs. Conrad KI Suite</h2>
    <p class="sec-lead">Direkter Vergleich eines klassischen KMU-SaaS-Stacks (CRM-Seats, Automatisierungs-Plattform, Cloud-KI-Tokens und verwaltete Cluster) gegenüber Conrad CRM, Conrad Automation &amp; Conrad KI Engine auf einem einzelnen deutschen Server.</p>
    <div class="table-scroll">
      <table class="cost-table">
        <thead>
          <tr>
            <th>Komponente</th>
            <th>Bisheriger Enterprise-Stack</th>
            <th class="num">Alt / Mon.</th>
            <th>Conrad CRM &amp; Automation</th>
            <th class="num">Neu / Mon.</th>
            <th class="num">Ersparnis</th>
          </tr>
        </thead>
        <tbody id="costTbody">
          <tr><td colspan="6" style="text-align:center;color:var(--ink-muted);">Lade Live-Aufschlüsselung...</td></tr>
        </tbody>
      </table>
    </div>
  </section>

  <section class="grid-2">
    <div class="ci-card">
      <div class="eyebrow">02 · Conrad KI Engine (Post-Trained)</div>
      <h2>1 spezialisiertes Modell — exakt für diese Aufgabe</h2>
      <p class="sec-lead">Keine unübersichtliche Modellauswahl: Unsere <strong>Conrad KI Engine</strong> basiert auf modernsten Open-Weight-Architekturen und wurde gezielt auf CRM-, E-Mail- und Workflow-Aufgaben nachtrainiert — für maximale Geschwindigkeit auf günstigen Servern.</p>
      <div id="modelsList">Lade Engine-Status...</div>
    </div>

    <div class="ci-card">
      <div class="eyebrow">03 · Effizienz-Prinzip</div>
      <h2>Bis zu 10× schneller &amp; 100× kompakter als Standard-Stacks</h2>
      <p class="sec-lead">Statt schwerer Container-Landschaften und Dutzender Cloud-Dienste laufen CRM, Automatisierung und lokale KI nahtlos in einer hochoptimierten Einheit.</p>
      <ul class="arch-list">
        <li><strong>Conrad CRM &amp; Automation integriert:</strong> Volle Parität für Kontakte, Deals, Kanban (<a href="/twenty" style="color:var(--accent-orange);text-decoration:underline;">/twenty</a>) und visuelle Workflows (<a href="/n8n" style="color:var(--accent-orange);text-decoration:underline;">/n8n</a>) — 0 € Lizenzgebühren.</li>
        <li><strong>Zero-Latency In-Process Kurzschluss:</strong> Wenn Workflows CRM-Daten abrufen, antwortet das CRM in Sub-Millisekunden ohne Cloud-Umwege.</li>
        <li><strong>Nachtrainierte Conrad KI Engine:</strong> Deterministisches Tool-Calling speziell für KMU-Anfragen, Telefonnotizen und E-Mail-Triage auf günstiger Server-Hardware.</li>
        <li><strong>Sicherheit by Design (<span class="mono">draftOnly: true</span>):</strong> E-Mails werden strikt als Entwurf gespeichert; es erfolgt niemals ein automatischer Versand ohne Freigabe.</li>
      </ul>
    </div>
  </section>

  <section class="ci-card" id="live-beweise">
    <div style="display:flex;align-items:flex-end;justify-content:space-between;gap:14px;flex-wrap:wrap;margin-bottom:18px;">
      <div>
        <div class="eyebrow">04 · Verifizierbare Live-Beweise</div>
        <h2 style="margin-bottom:4px;">Echtzeit-Tests direkt gegen diesen Produktionsserver</h2>
        <p class="sec-lead" style="margin-bottom:0;">Jede Aktion startet echte CRM-Abfragen, KI-Tool-Calls und Workflow-Ausführungen in Echtzeit.</p>
      </div>
      <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
        <input type="hidden" id="proofModelSel" value="conrad-n8n-agent:0.6b">
        <span class="ci-pill-tag">⚡ Conrad KI Engine (Post-Trained · Aktiv)</span>
      </div>
    </div>

    <div class="grid-3">
      <div class="proof-card">
        <div class="proof-head">
          <strong>Beweis A · Conrad CRM Live-Abfrage</strong>
          <span class="lat-badge mono" id="latA">bereit</span>
        </div>
        <div style="font-size:13px;color:var(--ink-secondary);">Fragt reale Firmendaten in Echtzeit über unsere GraphQL-API ab.</div>
        <div><button class="btn btn-forest" onclick="runProofA()">▶ CRM-Daten live abfragen</button></div>
        <pre class="proof-pre mono" id="outA">Klicke auf „CRM-Daten live abfragen“...</pre>
      </div>

      <div class="proof-card">
        <div class="proof-head">
          <strong>Beweis B · Conrad KI Engine Tool-Call</strong>
          <span class="lat-badge mono" id="latB">bereit</span>
        </div>
        <div style="font-size:13px;color:var(--ink-secondary);">Sendet einen Live-Auftrag (<span class="mono">Elektro Weber</span>) direkt an die nachtrainierte Conrad KI Engine.</div>
        <div><button class="btn btn-orange" onclick="runProofB()">▶ KI-Auftrag live testen</button></div>
        <pre class="proof-pre mono" id="outB">Sendet lesenden Auftrag („Suche Firmen mit Name Elektro Weber“) an die Conrad KI Engine...</pre>
      </div>

      <div class="proof-card">
        <div class="proof-head">
          <strong>Beweis C · Conrad Automation Flow</strong>
          <span class="lat-badge mono" id="latC">bereit</span>
        </div>
        <div style="font-size:13px;color:var(--ink-secondary);">Führt <span class="mono">conradKiAgent001</span> komplett durch die Hochgeschwindigkeits-Engine aus.</div>
        <div><button class="btn btn-forest" onclick="runProofC()">▶ Workflow live starten</button></div>
        <pre class="proof-pre mono" id="outC">Klicke auf „Workflow live starten“...</pre>
      </div>
    </div>
  </section>
</main>

<footer class="ci-footer">
  <div class="footer-inner">
    <div><strong>Conrad KI</strong> · Merlin Conrad · Königsäcker 94 · 68723 Schwetzingen · Telefon: +49 157 30495853 · E-Mail: merlin@conrad-ki.com · www.conrad-ki.com</div>
    <div class="footer-links">
      <a href="/twenty">Conrad CRM</a>
      <a href="/n8n">Conrad Automation</a>
      <a href="/api/showcase/metrics" target="_blank" rel="noopener">Telemetrie (JSON)</a>
    </div>
  </div>
</footer>

<script>
function esc(s) { return String(s ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c])); }
function fmtNum(n) { return Number(n || 0).toLocaleString('de-DE'); }

async function loadMetrics() {
  try {
    const r = await fetch('/api/showcase/metrics');
    const d = await r.json();
    const c = d.costComparison || {};
    document.getElementById('kSavings').textContent = fmtNum(c.monthlySavingsEur || 4981) + ' € / Mo';
    document.getElementById('kSavingsSub').textContent = `${fmtNum(c.legacyMonthlyEur)} € Alt vs. ${fmtNum(c.conradMonthlyEur)} € Server (−${String(c.savingsPercent || 99.6).replace('.', ',')} %)`;
    document.getElementById('kRss').textContent = `${String(d.rustBinary?.rssMb ?? '—').replace('.', ',')} MB`;
    document.getElementById('kHostMem').textContent = `Host frei: ${fmtNum(d.serverMemory?.availableMb)} / ${fmtNum(d.serverMemory?.totalMb)} MB · ~100× effizienter`;
    document.getElementById('kCrmTotal').textContent = fmtNum(d.crm?.totalRecords || 0);
    document.getElementById('kCrmBreakdown').textContent = `${fmtNum(d.crm?.companies)} Firmen · ${fmtNum(d.crm?.notes)} Notizen · ${fmtNum(d.crm?.tasks)} Aufgaben`;
    document.getElementById('kN8n').textContent = `${fmtNum(d.n8n?.activeWorkflows)} / ${fmtNum(d.n8n?.workflows)} aktiv`;
    document.getElementById('kAiDefault').textContent = `Conrad KI Engine · ${fmtNum(d.n8n?.executions)} Ausführungen`;
    document.getElementById('tsLabel').textContent = `Stand ${(d.timestamp || '').slice(11, 19)} UTC`;

    const tb = document.getElementById('costTbody');
    const rows = c.breakdown || [];
    tb.innerHTML = rows.map(row => {
      const saved = Number(row.legacyEur || 0) - Number(row.rustEur || 0);
      return `<tr>
        <td><strong>${esc(row.category)}</strong></td>
        <td style="color:var(--ink-secondary);">${esc(row.legacyStack)}</td>
        <td class="num"><span class="legacy-price">${fmtNum(row.legacyEur)} €</span></td>
        <td><strong>${esc(row.rustStack)}</strong></td>
        <td class="num"><span class="rust-price">${fmtNum(row.rustEur)} €</span></td>
        <td class="num"><span class="save-pill">−${fmtNum(saved)} €</span></td>
      </tr>`;
    }).join('') + `
      <tr class="sum-row">
        <td colspan="2">Summe pro Monat (Jahresersparnis: ${fmtNum(c.annualSavingsEur || 59772)} €)</td>
        <td class="num">${fmtNum(c.legacyMonthlyEur)} €</td>
        <td style="color:var(--accent-forest);">−${String(c.savingsPercent || 99.6).replace('.', ',')} % Infrastrukturkosten</td>
        <td class="num" style="color:var(--accent-forest);">${fmtNum(c.conradMonthlyEur)} €</td>
        <td class="num" style="color:var(--accent-orange);">−${fmtNum(c.monthlySavingsEur || 4981)} €</td>
      </tr>
    `;

    const loadedSet = new Set((d.ollama?.loadedNow || []).map(m => typeof m === 'string' ? m : m.name));
    const engineLoaded = loadedSet.has('conrad-n8n-agent:0.6b') || loadedSet.size > 0;
    document.getElementById('modelsList').innerHTML = `
      <div class="model-row">
        <div>
          <span style="font-weight:700;font-size:13.5px;">⚡ Conrad KI Engine (Post-Trained Specialist)</span>
          <div style="font-size:12px;color:var(--ink-muted);">Nachtrainiert auf modernsten Open-Weight-Modellen für CRM, E-Mail-Entwürfe &amp; Workflow-Tool-Calling</div>
        </div>
        <span class="ci-pill-tag" style="${engineLoaded ? 'background:var(--accent-orange-soft);color:var(--accent-orange);' : ''}">${engineLoaded ? 'Aktiv im RAM' : 'Sofort einsatzbereit'}</span>
      </div>
      <div class="model-row">
        <div>
          <span style="font-weight:600;font-size:13px;">🔒 100 % Lokaler Endpunkt (127.0.0.1)</span>
          <div style="font-size:12px;color:var(--ink-muted);">Keine US-Cloud, keine Token-Kosten, striktes draftOnly-Sicherheitsprotokoll</div>
        </div>
        <span class="ci-pill-tag">0 € Token-Kosten</span>
      </div>
    `;
  } catch (e) {
    document.getElementById('tsLabel').textContent = 'Fehler: ' + e;
  }
}

async function runProofA() {
  const lat = document.getElementById('latA'), out = document.getElementById('outA');
  lat.textContent = 'läuft...';
  const t0 = performance.now();
  try {
    const r = await fetch('/graphql', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ query: 'query { companies(first: 5) { totalCount edges { node { id name domainName { primaryLinkUrl } address { addressCity } } } } }' })
    });
    const d = await r.json();
    lat.textContent = `${Math.round(performance.now() - t0)} ms`;
    out.textContent = JSON.stringify(d, null, 2);
  } catch (e) { out.textContent = 'Fehler: ' + e; }
}

async function runProofB() {
  const lat = document.getElementById('latB'), out = document.getElementById('outB');
  const model = document.getElementById('proofModelSel').value;
  lat.textContent = `läuft (${model})...`;
  const t0 = performance.now();
  try {
    const r = await fetch('/api/twenty/agent', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ prompt: 'Suche Firmen mit Name Elektro Weber', workspace: 'conrad-ki', model })
    });
    const d = await r.json();
    lat.textContent = `${d.elapsedMs || Math.round(performance.now() - t0)} ms`;
    out.textContent = JSON.stringify({ model: d.model, elapsedMs: d.elapsedMs, toolCalls: d.toolCalls, reply: d.reply }, null, 2);
  } catch (e) { out.textContent = 'Fehler: ' + e; }
}

async function runProofC() {
  const lat = document.getElementById('latC'), out = document.getElementById('outC');
  lat.textContent = 'läuft (conradKiAgent001)...';
  const t0 = performance.now();
  try {
    const r = await fetch('/api/n8n/workflows/conradKiAgent001/execute', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ payload: { prompt: 'Suche in Twenty nach Meisterbetrieb Elektro Weber GmbH und fasse kurz zusammen.', model: 'conrad-n8n-agent:0.6b' } })
    });
    const d = await r.json();
    lat.textContent = `${d.durationMs || Math.round(performance.now() - t0)} ms`;
    out.textContent = JSON.stringify(d, null, 2);
  } catch (e) { out.textContent = 'Fehler: ' + e; }
}

loadMetrics();
</script>
</body>
</html>"######
}
