pub fn render_n8n_studio_html() -> &'static str {
    r######"<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Conrad Automation — Conrad KI</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
<style>
:root {
  --n8n-coral: #ff6d5a;
  --n8n-coral-hover: #f25540;
  --n8n-coral-soft: rgba(255, 109, 90, 0.12);
  --n8n-green: #25a56a;
  --n8n-green-soft: rgba(37, 165, 106, 0.13);
  --n8n-red: #e04f5f;
  --n8n-red-soft: rgba(224, 79, 95, 0.12);
  --n8n-violet: #7c3aed;
  --n8n-violet-soft: rgba(124, 58, 237, 0.12);
  --n8n-blue: #4078f2;
  --n8n-select: #5b60e8;
  --n8n-select-ring: rgba(91, 96, 232, 0.16);
  --n8n-sidebar-bg: #242529;
  --n8n-sidebar-border: #34353b;
  --n8n-sidebar-hover: #323339;
  --n8n-sidebar-active: #3c3d45;
  --n8n-sidebar-text: #d4d4d8;
  --n8n-header-bg: #ffffff;
  --n8n-canvas-bg: #f4f5f7;
  --n8n-canvas-dot: #cfd4df;
  --n8n-node-bg: #ffffff;
  --n8n-node-border: rgba(15, 23, 42, 0.14);
  --n8n-panel-bg: #ffffff;
  --n8n-subtle-bg: #f8f9fb;
  --n8n-border: #e2e4ea;
  --n8n-text-1: #1f2329;
  --n8n-text-2: #5b626e;
  --n8n-text-3: #8c93a1;
  --n8n-wire: #9ca3af;
  --n8n-shadow: 0 2px 6px rgba(15, 23, 42, 0.06);
  --n8n-shadow-lg: 0 18px 48px rgba(15, 23, 42, 0.18);
}
html.dark {
  --n8n-header-bg: #222326;
  --n8n-canvas-bg: #1c1c1f;
  --n8n-canvas-dot: #34343a;
  --n8n-node-bg: #2b2c30;
  --n8n-node-border: rgba(255, 255, 255, 0.16);
  --n8n-panel-bg: #242528;
  --n8n-subtle-bg: #2c2d31;
  --n8n-border: #383940;
  --n8n-text-1: #f0f1f4;
  --n8n-text-2: #a8abb2;
  --n8n-text-3: #787b82;
  --n8n-wire: #6b7280;
}
* { box-sizing: border-box; }
html, body {
  margin: 0; padding: 0; height: 100%; overflow: hidden;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 13px; color: var(--n8n-text-1); background: var(--n8n-canvas-bg);
}
.mono { font-family: 'JetBrains Mono', ui-monospace, monospace; }
button, input, select, textarea { font-family: inherit; font-size: inherit; color: inherit; }
a { color: inherit; text-decoration: none; }

.n8n-shell { display: flex; width: 100vw; height: 100vh; overflow: hidden; }

/* Left Dark Sidebar */
.n8n-nav {
  width: 216px; min-width: 216px; background: var(--n8n-sidebar-bg); color: var(--n8n-sidebar-text);
  border-right: 1px solid var(--n8n-sidebar-border); display: flex; flex-direction: column;
  transition: width 0.15s ease, min-width 0.15s ease, margin-left 0.15s ease; z-index: 40; user-select: none;
}
.n8n-nav.collapsed { width: 54px; min-width: 54px; }
.n8n-nav.collapsed .nav-txt, .n8n-nav.collapsed .nav-badge, .n8n-nav.collapsed .brand-wordmark { display: none; }

.n8n-nav-top {
  height: 52px; padding: 0 12px; display: flex; align-items: center; justify-content: space-between;
  border-bottom: 1px solid var(--n8n-sidebar-border);
}
.n8n-brand { display: flex; align-items: center; gap: 9px; font-weight: 700; font-size: 14px; color: #fff; cursor: pointer; min-width: 0; }
.n8n-nav-btn-icon {
  width: 26px; height: 26px; border-radius: 5px; border: none; background: transparent;
  color: #a1a1aa; cursor: pointer; display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;
}
.n8n-nav-btn-icon:hover { background: var(--n8n-sidebar-hover); color: #fff; }

.n8n-create-wrap { padding: 10px 10px 6px; }
.n8n-create-btn {
  width: 100%; height: 32px; border-radius: 6px; border: 1px solid #484951;
  background: #303137; color: #ffffff; font-weight: 600; font-size: 12.5px; cursor: pointer;
  display: flex; align-items: center; justify-content: center; gap: 6px; transition: all 0.12s;
}
.n8n-create-btn:hover { background: var(--n8n-coral); border-color: var(--n8n-coral); }

.n8n-nav-list { flex: 1; overflow-y: auto; padding: 4px 8px; display: flex; flex-direction: column; gap: 2px; }
.n8n-nav-item {
  height: 33px; padding: 0 9px; border-radius: 6px; border: none; background: transparent;
  color: var(--n8n-sidebar-text); font-size: 13px; font-weight: 500; cursor: pointer;
  display: flex; align-items: center; justify-content: space-between; gap: 8px; width: 100%; text-align: left;
}
.n8n-nav-item:hover { background: var(--n8n-sidebar-hover); color: #fff; }
.n8n-nav-item.active { background: var(--n8n-sidebar-active); color: #fff; font-weight: 600; }
.n8n-nav-left { display: flex; align-items: center; gap: 9px; min-width: 0; }
.nav-badge { font-size: 10.5px; background: #3f3f46; color: #e4e4e7; padding: 1px 6px; border-radius: 99px; }

.n8n-nav-foot {
  padding: 8px; border-top: 1px solid var(--n8n-sidebar-border);
  display: flex; flex-direction: column; gap: 2px;
}
.n8n-user-row { padding: 6px 8px; display: flex; align-items: center; gap: 8px; border-radius: 6px; }
.n8n-avatar {
  width: 24px; height: 24px; border-radius: 99px; background: var(--n8n-coral); color: #fff;
  font-size: 10.5px; font-weight: 700; display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;
}

/* Main Workspace */
.n8n-main { flex: 1; min-width: 0; display: flex; flex-direction: column; position: relative; overflow: hidden; }

/* Top 52px Workflow Header Bar */
.n8n-hdr {
  height: 52px; min-height: 52px; padding: 0 14px; background: var(--n8n-header-bg);
  border-bottom: 1px solid var(--n8n-border); display: flex; align-items: center; justify-content: space-between; gap: 10px;
  z-index: 30;
}
.n8n-hdr-left { display: flex; align-items: center; gap: 7px; min-width: 0; flex: 1; }
.n8n-title-wrap { display: flex; align-items: center; gap: 6px; min-width: 0; overflow: hidden; flex: 1; }
.n8n-crumb-btn {
  border: none; background: transparent; color: var(--n8n-text-2); font-size: 13px; cursor: pointer;
  padding: 4px 6px; border-radius: 5px; display: inline-flex; align-items: center; gap: 4px; white-space: nowrap;
}
.n8n-crumb-btn:hover { background: var(--n8n-subtle-bg); color: var(--n8n-text-1); }
.n8n-title-inp {
  min-width: 140px; flex: 0 1 260px; max-width: 260px; border: 1px solid transparent; border-radius: 5px;
  background: transparent; font-weight: 600; font-size: 13.5px; padding: 4px 7px; outline: none;
  text-overflow: ellipsis; overflow: hidden;
}
.n8n-title-inp:hover { border-color: var(--n8n-border); }
.n8n-title-inp:focus { border-color: var(--n8n-coral); background: var(--n8n-panel-bg); }
.n8n-tag-pill, .tag-pill {
  height: 22px; padding: 0 8px; border-radius: 99px; border: 1px solid var(--n8n-border);
  background: var(--n8n-subtle-bg); color: var(--n8n-text-2); font-size: 11px; font-weight: 500;
  cursor: pointer; display: inline-flex; align-items: center; gap: 4px; white-space: nowrap; flex-shrink: 0;
}
.n8n-tag-pill:hover { border-color: var(--n8n-text-3); color: var(--n8n-text-1); }

/* Center Segmented Switcher */
.n8n-seg {
  display: inline-flex; background: var(--n8n-subtle-bg); border: 1px solid var(--n8n-border);
  border-radius: 7px; padding: 2px; gap: 2px;
}
.n8n-seg-btn {
  height: 26px; padding: 0 11px; border-radius: 5px; border: none; background: transparent;
  font-size: 12px; font-weight: 500; color: var(--n8n-text-2); cursor: pointer; white-space: nowrap;
}
.n8n-seg-btn.active { background: var(--n8n-panel-bg); color: var(--n8n-text-1); font-weight: 600; box-shadow: var(--n8n-shadow); }

.n8n-hdr-right { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.n8n-active-wrap { display: inline-flex; align-items: center; gap: 6px; font-size: 12px; color: var(--n8n-text-2); cursor: pointer; }
.n8n-switch {
  width: 34px; height: 19px; border-radius: 99px; background: #cbd5e1; position: relative;
  transition: background 0.15s ease; border: none; cursor: pointer; padding: 0; flex-shrink: 0;
}
.n8n-switch.on { background: var(--n8n-green); }
.n8n-switch::after {
  content: ""; position: absolute; top: 2.5px; left: 2.5px; width: 14px; height: 14px;
  border-radius: 50%; background: #fff; transition: transform 0.15s ease; box-shadow: 0 1px 2px rgba(0,0,0,0.2);
}
.n8n-switch.on::after { transform: translateX(15px); }

.n8n-btn {
  height: 30px; padding: 0 11px; border-radius: 6px; border: 1px solid var(--n8n-border);
  background: var(--n8n-panel-bg); color: var(--n8n-text-1); font-size: 12px; font-weight: 500;
  cursor: pointer; display: inline-flex; align-items: center; gap: 5px; white-space: nowrap;
}
.n8n-btn:hover { background: var(--n8n-subtle-bg); }
.n8n-btn-coral { background: var(--n8n-coral); color: #fff; border-color: var(--n8n-coral); font-weight: 600; }
.n8n-btn-coral:hover { background: var(--n8n-coral-hover); }
.n8n-btn-ghost { border-color: transparent; background: transparent; }

.n8n-pop {
  position: absolute; z-index: 95; min-width: 225px; background: var(--n8n-panel-bg);
  border: 1px solid var(--n8n-border); border-radius: 8px; box-shadow: var(--n8n-shadow-lg); padding: 5px; display: none;
}
.n8n-pop.open { display: block; }
.n8n-pop-item {
  width: 100%; border: none; background: transparent; padding: 7px 9px; border-radius: 5px;
  display: flex; align-items: center; justify-content: space-between; gap: 8px; cursor: pointer;
  font-size: 12.5px; color: var(--n8n-text-1); text-align: left;
}
.n8n-pop-item:hover, .n8n-pop-item.active { background: var(--n8n-subtle-bg); }

/* Execution Banner when viewing a past execution on canvas */
.n8n-exec-banner {
  display: none; align-items: center; justify-content: space-between; gap: 10px;
  padding: 6px 14px; background: var(--n8n-violet-soft); border-bottom: 1px solid rgba(124,58,237,0.28);
  font-size: 12px; color: var(--n8n-text-1);
}
.n8n-exec-banner.open { display: flex; }

/* Canvas Stage */
.n8n-stage { flex: 1; min-height: 0; position: relative; overflow: hidden; display: flex; flex-direction: column; }
.n8n-canvas {
  flex: 1; position: relative; overflow: hidden;
  background-color: var(--n8n-canvas-bg);
  background-image: radial-gradient(var(--n8n-canvas-dot) 1.3px, transparent 1.3px);
  background-size: 20px 20px;
  cursor: grab;
}
.n8n-canvas.select-mode { cursor: crosshair; }
.n8n-canvas.panning { cursor: grabbing; }
.n8n-viewport { position: absolute; inset: 0; transform-origin: 0 0; }
svg.n8n-wires { position: absolute; top: 0; left: 0; width: 8000px; height: 6000px; pointer-events: none; overflow: visible; }
.wire-hit { fill: none; stroke: transparent; stroke-width: 18px; pointer-events: stroke; cursor: pointer; }
.wire-path { fill: none; stroke: var(--n8n-wire); stroke-width: 2px; pointer-events: none; transition: stroke 0.12s, stroke-width 0.12s; }
.wire-group:hover .wire-path { stroke: var(--n8n-coral); stroke-width: 3px; }
.wire-path.executed { stroke: var(--n8n-green); stroke-width: 2.3px; }
.wire-path.pinned { stroke: var(--n8n-violet); stroke-width: 2.3px; }
.wire-path.ai { stroke: #9ca3af; stroke-width: 1.8px; stroke-dasharray: 5 5; }
.wire-path.draft { stroke: var(--n8n-coral); stroke-width: 2.4px; stroke-dasharray: 5 4; pointer-events: none; }

/* Edge Hover Toolbar (+ Insert / Trash) */
.n8n-edge-tb {
  position: absolute; transform: translate(-50%, -50%); display: none; align-items: center; gap: 4px;
  z-index: 22; pointer-events: all; padding: 3px;
}
.n8n-edge-tb.visible { display: flex; }
.n8n-edge-btn {
  width: 24px; height: 24px; border-radius: 5px; border: 1px solid var(--n8n-border);
  background: var(--n8n-panel-bg); color: var(--n8n-text-2); box-shadow: var(--n8n-shadow);
  cursor: pointer; display: inline-flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600;
}
.n8n-edge-btn:hover { color: var(--n8n-coral); border-color: var(--n8n-coral); background: var(--n8n-subtle-bg); }
.n8n-edge-btn.del:hover { color: var(--n8n-red); border-color: var(--n8n-red); }

/* Selection Marquee */
.n8n-marquee {
  position: absolute; border: 1.5px solid var(--n8n-select); background: rgba(91, 96, 232, 0.1);
  border-radius: 3px; pointer-events: none; z-index: 35; display: none;
}

/* Authentic Canvas Nodes */
.n8n-node-wrap { position: absolute; user-select: none; z-index: 10; }
.n8n-node-wrap:hover, .n8n-node-wrap.is-selected { z-index: 15; }
.n8n-node-box {
  position: relative; background: var(--n8n-node-bg); border: 1.5px solid var(--n8n-node-border);
  box-shadow: var(--n8n-shadow); transition: border-color 0.12s, box-shadow 0.12s;
  display: flex; align-items: center; justify-content: center; cursor: pointer;
}
.n8n-node-box.standard { width: 96px; height: 96px; border-radius: 8px; }
.n8n-node-box.trigger { width: 96px; height: 96px; border-radius: 36px 8px 8px 36px; }
.n8n-node-box.agent-root {
  width: 224px; height: 96px; border-radius: 8px; justify-content: flex-start; padding: 0 16px; gap: 12px;
}
.n8n-node-box.subnode { width: 68px; height: 68px; border-radius: 50%; }
.n8n-node-box:hover { border-color: var(--n8n-text-3); }
.n8n-node-box.selected {
  border-color: var(--n8n-select);
  box-shadow: 0 0 0 6px var(--n8n-select-ring);
}
.n8n-node-box.status-success { border: 2px solid var(--n8n-green); }
.n8n-node-box.status-error { border: 2px solid var(--n8n-red); }
.n8n-node-box.pinned-node { border: 2px solid var(--n8n-violet); }
.n8n-node-box.disabled-node { opacity: 0.52; }
.n8n-disabled-strike {
  position: absolute; left: -12px; right: -12px; top: 50%; height: 2px;
  background: var(--n8n-wire); pointer-events: none; transform: translateY(-50%);
}

.n8n-trig-bolt {
  position: absolute; left: -11px; top: 50%; transform: translateY(-50%);
  width: 22px; height: 22px; border-radius: 50%; background: var(--n8n-coral); color: #fff;
  display: flex; align-items: center; justify-content: center; border: 2px solid var(--n8n-canvas-bg);
  font-size: 11px; box-shadow: var(--n8n-shadow); pointer-events: none;
}
.n8n-exec-badge {
  position: absolute; right: 6px; bottom: 6px; width: 18px; height: 18px; border-radius: 50%;
  background: var(--n8n-green); color: #fff; font-size: 10px; font-weight: 700;
  display: flex; align-items: center; justify-content: center; pointer-events: none;
}
.n8n-exec-badge.err { background: var(--n8n-red); }
.n8n-items-pill {
  position: absolute; top: -10px; right: 6px; padding: 1px 6px; border-radius: 99px;
  background: var(--n8n-green); color: #fff; font-size: 9.5px; font-weight: 600; white-space: nowrap; pointer-events: none;
}
.n8n-items-pill.err { background: var(--n8n-red); }
.n8n-pin-pill {
  position: absolute; top: -10px; left: 6px; background: var(--n8n-violet); color: #fff;
  border-radius: 99px; padding: 1px 6px; font-size: 9.5px; font-weight: 600; white-space: nowrap; pointer-events: none;
}

/* Node Hover Toolbar */
.n8n-node-toolbar {
  position: absolute; top: -31px; left: 50%; transform: translateX(-50%);
  background: var(--n8n-panel-bg); border: 1px solid var(--n8n-border); border-radius: 6px;
  box-shadow: var(--n8n-shadow); padding: 2px 4px; display: none; align-items: center; gap: 2px; z-index: 25;
}
.n8n-node-wrap:hover .n8n-node-toolbar { display: flex; }
.n8n-tb-btn {
  width: 22px; height: 22px; border: none; background: transparent; border-radius: 4px;
  cursor: pointer; font-size: 11.5px; color: var(--n8n-text-2); display: inline-flex; align-items: center; justify-content: center;
}
.n8n-tb-btn:hover { background: var(--n8n-subtle-bg); color: var(--n8n-coral); }
.n8n-tb-btn.del:hover { color: var(--n8n-red); }

/* Ports */
.n8n-port-in {
  position: absolute; left: -6px; top: 50%; transform: translateY(-50%);
  width: 11px; height: 11px; border-radius: 50%; background: var(--n8n-node-bg); border: 2px solid var(--n8n-wire);
  transition: transform 0.1s, border-color 0.1s, background 0.1s;
}
.n8n-port-in.snap { border-color: var(--n8n-coral); background: var(--n8n-coral); transform: translateY(-50%) scale(1.35); }
.n8n-port-out {
  position: absolute; right: -6px; width: 11px; height: 11px; border-radius: 50%;
  background: var(--n8n-node-bg); border: 2px solid var(--n8n-wire); cursor: crosshair; z-index: 3;
}
.n8n-port-out:hover { border-color: var(--n8n-coral); transform: scale(1.25); }
.n8n-port-stub {
  position: absolute; right: -22px; width: 16px; height: 2px; background: var(--n8n-wire); opacity: 0.45; pointer-events: none;
}
.n8n-port-add {
  position: absolute; right: -38px; width: 18px; height: 18px; border-radius: 4px;
  border: 1px solid var(--n8n-node-border); background: var(--n8n-node-bg); color: var(--n8n-text-2);
  font-size: 13px; line-height: 1; display: flex; align-items: center; justify-content: center;
  cursor: pointer; opacity: 0; transition: opacity 0.12s, border-color 0.12s; z-index: 4;
}
.n8n-port-add.unconnected { opacity: 0.92; }
.n8n-node-wrap:hover .n8n-port-add, .n8n-port-add:hover { opacity: 1; }
.n8n-port-add:hover { border-color: var(--n8n-coral); color: var(--n8n-coral); }
.n8n-branch-lbl {
  position: absolute; right: 10px; font-size: 9.5px; color: var(--n8n-text-3); font-weight: 600; pointer-events: none;
  background: var(--n8n-node-bg); padding: 0 2px; border-radius: 3px;
}
.n8n-diamond-ports {
  position: absolute; bottom: -21px; left: 12px; right: 12px; display: flex; justify-content: space-around;
}
.n8n-diamond { display: flex; flex-direction: column; align-items: center; gap: 2px; font-size: 9.5px; color: var(--n8n-text-3); cursor: pointer; }
.n8n-diamond-sq { width: 10px; height: 10px; background: var(--n8n-node-bg); border: 2px solid var(--n8n-wire); transform: rotate(45deg); transition: border-color 0.12s; }
.n8n-diamond:hover .n8n-diamond-sq { border-color: var(--n8n-coral); }

/* Node Label Centered Below Node */
.n8n-node-label {
  position: relative; width: 164px; margin-left: -34px; margin-top: 8px;
  text-align: center; pointer-events: none;
}
.n8n-node-wrap.agent-root-wrap .n8n-node-label { display: none; }
.n8n-node-wrap.subnode-wrap .n8n-node-label { width: 136px; margin-left: -34px; }
.n8n-node-title { font-size: 12.5px; font-weight: 600; color: var(--n8n-text-1); line-height: 1.25; word-break: break-word; }
.n8n-node-sub { font-size: 10.5px; color: var(--n8n-text-3); margin-top: 2px; }

/* Sticky Note */
.n8n-sticky {
  position: absolute; border-radius: 6px; padding: 12px; font-size: 12px; line-height: 1.48;
  box-shadow: var(--n8n-shadow); cursor: move; z-index: 4; display: flex; flex-direction: column;
}
.n8n-sticky.selected { box-shadow: 0 0 0 3px var(--n8n-select); }
.n8n-sticky-tb {
  position: absolute; top: -30px; right: 0; background: var(--n8n-panel-bg); border: 1px solid var(--n8n-border);
  border-radius: 6px; padding: 3px 5px; display: none; align-items: center; gap: 4px; box-shadow: var(--n8n-shadow); z-index: 16;
}
.n8n-sticky:hover .n8n-sticky-tb, .n8n-sticky.selected .n8n-sticky-tb { display: flex; }
.n8n-sticky-swatch { width: 14px; height: 14px; border-radius: 50%; border: 1px solid rgba(0,0,0,0.2); cursor: pointer; padding: 0; }
.n8n-sticky-content { flex: 1; overflow: auto; word-break: break-word; }
.n8n-sticky-content h1, .n8n-sticky-content h2, .n8n-sticky-content h3 { margin: 0 0 5px; font-size: 13.5px; }
.n8n-sticky-ed {
  flex: 1; width: 100%; border: none; background: transparent; resize: none; outline: none; font-size: 12px; line-height: 1.45;
}
.n8n-sticky-resize {
  position: absolute; right: 2px; bottom: 2px; width: 14px; height: 14px; cursor: nwse-resize; opacity: 0.45;
}
.n8n-sticky:hover .n8n-sticky-resize { opacity: 0.9; }

/* Floating Canvas Controls */
.n8n-float-plus {
  position: absolute; top: 16px; right: 16px; z-index: 20;
  width: 42px; height: 42px; border-radius: 8px; border: 1px solid var(--n8n-border);
  background: var(--n8n-panel-bg); color: var(--n8n-text-1); font-size: 20px; font-weight: 500;
  box-shadow: var(--n8n-shadow); cursor: pointer; display: flex; align-items: center; justify-content: center;
}
.n8n-float-plus:hover { border-color: var(--n8n-coral); color: var(--n8n-coral); }

.n8n-controls-bl {
  position: absolute; bottom: 16px; left: 16px; z-index: 20; display: flex; gap: 6px; align-items: center;
}
.n8n-sq-btn {
  width: 34px; height: 34px; border-radius: 6px; border: 1px solid var(--n8n-border);
  background: var(--n8n-panel-bg); color: var(--n8n-text-2); cursor: pointer; box-shadow: var(--n8n-shadow);
  display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600;
}
.n8n-sq-btn:hover { color: var(--n8n-text-1); border-color: var(--n8n-text-3); }
.n8n-sq-btn.active { border-color: var(--n8n-coral); color: var(--n8n-coral); background: var(--n8n-coral-soft); }

.n8n-minimap {
  position: absolute; bottom: 16px; right: 16px; z-index: 20; width: 170px; height: 108px;
  border-radius: 8px; border: 1px solid var(--n8n-border); background: rgba(255,255,255,0.9);
  box-shadow: var(--n8n-shadow); overflow: hidden; cursor: pointer;
  transition: top 0.14s ease, bottom 0.14s ease;
}
#canvasStage.logs-open .n8n-minimap { top: 68px; bottom: auto; }
html.dark .n8n-minimap { background: rgba(34,35,38,0.9); }

.n8n-test-bar {
  position: absolute; bottom: 16px; left: 50%; transform: translateX(-50%); z-index: 20;
  display: flex; align-items: center; gap: 6px; background: rgba(255,255,255,0.9);
  backdrop-filter: blur(8px); padding: 5px; border-radius: 10px; border: 1px solid var(--n8n-border);
  box-shadow: var(--n8n-shadow-lg);
}
html.dark .n8n-test-bar { background: rgba(34,35,38,0.9); }

/* Context Menu */
.n8n-ctx {
  position: fixed; z-index: 130; min-width: 210px; background: var(--n8n-panel-bg);
  border: 1px solid var(--n8n-border); border-radius: 8px; box-shadow: var(--n8n-shadow-lg); padding: 5px; display: none;
}
.n8n-ctx.open { display: block; }

/* Collapsible Bottom Logs Panel (n8n 2.x Split View) */
.n8n-bottom-logs {
  height: 0; border-top: 1px solid var(--n8n-border); background: var(--n8n-panel-bg);
  transition: height 0.14s ease; overflow: hidden; display: flex; flex-direction: column; z-index: 26; position: relative;
}
.n8n-bottom-logs.open { height: 260px; }
.n8n-logs-resizer {
  height: 5px; cursor: ns-resize; background: transparent; position: absolute; top: 0; left: 0; right: 0; z-index: 5;
}
.n8n-logs-resizer:hover { background: var(--n8n-coral); }

/* Slide-over Node Creator Drawer */
.n8n-creator {
  position: fixed; top: 52px; right: 0; bottom: 0; width: 356px; max-width: 96vw;
  background: var(--n8n-panel-bg); border-left: 1px solid var(--n8n-border); box-shadow: var(--n8n-shadow-lg);
  z-index: 60; display: none; flex-direction: column;
}
.n8n-creator.open { display: flex; }
.n8n-creator-item {
  width: 100%; padding: 8px 10px; border-radius: 7px; border: 1px solid transparent; background: transparent;
  display: flex; align-items: center; gap: 10px; cursor: pointer; text-align: left;
}
.n8n-creator-item:hover { background: var(--n8n-subtle-bg); border-color: var(--n8n-border); }

/* 3-Column NDV Modal */
.n8n-ndv-backdrop {
  position: fixed; inset: 0; background: rgba(15,23,42,0.55); backdrop-filter: blur(2px);
  z-index: 90; display: none; align-items: center; justify-content: center;
}
.n8n-ndv-backdrop.open { display: flex; }
.n8n-ndv {
  width: 95vw; max-width: 1400px; height: 88vh; background: var(--n8n-panel-bg);
  border: 1px solid var(--n8n-border); border-radius: 12px; box-shadow: var(--n8n-shadow-lg);
  display: flex; flex-direction: column; overflow: hidden;
}
.n8n-ndv-top {
  height: 46px; padding: 0 14px; border-bottom: 1px solid var(--n8n-border);
  display: flex; align-items: center; justify-content: space-between; background: var(--n8n-subtle-bg); gap: 10px;
}
.n8n-ndv-grid { flex: 1; min-height: 0; display: grid; grid-template-columns: 28% 44% 28%; }
.n8n-ndv-col { display: flex; flex-direction: column; min-height: 0; border-right: 1px solid var(--n8n-border); overflow: hidden; }
.n8n-ndv-col:last-child { border-right: none; }
.n8n-ndv-col-hdr {
  min-height: 44px; padding: 6px 12px; border-bottom: 1px solid var(--n8n-border);
  display: flex; align-items: center; justify-content: space-between; gap: 8px; flex-wrap: wrap;
}
.n8n-ndv-body { flex: 1; overflow: auto; padding: 12px 14px; display: flex; flex-direction: column; gap: 10px; }
.n8n-field-lbl { font-size: 11.5px; font-weight: 600; color: var(--n8n-text-2); display: inline-block; }
.n8n-param-hdr { display: flex; align-items: center; justify-content: space-between; gap: 8px; margin-bottom: 4px; }
.n8n-mode-toggle {
  display: inline-flex; border: 1px solid var(--n8n-border); border-radius: 5px; overflow: hidden; background: var(--n8n-subtle-bg);
}
.n8n-mode-btn {
  border: none; background: transparent; padding: 2px 7px; font-size: 10.5px; color: var(--n8n-text-2); cursor: pointer;
}
.n8n-mode-btn.active { background: var(--n8n-panel-bg); color: var(--n8n-coral); font-weight: 600; }
.n8n-input, .n8n-textarea, .n8n-select {
  width: 100%; padding: 7px 9px; border: 1px solid var(--n8n-border); border-radius: 6px;
  background: var(--n8n-subtle-bg); outline: none; font-size: 12.5px;
}
.n8n-input:focus, .n8n-textarea:focus, .n8n-select:focus { border-color: var(--n8n-coral); background: var(--n8n-panel-bg); }
.n8n-input.is-expr, .n8n-textarea.is-expr { border-color: #f59e0b; background: rgba(245, 158, 11, 0.06); }
.n8n-input.drop-over, .n8n-textarea.drop-over { border-color: var(--n8n-coral); box-shadow: 0 0 0 3px var(--n8n-coral-soft); }
.n8n-code-ed {
  width: 100%; padding: 10px; border-radius: 6px; background: #161b22; color: #e6edf3;
  border: 1px solid #30363d; font-size: 12px; line-height: 1.5; outline: none;
}
.n8n-expr-preview {
  margin-top: 4px; padding: 5px 8px; border-radius: 5px; font-size: 11px;
  background: var(--n8n-green-soft); color: var(--n8n-green); word-break: break-word;
}

/* Schema Tree Items (Draggable) */
.n8n-schema-row {
  display: flex; align-items: center; gap: 7px; padding: 5px 8px; border-radius: 6px;
  border: 1px solid var(--n8n-border); background: var(--n8n-subtle-bg); cursor: grab; font-size: 12px;
}
.n8n-schema-row:hover { border-color: var(--n8n-coral); }
.n8n-type-badge {
  font-size: 10px; font-weight: 700; min-width: 22px; text-align: center; padding: 1px 4px;
  border-radius: 4px; background: var(--n8n-panel-bg); border: 1px solid var(--n8n-border); color: var(--n8n-text-2);
}

/* Subpages */
.n8n-page { flex: 1; overflow: auto; padding: 24px 32px; background: var(--n8n-canvas-bg); }
.n8n-wf-card {
  background: var(--n8n-panel-bg); border: 1px solid var(--n8n-border); border-radius: 8px;
  padding: 13px 16px; display: flex; align-items: center; justify-content: space-between; gap: 12px;
  margin-bottom: 8px; cursor: pointer;
}
.n8n-wf-card:hover { border-color: var(--n8n-coral); }

.n8n-toast {
  position: fixed; bottom: 16px; right: 16px; z-index: 140; padding: 8px 13px; border-radius: 6px;
  background: #1f2329; color: #fff; font-size: 12.5px; font-weight: 500; box-shadow: var(--n8n-shadow-lg); display: none;
}
.n8n-toast.show { display: block; }

@media (max-width: 960px) {
  .n8n-nav { position: fixed; top: 0; left: 0; bottom: 0; }
  .n8n-nav.collapsed { margin-left: -216px; width: 216px; min-width: 216px; }
  .n8n-seg { display: none; }
  .n8n-ndv-grid { grid-template-columns: 1fr; overflow-y: auto; }
  .n8n-ndv-col { border-right: none; border-bottom: 1px solid var(--n8n-border); min-height: 220px; }
  .n8n-minimap { display: none; }
}
@media (max-width: 640px) {
  .n8n-hdr { padding: 0 8px; gap: 6px; }
  .n8n-hdr-left { min-width: 0; flex: 1; gap: 4px; overflow: hidden; }
  .n8n-title-inp { min-width: 0; width: 100%; max-width: none; flex: 1; font-size: 13px; padding: 4px; }
  .n8n-hdr-right { gap: 5px; }
  .n8n-hide-sm { display: none !important; }
  .n8n-controls-bl { display: none; }
  .n8n-test-bar { max-width: calc(100vw - 16px); padding: 4px; gap: 4px; bottom: 12px; }
  .n8n-test-bar .n8n-btn { padding: 0 9px; font-size: 12px; }
  .n8n-page { padding: 14px 12px; }
}
</style>
</head>
<body>
<div class="n8n-shell">
  <!-- Left Dark Sidebar -->
  <aside class="n8n-nav" id="n8nNav">
    <div class="n8n-nav-top">
      <div class="n8n-brand" onclick="setSection('overview')">
        <svg width="26" height="20" viewBox="0 0 36 24" fill="none" aria-label="Conrad Automation">
          <circle cx="5" cy="12" r="3.5" fill="#ff6d5a"/><circle cx="15" cy="12" r="3.5" fill="#ff6d5a"/>
          <circle cx="29" cy="6" r="3.5" fill="#ff6d5a"/><circle cx="29" cy="18" r="3.5" fill="#ff6d5a"/>
          <path d="M8.5 12H11.5M18.5 12H22L25.5 7M18.5 12H22L25.5 17" stroke="#ff6d5a" stroke-width="2.4" stroke-linecap="round"/>
        </svg>
        <span class="brand-wordmark" style="font-size:13px;letter-spacing:-0.01em;white-space:nowrap;">Conrad Automation</span>
      </div>
      <button class="n8n-nav-btn-icon" onclick="toggleNav()" title="Sidebar einklappen">«</button>
    </div>

    <div class="n8n-create-wrap">
      <button class="n8n-create-btn" onclick="createBlankWorkflow()"><span>+</span><span class="nav-txt">Workflow erstellen</span></button>
    </div>

    <div class="n8n-nav-list">
      <button class="n8n-nav-item" id="nav-overview" onclick="setSection('overview')">
        <span class="n8n-nav-left"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M3 10h18"/><path d="M9 10v10"/></svg><span class="nav-txt">Übersicht</span></span>
        <span class="nav-badge" id="navWfCount">–</span>
      </button>
      <button class="n8n-nav-item active" id="nav-editor" onclick="setSection('editor')">
        <span class="n8n-nav-left"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="6" cy="12" r="3"/><circle cx="18" cy="6" r="3"/><circle cx="18" cy="18" r="3"/><path d="M9 11l6-4m-6 6l6 4"/></svg><span class="nav-txt">Persönlich</span></span>
      </button>
      <button class="n8n-nav-item" id="nav-templates" onclick="setSection('templates')">
        <span class="n8n-nav-left"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M4 5h16v6H4z"/><path d="M4 15h7v4H4z"/><path d="M15 15h5v4h-5z"/></svg><span class="nav-txt">Vorlagen</span></span>
      </button>
      <button class="n8n-nav-item" id="nav-credentials" onclick="setSection('credentials')">
        <span class="n8n-nav-left"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="5" y="11" width="14" height="10" rx="2"/><path d="M8 11V7a4 4 0 0 1 8 0v4"/></svg><span class="nav-txt">Anmeldedaten</span></span>
      </button>
      <button class="n8n-nav-item" id="nav-data-tables" onclick="setSection('data-tables')">
        <span class="n8n-nav-left"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><ellipse cx="12" cy="6" rx="8" ry="3"/><path d="M4 6v6a8 3 0 0 0 16 0v-6"/><path d="M4 12v6a8 3 0 0 0 16 0v-6"/></svg><span class="nav-txt">Datentabellen</span></span>
      </button>
      <button class="n8n-nav-item" id="nav-variables" onclick="setSection('variables')">
        <span class="n8n-nav-left"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M7 4l-3 8l3 8"/><path d="M17 4l3 8l-3 8"/><path d="M10 9l4 6"/></svg><span class="nav-txt">Variablen</span></span>
      </button>
      <button class="n8n-nav-item" id="nav-insights" onclick="setSection('insights')">
        <span class="n8n-nav-left"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M4 19h16"/><path d="M7 15v-4"/><path d="M12 15v-8"/><path d="M17 15v-11"/></svg><span class="nav-txt">Statistiken</span></span>
      </button>
    </div>

    <div class="n8n-nav-foot">
      <button class="n8n-nav-item" onclick="toggleN8nTheme()">
        <span class="n8n-nav-left"><svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><circle cx="12" cy="12" r="4"/><path d="M12 3v2m0 14v2M3 12h2m14 0h2"/></svg><span class="nav-txt" id="themeLbl">Dunkle Ansicht</span></span>
      </button>
      <a class="n8n-nav-item" href="/twenty">
        <span class="n8n-nav-left"><svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M3 21h18"/><path d="M5 21v-14l8 -4v18"/></svg><span class="nav-txt">Conrad CRM ↗</span></span>
      </a>
      <a class="n8n-nav-item" href="/showcase">
        <span class="n8n-nav-left"><svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M13 2L3 14h9l-1 8l10-12h-9l1-8z"/></svg><span class="nav-txt">5.000 € → &lt;20 € Vergleich</span></span>
      </a>
      <div class="n8n-user-row">
        <span class="n8n-avatar">MC</span>
        <div class="nav-txt" style="min-width:0;">
          <div style="font-size:12px;font-weight:600;color:#fff;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">Demo-Admin</div>
          <div style="font-size:10.5px;color:#9ca3af;">Conrad KI Engine · Aktiv</div>
        </div>
      </div>
    </div>
  </aside>

  <!-- Main Workspace -->
  <main class="n8n-main">
    <header class="n8n-hdr" id="wfHeader">
      <div class="n8n-hdr-left">
        <button class="n8n-btn" style="padding:0 7px;flex-shrink:0;" onclick="toggleNav()" title="Menü">☰</button>
        <span id="sectionTitleHdr" style="display:none;font-weight:600;font-size:14px;color:var(--n8n-text-1);white-space:nowrap;"></span>
        <div class="n8n-hide-sm" id="wfScopeWrap" style="position:relative;flex-shrink:0;">
          <button class="n8n-crumb-btn" onclick="toggleN8nPop('wfPickPop')">Persönlich ▾</button>
          <div class="n8n-pop" id="wfPickPop" style="top:34px;left:0;width:330px;max-height:430px;overflow-y:auto;"></div>
        </div>
        <span class="n8n-hide-sm" id="wfSlashSep" style="color:var(--n8n-text-3);flex-shrink:0;">/</span>
        <div class="n8n-title-wrap" id="wfTitleWrap">
          <input class="n8n-title-inp" id="wfTitleInp" value="Workflow" onchange="renameWorkflow(this.value)">
          <div class="n8n-hide-sm" id="wfTagsWrap" style="display:flex;align-items:center;gap:4px;min-width:0;overflow:hidden;flex-shrink:1;"></div>
          <button class="n8n-tag-pill n8n-hide-sm" id="wfAddTagBtn" onclick="promptAddTag()">+ Tag</button>
        </div>
      </div>

      <div class="n8n-seg" id="centerTabs">
        <button class="n8n-seg-btn active" data-tab="editor" onclick="setEditorTab('editor',this)">Editor</button>
        <button class="n8n-seg-btn" data-tab="executions" onclick="setEditorTab('executions',this)">Ausführungen</button>
        <button class="n8n-seg-btn" data-tab="evaluations" onclick="setEditorTab('evaluations',this)">Auswertungen</button>
      </div>

      <div class="n8n-hdr-right">
        <input type="hidden" id="modelSelect" value="conrad-n8n-agent:0.6b">
        <span class="n8n-btn mono n8n-hide-sm" style="font-size:11px;padding:0 9px;cursor:default;background:var(--n8n-green-soft);border-color:var(--n8n-green);color:var(--n8n-green);font-weight:700;" title="Spezialisierte Conrad KI Engine für Workflows &amp; CRM-Automatisierung">⚡ Conrad KI Engine</span>
        <div id="wfActionsRight" style="display:flex;align-items:center;gap:8px;">
          <div class="n8n-active-wrap" onclick="toggleActiveWf()">
            <span class="n8n-hide-sm" id="activeLbl">Inaktiv</span>
            <button type="button" class="n8n-switch" id="activeSwitch"></button>
          </div>
          <button class="n8n-btn n8n-hide-sm" onclick="openShareModal()">Teilen</button>
          <button class="n8n-btn n8n-btn-coral" id="saveBtn" onclick="saveCurrentWorkflow()">Speichern</button>
          <div style="position:relative;">
            <button class="n8n-btn" style="padding:0 8px;" onclick="toggleN8nPop('wfMenuPop')" title="Weitere Optionen">⋯</button>
            <div class="n8n-pop" id="wfMenuPop" style="top:34px;right:0;">
              <button class="n8n-pop-item" onclick="openWfSettingsModal();closeN8nPops()"><span>⚙ Workflow-Einstellungen</span></button>
              <button class="n8n-pop-item" onclick="duplicateCurrentWorkflow();closeN8nPops()"><span>⧉ Workflow duplizieren</span></button>
              <button class="n8n-pop-item" onclick="openWfImportModal();closeN8nPops()"><span>📥 JSON importieren</span></button>
              <button class="n8n-pop-item" onclick="downloadWorkflowJson();closeN8nPops()"><span>📤 JSON herunterladen</span></button>
              <button class="n8n-pop-item" onclick="syncSqlite();closeN8nPops()"><span>↻ Demo-Workflows zurücksetzen</span></button>
              <div style="border-top:1px solid var(--n8n-border);margin:4px 0;"></div>
              <button class="n8n-pop-item" style="color:var(--n8n-red);" onclick="deleteCurrentWorkflow();closeN8nPops()"><span>🗑 Workflow löschen</span></button>
            </div>
          </div>
        </div>
      </div>
    </header>

    <div id="mainContent" style="flex:1;min-height:0;display:flex;flex-direction:column;position:relative;overflow:hidden;">
      <!-- Historical Execution Inspection Banner -->
      <div class="n8n-exec-banner" id="execInspectBanner">
        <span id="execInspectLabel"></span>
        <div style="display:flex;gap:6px;">
          <button class="n8n-btn" style="height:24px;font-size:11.5px;" onclick="retryViewedExecution()">🔄 Erneut ausführen</button>
          <button class="n8n-btn n8n-btn-coral" style="height:24px;font-size:11.5px;" onclick="exitExecutionView()">Zurück zum Editor</button>
        </div>
      </div>

      <!-- Canvas Stage -->
      <div class="n8n-stage" id="canvasStage">
        <div class="n8n-canvas" id="n8nCanvas">
          <div class="n8n-viewport" id="n8nViewport">
            <svg class="n8n-wires" id="wireSvg"></svg>
            <div id="nodesLayer"></div>
            <div id="edgeToolbarLayer"></div>
          </div>
          <div class="n8n-marquee" id="marqueeBox"></div>
          <button class="n8n-float-plus" onclick="openCreatorDrawer(null)" title="Node hinzufügen (Tab)">+</button>
          <div class="n8n-controls-bl">
            <button class="n8n-sq-btn" onclick="fitToView()" title="An Ansicht anpassen">⊡</button>
            <button class="n8n-sq-btn" onclick="zoomBy(0.12)" title="Vergrößern">+</button>
            <button class="n8n-sq-btn" onclick="zoomBy(-0.12)" title="Verkleinern">−</button>
            <button class="n8n-sq-btn" id="selectModeBtn" onclick="toggleSelectMode()" title="Rahmen-Auswahlmodus (V)">⬚</button>
            <button class="n8n-sq-btn" onclick="addStickyNoteAtCenter()" title="Haftnotiz hinzufügen (Shift+S)">📝</button>
            <button class="n8n-sq-btn" onclick="tidyUpLayout()" title="Workflow aufräumen">🪄</button>
            <button class="n8n-sq-btn" id="undoBtn" onclick="undoCanvas()" title="Rückgängig (⌘Z)">↶</button>
            <button class="n8n-sq-btn" id="redoBtn" onclick="redoCanvas()" title="Wiederholen (⌘⇧Z)">↷</button>
            <button class="n8n-sq-btn active" id="minimapToggleBtn" onclick="toggleMinimap()" title="Minimap ein/aus">🗺</button>
          </div>
          <div class="n8n-minimap" id="n8nMinimap" title="Klicken zum Navigieren">
            <canvas id="minimapCanvas" width="170" height="108"></canvas>
          </div>
          <div class="n8n-test-bar">
            <button class="n8n-btn n8n-btn-coral" id="testWfBtn" onclick="runCurrentWorkflow()" style="height:34px;padding:0 16px;font-size:13px;">🧪 Workflow testen</button>
            <button class="n8n-btn" onclick="togglePayloadPop()" style="height:34px;">💬 Testdaten</button>
            <button class="n8n-btn" onclick="toggleBottomLogs()" id="logsToggleBtn" style="height:34px;">Protokoll</button>
          </div>
        </div>

        <!-- Collapsible Bottom Execution Logs Panel (n8n 2.x Split Style) -->
        <div class="n8n-bottom-logs" id="bottomLogs">
          <div class="n8n-logs-resizer" id="logsResizer"></div>
          <div style="height:36px;min-height:36px;padding:0 14px;border-bottom:1px solid var(--n8n-border);display:flex;align-items:center;justify-content:space-between;background:var(--n8n-subtle-bg);">
            <div style="display:flex;align-items:center;gap:10px;min-width:0;">
              <strong style="font-size:12px;" id="logsHdrTxt">Ausführungsprotokoll</strong>
              <span id="logsStatusPill" class="n8n-tag-pill mono" style="display:none;"></span>
            </div>
            <div style="display:flex;align-items:center;gap:6px;">
              <div class="n8n-seg" id="logsFmtSeg">
                <button class="n8n-seg-btn active" onclick="setLogsFmt('table',this)">Tabelle</button>
                <button class="n8n-seg-btn" onclick="setLogsFmt('json',this)">JSON</button>
                <button class="n8n-seg-btn" onclick="setLogsFmt('schema',this)">Schema</button>
              </div>
              <button class="n8n-btn" style="height:24px;" onclick="toggleBottomLogs(false)">Schließen ×</button>
            </div>
          </div>
          <div style="flex:1;min-height:0;display:grid;grid-template-columns:280px 1fr 1fr;" id="logsBodyGrid">
            <div id="logsStepList" style="border-right:1px solid var(--n8n-border);overflow-y:auto;padding:6px;"></div>
            <div style="border-right:1px solid var(--n8n-border);display:flex;flex-direction:column;min-height:0;overflow:hidden;">
              <div style="padding:5px 10px;border-bottom:1px solid var(--n8n-border);font-size:11px;font-weight:700;color:var(--n8n-text-2);background:var(--n8n-subtle-bg);">EINGANG</div>
              <div id="logsInBody" style="flex:1;overflow:auto;padding:8px 10px;"></div>
            </div>
            <div style="display:flex;flex-direction:column;min-height:0;overflow:hidden;">
              <div style="padding:5px 10px;border-bottom:1px solid var(--n8n-border);font-size:11px;font-weight:700;color:var(--n8n-text-2);background:var(--n8n-subtle-bg);">AUSGANG</div>
              <div id="logsOutBody" style="flex:1;overflow:auto;padding:8px 10px;"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Non-canvas View Container -->
      <div class="n8n-page" id="altPage" style="display:none;"></div>
    </div>
  </main>
</div>

<!-- Canvas / Node Context Menu -->
<div class="n8n-ctx" id="ctxMenu"></div>

<!-- Node Creator Slide-Over ("Was passiert als Nächstes?") -->
<aside class="n8n-creator" id="creatorDrawer">
  <div style="padding:14px 16px;border-bottom:1px solid var(--n8n-border);display:flex;align-items:center;justify-content:space-between;">
    <strong style="font-size:14px;" id="creatorTitle">Was passiert als Nächstes? <span class="tag-pill mono" id="creatorCountBadge" style="font-size:11px;margin-left:4px;">0</span></strong>
    <button class="n8n-btn" style="height:26px;" onclick="closeCreatorDrawer()">×</button>
  </div>
  <div style="padding:10px 14px;border-bottom:1px solid var(--n8n-border);display:flex;flex-direction:column;gap:8px;">
    <input class="n8n-input" id="creatorSearch" placeholder="Nodes durchsuchen (Webhook, KI-Agent, Code, Datentabelle...)" oninput="renderCreatorList()">
    <div id="creatorCatBar" style="display:flex;gap:5px;flex-wrap:wrap;"></div>
  </div>
  <div id="creatorList" style="flex:1;overflow-y:auto;padding:10px 12px;"></div>
</aside>

<!-- 3-Column NDV Modal -->
<div class="n8n-ndv-backdrop" id="ndvBackdrop" onclick="if(event.target===this)closeNdv()">
  <div class="n8n-ndv">
    <div class="n8n-ndv-top">
      <button class="n8n-btn" id="ndvBackBtn" onclick="closeNdv()">← Zurück zum Canvas</button>
      <div style="display:flex;align-items:center;gap:8px;font-size:12px;color:var(--n8n-text-2);">
        <span class="mono" id="ndvNodeTypeLbl"></span>
      </div>
      <div style="display:flex;align-items:center;gap:6px;">
        <button class="n8n-btn" id="ndvPinBtn" onclick="pinCurrentNodeOutput()">📌 Daten anheften</button>
        <button class="n8n-btn" onclick="closeNdv()">Fertig</button>
      </div>
    </div>
    <div class="n8n-ndv-grid">
      <!-- Column 1: EINGANG -->
      <div class="n8n-ndv-col">
        <div class="n8n-ndv-col-hdr">
          <div style="display:flex;align-items:center;gap:6px;">
            <strong style="font-size:12px;letter-spacing:0.04em;">EINGANG</strong>
            <select id="ndvInNodeSel" class="n8n-select" style="width:auto;max-width:155px;padding:3px 6px;font-size:11px;display:none;" onchange="st.ndvInSourceNode=this.value;renderNdvInput()"></select>
          </div>
          <div class="n8n-seg" id="ndvInFmtSeg">
            <button class="n8n-seg-btn" data-fmt="table" onclick="setNdvInFmt('table',this)">Tabelle</button>
            <button class="n8n-seg-btn active" data-fmt="json" onclick="setNdvInFmt('json',this)">JSON</button>
            <button class="n8n-seg-btn" data-fmt="schema" onclick="setNdvInFmt('schema',this)">Schema</button>
          </div>
        </div>
        <div class="n8n-ndv-body" id="ndvInBody"></div>
      </div>

      <!-- Column 2: Parameter / Einstellungen -->
      <div class="n8n-ndv-col">
        <div class="n8n-ndv-col-hdr">
          <div style="display:flex;align-items:center;gap:8px;min-width:0;flex:1;">
            <span id="ndvNodeIcon"></span>
            <input id="ndvNodeNameInp" class="n8n-input" style="font-weight:600;background:transparent;border-color:transparent;" onchange="renameNdvNode(this.value)">
          </div>
          <button class="n8n-btn n8n-btn-coral" id="ndvTestStepBtn" onclick="executeNdvStep()">▶ Schritt testen</button>
        </div>
        <div style="padding:0 14px;border-bottom:1px solid var(--n8n-border);display:flex;gap:14px;background:var(--n8n-subtle-bg);">
          <button class="n8n-seg-btn active" style="border-radius:0;padding:8px 2px;background:transparent;border-bottom:2px solid var(--n8n-coral);" id="ndvTabParamsBtn" onclick="setNdvCenterTab('params')">Parameter</button>
          <button class="n8n-seg-btn" style="border-radius:0;padding:8px 2px;background:transparent;" id="ndvTabSettingsBtn" onclick="setNdvCenterTab('settings')">Einstellungen</button>
        </div>
        <div class="n8n-ndv-body" id="ndvCenterBody"></div>
      </div>

      <!-- Column 3: AUSGANG -->
      <div class="n8n-ndv-col">
        <div class="n8n-ndv-col-hdr">
          <div style="display:flex;align-items:center;gap:7px;">
            <strong style="font-size:12px;letter-spacing:0.04em;">AUSGANG</strong>
            <span id="ndvOutBadge" class="tag-pill mono" style="font-size:11px;color:var(--n8n-green);"></span>
          </div>
          <div class="n8n-seg" id="ndvOutFmtSeg">
            <button class="n8n-seg-btn" data-fmt="table" onclick="setNdvOutFmt('table',this)">Tabelle</button>
            <button class="n8n-seg-btn active" data-fmt="json" onclick="setNdvOutFmt('json',this)">JSON</button>
            <button class="n8n-seg-btn" data-fmt="schema" onclick="setNdvOutFmt('schema',this)">Schema</button>
          </div>
        </div>
        <div class="n8n-ndv-body" id="ndvOutBody"></div>
      </div>
    </div>
  </div>
</div>

<!-- Quick Credential Create Modal inside NDV -->
<div class="n8n-ndv-backdrop" id="ndvCredModal" style="z-index:110;" onclick="if(event.target===this)this.classList.remove('open')">
  <div class="n8n-ndv" style="max-width:520px;height:auto;">
    <div class="n8n-ndv-top">
      <strong>Neue Anmeldedaten anlegen</strong>
      <button class="n8n-btn" onclick="document.getElementById('ndvCredModal').classList.remove('open')">Schließen</button>
    </div>
    <div id="ndvCredModalBody" style="padding:16px;display:flex;flex-direction:column;gap:10px;"></div>
  </div>
</div>

<!-- Workflow Settings Modal -->
<div class="n8n-ndv-backdrop" id="wfSettingsModal" onclick="if(event.target===this)this.classList.remove('open')">
  <div class="n8n-ndv" style="max-width:540px;height:auto;">
    <div class="n8n-ndv-top">
      <strong>Workflow-Einstellungen</strong>
      <button class="n8n-btn" onclick="document.getElementById('wfSettingsModal').classList.remove('open')">Schließen</button>
    </div>
    <div id="wfSettingsBody" style="padding:16px;display:flex;flex-direction:column;gap:12px;"></div>
  </div>
</div>

<!-- Test Payload Modal -->
<div class="n8n-ndv-backdrop" id="payloadModal" onclick="if(event.target===this)this.classList.remove('open')">
  <div class="n8n-ndv" style="max-width:520px;height:auto;">
    <div class="n8n-ndv-top">
      <strong>Webhook- / Chat-Trigger Testdaten</strong>
      <button class="n8n-btn" onclick="document.getElementById('payloadModal').classList.remove('open')">Schließen</button>
    </div>
    <div style="padding:16px;display:flex;flex-direction:column;gap:10px;">
      <textarea id="globalPayloadInput" class="n8n-code-ed mono" rows="7">{
  "message": "Suche Firma Elektro Weber in Conrad CRM und erstelle einen Entwurf",
  "workspace": "conrad-ki"
}</textarea>
      <div style="display:flex;justify-content:flex-end;gap:8px;">
        <button class="n8n-btn n8n-btn-coral" onclick="document.getElementById('payloadModal').classList.remove('open');runCurrentWorkflow()">▶ Jetzt Workflow testen</button>
      </div>
    </div>
  </div>
</div>

<!-- Workflow JSON Import Modal -->
<div class="n8n-ndv-backdrop" id="wfImportModal" onclick="if(event.target===this)this.classList.remove('open')">
  <div class="n8n-ndv" style="max-width:560px;height:auto;">
    <div class="n8n-ndv-top">
      <strong>Workflow-JSON importieren</strong>
      <button class="n8n-btn" onclick="document.getElementById('wfImportModal').classList.remove('open')">Schließen</button>
    </div>
    <div style="padding:16px;display:flex;flex-direction:column;gap:10px;">
      <input type="file" id="wfImportFile" accept=".json,application/json" onchange="onWfImportFile(event)" style="font-size:12px;">
      <textarea id="wfImportText" class="n8n-code-ed mono" rows="8" placeholder='{"name":"Mein Workflow","nodes":[...],"connections":{...}}'></textarea>
      <div style="display:flex;justify-content:flex-end;gap:8px;">
        <button class="n8n-btn" onclick="document.getElementById('wfImportModal').classList.remove('open')">Abbrechen</button>
        <button class="n8n-btn n8n-btn-coral" onclick="submitWfImport()">Workflow importieren</button>
      </div>
    </div>
  </div>
</div>

<!-- Share / Webhook Endpoint Modal -->
<div class="n8n-ndv-backdrop" id="shareModal" onclick="if(event.target===this)this.classList.remove('open')">
  <div class="n8n-ndv" style="max-width:520px;height:auto;">
    <div class="n8n-ndv-top">
      <strong>Workflow teilen &amp; Webhook-Endpunkt</strong>
      <button class="n8n-btn" onclick="document.getElementById('shareModal').classList.remove('open')">Schließen</button>
    </div>
    <div id="shareModalBody" style="padding:16px;display:flex;flex-direction:column;gap:10px;"></div>
  </div>
</div>

<div class="n8n-toast" id="n8nToast"></div>

<script>
const NODE_META = {
  'n8n-nodes-base.webhook': { label: 'Webhook', cat: 'Trigger', color: '#ff6d5a', trig: true, svg: '<svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#ff6d5a" stroke-width="2" stroke-linecap="round"><path d="M4.876 13.61a4 4 0 1 0 6.124 3.39h6"/><path d="M15.066 20.502a4 4 0 1 0 1.934 -7.502c-.706 0 -1.424 .179 -2 .5l-3 -5.5"/><path d="M16 8a4 4 0 1 0 -8 0c0 1.506 .77 2.818 2 3.5l-3 5.5"/></svg>' },
  'n8n-nodes-base.manualTrigger': { label: 'Manueller Trigger', cat: 'Trigger', color: '#ff6d5a', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="#ff6d5a"><polygon points="6,4 20,12 6,20"/></svg>' },
  'n8n-nodes-base.scheduleTrigger': { label: 'Zeitplan-Trigger', cat: 'Trigger', color: '#0d9488', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#0d9488" stroke-width="2"><circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 3"/></svg>' },
  '@n8n/n8n-nodes-langchain.chatTrigger': { label: 'Chat-Trigger', cat: 'Trigger', color: '#7c3aed', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#7c3aed" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>' },
  'n8n-nodes-base.gmailTrigger': { label: 'Gmail-Trigger', cat: 'Trigger', color: '#ea4335', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#ea4335" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.telegramTrigger': { label: 'Telegram-Trigger', cat: 'Trigger', color: '#26a5e4', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#26a5e4" stroke-width="2"><path d="M15 10l-4 4l6 6l4-16l-18 7l4 2l2 6l3-4"/></svg>' },
  'n8n-nodes-base.microsoftOutlookTrigger': { label: 'Outlook-Trigger', cat: 'Trigger', color: '#0078d4', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#0078d4" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.errorTrigger': { label: 'Fehler-Trigger', cat: 'Trigger', color: '#dc2626', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#dc2626" stroke-width="2"><path d="M12 9v4m0 4v.01"/><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/></svg>' },
  '@n8n/n8n-nodes-langchain.agent': { label: 'KI-Agent', cat: 'KI & LangChain', color: '#1f2329', rootAgent: true, svg: '<svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="7" width="18" height="12" rx="3"/><circle cx="9" cy="13" r="1.5" fill="currentColor"/><circle cx="15" cy="13" r="1.5" fill="currentColor"/><path d="M12 7V4m-3 0h6"/></svg>' },
  '@n8n/n8n-nodes-langchain.chainLlm': { label: 'Basis-LLM-Kette', cat: 'KI & LangChain', color: '#7c3aed', rootAgent: true, svg: '<svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="#7c3aed" stroke-width="2"><path d="M12 3l2.5 5.5L20 11l-5.5 2.5L12 19l-2.5-5.5L4 11l5.5-2.5z"/></svg>' },
  '@n8n/n8n-nodes-langchain.lmChatOllama': { label: 'Ollama Chat-Modell', cat: 'KI & LangChain', color: '#18181b', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="8"/><circle cx="9.5" cy="10.5" r="1" fill="currentColor"/><circle cx="14.5" cy="10.5" r="1" fill="currentColor"/><path d="M9.5 14.5c1 .8 4 .8 5 0"/></svg>' },
  '@n8n/n8n-nodes-langchain.lmChatOpenAi': { label: 'OpenAI Chat-Modell', cat: 'KI & LangChain', color: '#10a37f', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#10a37f" stroke-width="2"><circle cx="12" cy="12" r="8"/><path d="M12 4v16M5 8l14 8M19 8L5 16"/></svg>' },
  '@n8n/n8n-nodes-langchain.memoryBufferWindow': { label: 'Fenster-Pufferspeicher', cat: 'KI & LangChain', color: '#7c3aed', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#7c3aed" stroke-width="2"><rect x="6" y="6" width="12" height="12" rx="2"/><path d="M9 2v4m6-4v4m-6 12v4m6-4v4M2 9h4m-4 6h4m12-6h4m-4 6h4"/></svg>' },
  '@n8n/n8n-nodes-langchain.vectorStoreQdrant': { label: 'Qdrant Vektorspeicher', cat: 'KI & LangChain', color: '#dc2626', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#dc2626" stroke-width="2"><polygon points="12,2 21,7 21,17 12,22 3,17 3,7"/><circle cx="12" cy="12" r="3"/></svg>' },
  '@n8n/n8n-nodes-langchain.embeddingsOllama': { label: 'Ollama-Embeddings', cat: 'KI & LangChain', color: '#4f46e5', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#4f46e5" stroke-width="2"><path d="M4 7h16M4 12h16M4 17h10"/></svg>' },
  'n8n-nodes-base.httpRequest': { label: 'HTTP-Anfrage', cat: 'Kern', color: '#4078f2', svg: '<svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#4078f2" stroke-width="2"><circle cx="12" cy="12" r="9"/><path d="M3.6 9h16.8M3.6 15h16.8M11.5 3a17 17 0 0 0 0 18M12.5 3a17 17 0 0 1 0 18"/></svg>' },
  'n8n-nodes-base.code': { label: 'Code', cat: 'Kern', color: '#f97316', svg: '<svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#f97316" stroke-width="2.1" stroke-linecap="round"><path d="M8 6c-2 0-2 2-2 4s-2 2-2 2s2 0 2 2s0 4 2 4"/><path d="M16 6c2 0 2 2 2 4s2 2 2 2s-2 0-2 2s0 4-2 4"/></svg>' },
  'n8n-nodes-base.if': { label: 'Wenn (If)', cat: 'Datenfluss', color: '#16a34a', svg: '<svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#16a34a" stroke-width="2.2" stroke-linecap="round"><path d="M4 12h5l4-5h7"/><path d="M9 12l4 5h7"/></svg>' },
  'n8n-nodes-base.switch': { label: 'Weiche (Switch)', cat: 'Datenfluss', color: '#6366f1', svg: '<svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="#6366f1" stroke-width="2" stroke-linecap="round"><path d="M3 12h6M13 6h8M13 12h8M13 18h8M9 12l4-6M9 12l4 6"/></svg>' },
  'n8n-nodes-base.set': { label: 'Felder bearbeiten (Set)', cat: 'Datenverarbeitung', color: '#2563eb', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#2563eb" stroke-width="2"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1l1-4z"/></svg>' },
  'n8n-nodes-base.respondToWebhook': { label: 'Auf Webhook antworten', cat: 'Kern', color: '#d97706', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#d97706" stroke-width="2"><path d="M9 14l-4-4l4-4"/><path d="M5 10h11a4 4 0 1 1 0 8h-1"/></svg>' },
  'n8n-nodes-base.gmail': { label: 'Gmail', cat: 'Integrationen', color: '#ea4335', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#ea4335" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.telegram': { label: 'Telegram', cat: 'Integrationen', color: '#26a5e4', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#26a5e4" stroke-width="2"><path d="M15 10l-4 4l6 6l4-16l-18 7l4 2l2 6l3-4"/></svg>' },
  'n8n-nodes-base.microsoftOutlook': { label: 'Microsoft Outlook', cat: 'Integrationen', color: '#0078d4', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#0078d4" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.postgres': { label: 'PostgreSQL', cat: 'Integrationen', color: '#336791', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#336791" stroke-width="2"><ellipse cx="12" cy="6" rx="8" ry="3"/><path d="M4 6v6a8 3 0 0 0 16 0v-6"/><path d="M4 12v6a8 3 0 0 0 16 0v-6"/></svg>' },
  'n8n-nodes-base.googleSheets': { label: 'Google Sheets', cat: 'Integrationen', color: '#0f9d58', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#0f9d58" stroke-width="2"><rect x="4" y="3" width="16" height="18" rx="2"/><path d="M4 9h16M4 15h16M10 9v12"/></svg>' },
  'n8n-nodes-base.emailSend': { label: 'E-Mail Entwurf (SMTP)', cat: 'Integrationen', color: '#0d9488', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#0d9488" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.twentyCrm': { label: 'Conrad CRM', cat: 'Integrationen', color: '#5b60e8', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#5b60e8" stroke-width="2"><path d="M3 21h18"/><path d="M5 21V7l7-4v18"/><path d="M19 21V11l-7-4"/></svg>' },
  'n8n-nodes-base.dataTable': { label: 'Datentabelle', cat: 'Datenverarbeitung', color: '#0d9488', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#0d9488" stroke-width="2"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M3 10h18M9 10v10"/></svg>' },
  'n8n-nodes-base.noOp': { label: 'Keine Aktion', cat: 'Datenfluss', color: '#64748b', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#64748b" stroke-width="2"><path d="M5 12h14m-5-5l5 5l-5 5"/></svg>' },
  'n8n-nodes-base.stickyNote': { label: 'Haftnotiz', cat: 'Kern', color: '#ca8a04', svg: '<svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="2"><path d="M13 20l7-7V5a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2z"/></svg>' }
};

const STICKY_THEMES = [
  { id: 1, bg: '#fef9c3', border: '#fde047', fg: '#422006' },
  { id: 2, bg: '#ffedd5', border: '#fdba74', fg: '#431407' },
  { id: 3, bg: '#fee2e2', border: '#fca5a5', fg: '#450a0a' },
  { id: 4, bg: '#dcfce7', border: '#86efac', fg: '#052e16' },
  { id: 5, bg: '#dbeafe', border: '#93c5fd', fg: '#172554' },
  { id: 6, bg: '#f3e8ff', border: '#d8b4fe', fg: '#3b0764' },
  { id: 7, bg: '#f3f4f6', border: '#d1d5db', fg: '#1f2937' }
];

const st = {
  section: 'editor',
  editorTab: 'editor',
  overviewTab: 'workflows',
  overviewSort: 'updated',
  overviewTag: 'Alle',
  execFilter: 'all',
  workflows: [],
  currentWf: null,
  nodeCatalog: [],
  nodeSchemaByName: {},
  credCatalog: [],
  savedCredentials: [],
  variables: [],
  dataTables: [],
  selectedDataTableId: null,
  dataTableRows: [],
  editingDtRowId: null,
  creatorCat: 'Alle',
  executions: [],
  selectedExecIdx: 0,
  viewingExecutionId: null,
  nodeStatusMap: {},
  lastExec: null,
  selectedLogIdx: 0,
  logsFmt: 'table',
  ndvNodeName: null,
  ndvInSourceNode: null,
  ndvCenterTab: 'params',
  ndvInFmt: 'json',
  ndvOutFmt: 'json',
  ndvOutFilter: '',
  creatorMode: 'add', // 'add' | 'insertEdge' | 'agentSub'
  creatorSourceNode: null,
  creatorSourceBranch: 0,
  creatorInsertEdge: null,
  creatorAgentSlot: null,
  zoom: 0.92,
  panX: 60,
  panY: 70,
  selectMode: false,
  showMinimap: true,
  dirty: false,
  selectedNodes: new Set(),
  editingStickyName: null,
  clipboard: null,
  undoStack: [],
  redoStack: []
};

function esc(s) { return String(s ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c])); }
function toast(msg) { const el = document.getElementById('n8nToast'); el.textContent = msg; el.classList.add('show'); clearTimeout(window._tTmr); window._tTmr = setTimeout(() => el.classList.remove('show'), 2200); }
const CAT_COLORS = { Trigger: '#ff6d5a', 'KI & LangChain': '#7c3aed', Kern: '#2563eb', Datenverarbeitung: '#0d9488', Datenfluss: '#6366f1', Integrationen: '#059669', 'Dateien & Krypto': '#d97706' };

function makeNodeSvg(color, glyph) {
  return `<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="${color}" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="5" fill="${color}14"/><text x="12" y="15.5" text-anchor="middle" font-size="8.5" font-weight="700" fill="${color}" stroke="none">${esc(glyph)}</text></svg>`;
}
function metaFor(tp) {
  if (NODE_META[tp]) return NODE_META[tp];
  const sc = st.nodeSchemaByName[tp];
  if (sc) {
    const cat = sc.category || 'Kern';
    const clr = CAT_COLORS[cat] || '#4078f2';
    const trig = Array.isArray(sc.group) && sc.group.includes('trigger');
    const sub = Array.isArray(sc.inputs) && sc.inputs.length === 0 && !trig && sc.name.includes('langchain');
    const rootAgent = sc.name.includes('agent') || sc.name.includes('chainLlm');
    const glyph = (sc.displayName || sc.name || 'N').replace(/[^A-Za-z0-9ÄÖÜäöü]/g, '').slice(0, 2).toUpperCase() || 'CA';
    return { label: sc.displayName || tp, cat, color: clr, trig, subnode: sub, rootAgent, svg: makeNodeSvg(clr, glyph), description: sc.description || '' };
  }
  return { label: (tp || 'Node').split('.').pop(), cat: 'Kern', color: '#64748b', svg: NODE_META['n8n-nodes-base.noOp'].svg };
}
function isTrigger(tp) { const t = String(tp || ''); return !!metaFor(t).trig || (t.toLowerCase().includes('trigger') && !t.includes('respond')); }
function isSubnode(tp) { return !!metaFor(tp).subnode; }
function isAgentRoot(tp) { return !!metaFor(tp).rootAgent; }
function nodeCountOf(w) { return typeof w?.nodeCount === 'number' ? w.nodeCount : (Array.isArray(w?.nodes) ? w.nodes.length : 0); }
function triggerCountOf(w) { return typeof w?.triggerCount === 'number' ? w.triggerCount : (Array.isArray(w?.nodes) ? w.nodes.filter(n => isTrigger(n.type)).length : 0); }
function wfWebhookPath(w) {
  if (w?.webhookPath) return w.webhookPath;
  const wn = (w?.nodes || []).find(n => (n.type || '').toLowerCase().includes('webhook') || (n.type || '').toLowerCase().includes('formtrigger'));
  return wn?.parameters?.path || '';
}

function pushHistory() {
  if (!st.currentWf) return;
  const snap = JSON.stringify({
    nodes: st.currentWf.nodes || [],
    connections: st.currentWf.connections || {},
    pinData: st.currentWf.pinData || {},
    settings: st.currentWf.settings || {}
  });
  if (st.undoStack[st.undoStack.length - 1] === snap) return;
  st.undoStack.push(snap);
  if (st.undoStack.length > 50) st.undoStack.shift();
  st.redoStack = [];
}
function undoCanvas() {
  if (!st.currentWf || !st.undoStack.length) return;
  const cur = JSON.stringify({ nodes: st.currentWf.nodes || [], connections: st.currentWf.connections || {}, pinData: st.currentWf.pinData || {}, settings: st.currentWf.settings || {} });
  st.redoStack.push(cur);
  const prev = JSON.parse(st.undoStack.pop());
  st.currentWf.nodes = prev.nodes; st.currentWf.connections = prev.connections;
  st.currentWf.pinData = prev.pinData; st.currentWf.settings = prev.settings;
  st.dirty = true; updateHeaderState(); renderCanvas(); toast('Rückgängig gemacht');
}
function redoCanvas() {
  if (!st.currentWf || !st.redoStack.length) return;
  const cur = JSON.stringify({ nodes: st.currentWf.nodes || [], connections: st.currentWf.connections || {}, pinData: st.currentWf.pinData || {}, settings: st.currentWf.settings || {} });
  st.undoStack.push(cur);
  const nxt = JSON.parse(st.redoStack.pop());
  st.currentWf.nodes = nxt.nodes; st.currentWf.connections = nxt.connections;
  st.currentWf.pinData = nxt.pinData; st.currentWf.settings = nxt.settings;
  st.dirty = true; updateHeaderState(); renderCanvas(); toast('Wiederhergestellt');
}

if (localStorage.getItem('n8n_theme') === 'dark') { document.documentElement.classList.add('dark'); }
function toggleN8nTheme() {
  const dark = document.documentElement.classList.toggle('dark');
  localStorage.setItem('n8n_theme', dark ? 'dark' : 'light');
  document.getElementById('themeLbl').textContent = dark ? 'Helle Ansicht' : 'Dunkle Ansicht';
  drawMinimap();
}
function toggleNav() { document.getElementById('n8nNav').classList.toggle('collapsed'); }
function toggleN8nPop(id) {
  const el = document.getElementById(id); const was = el.classList.contains('open');
  closeN8nPops(); if (!was) el.classList.add('open');
}
function closeN8nPops() { document.querySelectorAll('.n8n-pop').forEach(p => p.classList.remove('open')); }
function closeCtxMenu() { const c = document.getElementById('ctxMenu'); if (c) c.classList.remove('open'); }
document.addEventListener('click', e => {
  if (!e.target.closest('#wfPickPop,.n8n-crumb-btn,#wfMenuPop,.n8n-hdr-right')) closeN8nPops();
  if (!e.target.closest('#ctxMenu,.n8n-tb-btn')) closeCtxMenu();
});

async function loadCatalogs() {
  try {
    const [nRes, cRes, crRes] = await Promise.all([
      fetch('/types/nodes.json'),
      fetch('/types/credentials.json'),
      fetch('/rest/credentials')
    ]);
    if (nRes.ok) {
      const nodes = await nRes.json();
      st.nodeCatalog = Array.isArray(nodes) ? nodes : (nodes.data || []);
      st.nodeSchemaByName = Object.fromEntries(st.nodeCatalog.map(x => [x.name, x]));
      const cb = document.getElementById('creatorCountBadge');
      if (cb) cb.textContent = String(st.nodeCatalog.length);
    }
    if (cRes.ok) {
      const creds = await cRes.json();
      st.credCatalog = Array.isArray(creds) ? creds : (creds.data || []);
    }
    if (crRes.ok) {
      const saved = await crRes.json();
      st.savedCredentials = saved.data || [];
    }
  } catch {}
}

async function init() {
  const [r] = await Promise.all([fetch('/api/n8n/workflows'), loadCatalogs()]);
  const d = await r.json();
  st.workflows = d.data || [];
  document.getElementById('navWfCount').textContent = st.workflows.length;
  renderWfPicker();
  if (!st.currentWf) {
    const preferred = st.workflows.find(w => w.id === 'conradKiAgent001') || st.workflows.find(w => w.active) || st.workflows[0];
    if (preferred) await selectWorkflow(preferred.id);
  }
}

function renderWfPicker() {
  document.getElementById('wfPickPop').innerHTML = st.workflows.map(w => `
    <button class="n8n-pop-item" onclick="selectWorkflow('${esc(w.id)}');closeN8nPops()">
      <span style="overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">${w.active ? '🟢 ' : ''}${esc(w.name)}</span>
      <span class="mono" style="font-size:10.5px;color:var(--n8n-text-3);">${nodeCountOf(w)}n</span>
    </button>
  `).join('');
}

async function selectWorkflow(id) {
  const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(id)}`);
  st.currentWf = await r.json();
  st.nodeStatusMap = {};
  st.lastExec = null;
  st.viewingExecutionId = null;
  document.getElementById('execInspectBanner')?.classList.remove('open');
  st.dirty = false;
  st.selectedNodes.clear();
  st.undoStack = [];
  st.redoStack = [];
  st.section = 'editor';
  st.editorTab = 'editor';
  document.querySelectorAll('#centerTabs .n8n-seg-btn').forEach(b => b.classList.toggle('active', b.dataset.tab === 'editor'));
  updateHeaderState();
  setSection('editor');
  fitToView();
}

function updateHeaderState() {
  if (!st.currentWf) return;
  document.getElementById('wfTitleInp').value = st.currentWf.name || '';
  const act = !!st.currentWf.active;
  document.getElementById('activeLbl').textContent = act ? 'Aktiv' : 'Inaktiv';
  document.getElementById('activeSwitch').classList.toggle('on', act);
  const tags = st.currentWf.tags || [];
  const visTags = tags.slice(0, 2);
  const extraTags = tags.slice(2).map(t => typeof t === 'string' ? t : t.name);
  document.getElementById('wfTagsWrap').innerHTML = visTags.map((t, i) => `
    <span class="n8n-tag-pill">${esc(typeof t === 'string' ? t : t.name)} <span onclick="removeTag(${i})" style="margin-left:2px;">×</span></span>
  `).join('') + (extraTags.length ? `<span class="n8n-tag-pill" title="${esc(extraTags.join(', '))}">+${extraTags.length}</span>` : '');
  document.getElementById('saveBtn').textContent = st.dirty ? 'Speichern *' : 'Speichern';
}

function setSection(sec) {
  st.section = sec;
  document.querySelectorAll('.n8n-nav-item').forEach(el => el.classList.remove('active'));
  document.getElementById('nav-' + sec)?.classList.add('active');
  const isWf = sec === 'editor';
  const secLabels = { overview: 'Übersicht', templates: 'Vorlagen', credentials: 'Anmeldedaten', 'data-tables': 'Datentabellen', variables: 'Variablen', insights: 'Statistiken' };
  document.getElementById('wfScopeWrap').style.display = isWf ? '' : 'none';
  document.getElementById('wfSlashSep').style.display = isWf ? '' : 'none';
  document.getElementById('wfTitleWrap').style.display = isWf ? 'flex' : 'none';
  document.getElementById('centerTabs').style.display = isWf ? 'inline-flex' : 'none';
  document.getElementById('wfActionsRight').style.display = isWf ? 'flex' : 'none';
  const secHdr = document.getElementById('sectionTitleHdr');
  secHdr.style.display = isWf ? 'none' : 'inline-block';
  secHdr.textContent = secLabels[sec] || '';
  const stage = document.getElementById('canvasStage');
  const alt = document.getElementById('altPage');
  if (isWf) {
    stage.style.display = st.editorTab === 'editor' ? 'flex' : 'none';
    alt.style.display = st.editorTab === 'editor' ? 'none' : 'block';
    if (st.editorTab !== 'editor') renderAltEditorTab();
    else renderCanvas();
  } else {
    stage.style.display = 'none';
    alt.style.display = 'block';
    renderSectionPage(sec);
  }
}

function setEditorTab(tab, btn) {
  st.editorTab = tab;
  document.querySelectorAll('#centerTabs .n8n-seg-btn').forEach(b => b.classList.remove('active'));
  btn.classList.add('active');
  setSection('editor');
}

function renderOverviewToolbar() {
  const allTags = ['Alle', ...Array.from(new Set(st.workflows.flatMap(w => (w.tags || []).map(t => typeof t === 'string' ? t : t.name))))];
  return `
    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:14px;flex-wrap:wrap;gap:10px;">
      <div>
        <h2 style="margin:0;font-size:20px;">Übersicht · Conrad Automation</h2>
        <div style="color:var(--n8n-text-2);font-size:12.5px;">Workflows, Anmeldedaten &amp; Ausführungen · Lokal &amp; sofort persistent</div>
      </div>
      <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap;">
        <button class="n8n-btn" onclick="openWfImportModal()">📥 JSON importieren</button>
        <button class="n8n-btn n8n-btn-coral" onclick="createBlankWorkflow()">+ Workflow erstellen</button>
      </div>
    </div>
    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:14px;flex-wrap:wrap;gap:10px;border-bottom:1px solid var(--n8n-border);padding-bottom:10px;">
      <div class="n8n-seg">
        <button class="n8n-seg-btn ${st.overviewTab==='workflows'?'active':''}" onclick="st.overviewTab='workflows';renderSectionPage('overview')">Workflows (${st.workflows.length})</button>
        <button class="n8n-seg-btn ${st.overviewTab==='credentials'?'active':''}" onclick="st.overviewTab='credentials';renderSectionPage('overview')">Anmeldedaten</button>
        <button class="n8n-seg-btn ${st.overviewTab==='executions'?'active':''}" onclick="st.overviewTab='executions';renderSectionPage('overview')">Ausführungen</button>
      </div>
      ${st.overviewTab === 'workflows' ? `<div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap;">
        <input class="n8n-input" id="ovSearch" placeholder="Workflow suchen..." style="width:210px;" oninput="filterOverviewList()">
        <select class="n8n-select" style="width:auto;" onchange="st.overviewSort=this.value;filterOverviewList()">
          <option value="updated" ${st.overviewSort==='updated'?'selected':''}>Zuletzt aktualisiert</option>
          <option value="name" ${st.overviewSort==='name'?'selected':''}>Name (A–Z)</option>
          <option value="active" ${st.overviewSort==='active'?'selected':''}>Aktiv zuerst</option>
        </select>
      </div>` : ''}
    </div>
    ${st.overviewTab === 'workflows' && allTags.length > 1 ? `<div style="display:flex;gap:6px;flex-wrap:wrap;margin-bottom:12px;">
      ${allTags.map(t => `<button class="n8n-tag-pill" style="${st.overviewTag===t?'border-color:var(--n8n-coral);background:var(--n8n-coral-soft);color:var(--n8n-coral);':''}" onclick="st.overviewTag='${esc(t)}';renderSectionPage('overview')">${esc(t)}</button>`).join('')}
    </div>` : ''}
  `;
}

async function renderSectionPage(sec) {
  const alt = document.getElementById('altPage');
  alt.innerHTML = '<div style="padding:28px;color:var(--n8n-text-2);font-size:13px;">Lade Ansicht…</div>';
  if (sec === 'overview') {
    const r = await fetch('/api/n8n/workflows');
    const d = await r.json();
    st.workflows = d.data || [];
    document.getElementById('navWfCount').textContent = st.workflows.length;
    renderWfPicker();

    if (st.overviewTab === 'credentials') {
      const crRes = await fetch('/rest/credentials');
      if (crRes.ok) { const cr = await crRes.json(); st.savedCredentials = cr.data || []; }
      alt.innerHTML = renderOverviewToolbar() + `
        <div style="margin-bottom:10px;display:flex;justify-content:space-between;align-items:center;">
          <span style="color:var(--n8n-text-2);">${st.savedCredentials.length} verschlüsselte Anmeldedaten</span>
          <button class="n8n-btn n8n-btn-coral" onclick="setSection('credentials')">+ Verwalten / Neu anlegen</button>
        </div>
        ${(st.savedCredentials || []).map(cr => `<div class="n8n-wf-card" style="cursor:default;">
          <div><strong>${esc(cr.name)}</strong> <span class="n8n-tag-pill mono">${esc(cr.type)}</span> <span class="n8n-tag-pill" style="color:var(--n8n-green);">${esc(cr.cipherFormat || 'AES-256-CBC')}</span></div>
          <div style="display:flex;gap:6px;"><button class="n8n-btn" onclick="testSavedCredential('${esc(cr.type)}')">Testen</button><button class="n8n-btn" style="color:var(--n8n-red);" onclick="deleteCredential('${esc(cr.id)}')">Löschen</button></div>
        </div>`).join('') || '<div style="color:var(--n8n-text-3);">Noch keine Anmeldedaten hinterlegt.</div>'}
      `;
      return;
    }
    if (st.overviewTab === 'executions') {
      const exRes = await fetch('/api/n8n/executions?limit=50');
      const exData = exRes.ok ? await exRes.json() : { data: [] };
      const list = exData.data || [];
      alt.innerHTML = renderOverviewToolbar() + (list.map(e => `
        <div class="n8n-wf-card" onclick="openExecutionOnCanvas('${esc(e.workflowId)}','${esc(e.id)}')">
          <div>
            <strong style="color:${e.status==='success'?'var(--n8n-green)':'var(--n8n-red)'};">${e.status==='success'?'✓ Erfolgreich':'✕ Fehler'}</strong>
            <span style="font-weight:600;margin-left:8px;">${esc(e.workflowName || e.workflowId)}</span>
            <div class="mono" style="font-size:11px;color:var(--n8n-text-3);margin-top:3px;">${esc((e.startedAt||'').replace('T',' ').slice(0,19))} · ${e.durationMs}ms · Modus: ${esc(e.mode)}</div>
          </div>
          <button class="n8n-btn">Auf Canvas ansehen →</button>
        </div>
      `).join('') || '<div style="color:var(--n8n-text-3);">Noch keine Ausführungen vorhanden.</div>');
      return;
    }

    alt.innerHTML = renderOverviewToolbar() + `<div id="ovList"></div>`;
    filterOverviewList();
  } else if (sec === 'templates') {
    const tpls = [
      {
        name: 'Lead-Qualifizierung → Conrad CRM',
        desc: 'Webhook-Anfrage → KI-Qualifizierung (Conrad KI Engine) → Verzweigung → Conrad CRM Firma/Deal anlegen',
        nodes: [
          { id: 't1', name: 'Webhook', type: 'n8n-nodes-base.webhook', position: [140, 260], parameters: { path: 'lead-qualify', httpMethod: 'POST' } },
          { id: 't2', name: 'KI-Agent', type: '@n8n/n8n-nodes-langchain.agent', position: [410, 260], parameters: { text: '={{ $json.message }}' } },
          { id: 't3', name: 'Ollama Chat-Modell', type: '@n8n/n8n-nodes-langchain.lmChatOllama', position: [390, 440], parameters: { model: 'conrad-n8n-agent:0.6b' } },
          { id: 't4', name: 'Conrad CRM', type: 'n8n-nodes-base.twentyCrm', position: [720, 260], parameters: { resource: 'company', operation: 'create' } },
          { id: 't5', name: 'Auf Webhook antworten', type: 'n8n-nodes-base.respondToWebhook', position: [960, 260], parameters: { respondWith: 'json' } }
        ],
        connections: { 'Webhook': { main: [[{ node: 'KI-Agent', type: 'main', index: 0 }]] }, 'KI-Agent': { main: [[{ node: 'Conrad CRM', type: 'main', index: 0 }]] }, 'Conrad CRM': { main: [[{ node: 'Auf Webhook antworten', type: 'main', index: 0 }]] }, 'Ollama Chat-Modell': { ai_languageModel: [[{ node: 'KI-Agent', type: 'ai_languageModel', index: 0 }]] } }
      },
      {
        name: 'E-Mail-Triage & Entwurfs-Assistent (Safe Draft)',
        desc: 'E-Mail Trigger (IMAP) → KI-Klassifizierung → Wenn Antwort nötig → Gmail-Entwurf speichern (kein Blindversand)',
        nodes: [
          { id: 'g1', name: 'E-Mail Trigger (IMAP)', type: 'n8n-nodes-base.emailReadImap', position: [160, 260], parameters: {} },
          { id: 'g2', name: 'KI Klassifizierung', type: 'n8n-nodes-base.httpRequest', position: [400, 260], parameters: { method: 'POST', url: 'http://127.0.0.1:11434/api/chat' } },
          { id: 'g3', name: 'Wenn Antwort nötig', type: 'n8n-nodes-base.if', position: [640, 260], parameters: {} },
          { id: 'g4', name: 'Gmail-Entwurf', type: 'n8n-nodes-base.gmail', position: [900, 190], parameters: { operation: 'createDraft', draftOnly: true } },
          { id: 'g5', name: 'Ablage ohne Antwort', type: 'n8n-nodes-base.noOp', position: [900, 340], parameters: {} }
        ],
        connections: { 'E-Mail Trigger (IMAP)': { main: [[{ node: 'KI Klassifizierung', type: 'main', index: 0 }]] }, 'KI Klassifizierung': { main: [[{ node: 'Wenn Antwort nötig', type: 'main', index: 0 }]] }, 'Wenn Antwort nötig': { main: [[{ node: 'Gmail-Entwurf', type: 'main', index: 0 }], [{ node: 'Ablage ohne Antwort', type: 'main', index: 0 }]] } }
      },
      {
        name: 'Sprachmemo → Handwerks-Angebot (Elektro Weber)',
        desc: 'Webhook (Transkript) → Positionen per Code extrahieren → Datentabelle Preiskatalog → E-Mail Entwurf',
        nodes: [
          { id: 's1', name: 'Webhook', type: 'n8n-nodes-base.webhook', position: [140, 260], parameters: { path: 'angebot-memo', httpMethod: 'POST' } },
          { id: 's2', name: 'Positionen parsen', type: 'n8n-nodes-base.code', position: [380, 260], parameters: { language: 'javaScript', jsCode: 'const item = $input.first().json;\nreturn [{ json: { kunde: item.kunde || "Elektro Weber GmbH", netto: 1450, positionen: ["Wallbox Montage", "Zählerschrank-Check"] } }];' } },
          { id: 's3', name: 'Datentabelle', type: 'n8n-nodes-base.dataTable', position: [620, 260], parameters: { operation: 'getRows', tableName: 'Lead-Scoring-Regeln' } },
          { id: 's4', name: 'E-Mail Entwurf', type: 'n8n-nodes-base.emailSend', position: [860, 260], parameters: { draftOnly: true, subject: 'Ihr Angebot von Elektro Weber' } }
        ],
        connections: { 'Webhook': { main: [[{ node: 'Positionen parsen', type: 'main', index: 0 }]] }, 'Positionen parsen': { main: [[{ node: 'Datentabelle', type: 'main', index: 0 }]] }, 'Datentabelle': { main: [[{ node: 'E-Mail Entwurf', type: 'main', index: 0 }]] } }
      },
      {
        name: 'Eingangsrechnung GoBD & DATEV-Prüfung',
        desc: 'Manueller Trigger → Felder prüfen (Set) → Betrag > 250 € (If) → Freigabe-Entwurf',
        nodes: [
          { id: 'd1', name: 'Manueller Trigger', type: 'n8n-nodes-base.manualTrigger', position: [160, 260], parameters: {} },
          { id: 'd2', name: 'Felder normalisieren', type: 'n8n-nodes-base.set', position: [400, 260], parameters: { mode: 'raw', jsonOutput: '{"lieferant":"SHK Rhein-Neckar GmbH","betrag":480,"gobdOk":true}' } },
          { id: 'd3', name: 'Prüfung > 250 €', type: 'n8n-nodes-base.if', position: [640, 260], parameters: {} },
          { id: 'd4', name: 'Freigabe anfordern', type: 'n8n-nodes-base.emailSend', position: [900, 200], parameters: { draftOnly: true } }
        ],
        connections: { 'Manueller Trigger': { main: [[{ node: 'Felder normalisieren', type: 'main', index: 0 }]] }, 'Felder normalisieren': { main: [[{ node: 'Prüfung > 250 €', type: 'main', index: 0 }]] }, 'Prüfung > 250 €': { main: [[{ node: 'Freigabe anfordern', type: 'main', index: 0 }], []] } }
      }
    ];
    window._n8nTpls = tpls;
    alt.innerHTML = `<h2 style="margin:0 0 6px;font-size:20px;">Workflow-Vorlagen</h2><div style="color:var(--n8n-text-2);margin-bottom:16px;">Praxisnahe KMU-Automatisierungen mit 1 Klick übernehmen</div>` + tpls.map((t, i) => `
      <div class="n8n-wf-card" onclick="useTemplate(${i})">
        <div style="min-width:0;flex:1;">
          <strong style="font-size:14px;">${esc(t.name)}</strong>
          <div style="color:var(--n8n-text-2);font-size:12px;margin-top:3px;">${esc(t.desc)}</div>
          <div style="display:flex;gap:5px;flex-wrap:wrap;margin-top:8px;">${t.nodes.map(n => `<span class="n8n-tag-pill">${esc(n.name)}</span>`).join('')}</div>
        </div>
        <button class="n8n-btn n8n-btn-coral">Vorlage verwenden</button>
      </div>
    `).join('');
  } else if (sec === 'credentials') {
    const [cRes, crRes] = await Promise.all([
      st.credCatalog.length ? Promise.resolve(null) : fetch('/types/credentials.json'),
      fetch('/rest/credentials')
    ]);
    if (cRes && cRes.ok) { const cl = await cRes.json(); st.credCatalog = Array.isArray(cl) ? cl : (cl.data || []); }
    if (crRes.ok) { const cr = await crRes.json(); st.savedCredentials = cr.data || []; }
    const selType = st._newCredType || (st.credCatalog[0]?.name || 'httpHeaderAuth');
    const typeSpec = st.credCatalog.find(c => c.name === selType) || st.credCatalog[0];
    const typeOpts = st.credCatalog.map(c => `<option value="${esc(c.name)}" ${c.name===selType?'selected':''}>${esc(c.displayName || c.name)}</option>`).join('');
    const fieldInputs = (typeSpec?.properties || [{ name: 'value', displayName: 'Schlüssel / Wert', typeOptions: { password: true } }]).map(fp => {
      const isPwd = fp.typeOptions?.password;
      return `<div><div style="font-size:11px;color:var(--n8n-text-2);margin-bottom:3px;">${esc(fp.displayName || fp.name)}</div><input class="n8n-input cred-dyn-inp" type="${isPwd?'password':'text'}" data-ck="${esc(fp.name)}" value="${esc(fp.default||'')}" placeholder="${esc(fp.displayName || fp.name)}"></div>`;
    }).join('');
    alt.innerHTML = `
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:14px;flex-wrap:wrap;gap:10px;">
        <div><h2 style="margin:0;font-size:20px;">Anmeldedaten (${st.savedCredentials.length})</h2><div style="color:var(--n8n-text-2);font-size:12.5px;">OpenSSL-kompatible AES-256-CBC Verschlüsselung (Salted__) · ${st.credCatalog.length} unterstützte Typen</div></div>
      </div>
      <div class="n8n-wf-card" style="flex-direction:column;align-items:stretch;cursor:default;margin-bottom:16px;">
        <strong style="margin-bottom:8px;">+ Neue Anmeldedaten speichern</strong>
        <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(190px,1fr));gap:10px;">
          <div><div style="font-size:11px;color:var(--n8n-text-2);margin-bottom:3px;">Typ (${st.credCatalog.length})</div><select id="newCredTypeSel" class="n8n-select" onchange="st._newCredType=this.value;renderSectionPage('credentials')">${typeOpts}</select></div>
          <div><div style="font-size:11px;color:var(--n8n-text-2);margin-bottom:3px;">Bezeichnung *</div><input id="newCredNameInp" class="n8n-input" placeholder="z.B. OpenAI Produktiv"></div>
          ${fieldInputs}
        </div>
        <div style="display:flex;justify-content:flex-end;gap:8px;margin-top:10px;">
          <button class="n8n-btn" onclick="testNewCredential()">Verbindung testen</button>
          <button class="n8n-btn n8n-btn-coral" id="saveCredBtn" onclick="createNewCredential()">Verschlüsselt speichern</button>
        </div>
      </div>
      <div id="credsList">
        ${(st.savedCredentials || []).map(cr => `<div class="n8n-wf-card" style="cursor:default;">
          <div>
            <strong>${esc(cr.name)}</strong> <span class="n8n-tag-pill mono">${esc(cr.type)}</span> <span class="n8n-tag-pill" style="color:var(--n8n-green);">${esc(cr.cipherFormat || 'AES-256-CBC')}</span>
            <div class="mono" style="font-size:11px;color:var(--n8n-text-3);margin-top:4px;">${esc(JSON.stringify(cr.data || {}))}</div>
          </div>
          <div style="display:flex;gap:6px;">
            <button class="n8n-btn" onclick="testSavedCredential('${esc(cr.type)}')">Testen</button>
            <button class="n8n-btn" style="color:var(--n8n-red);" onclick="deleteCredential('${esc(cr.id)}')">Löschen</button>
          </div>
        </div>`).join('') || '<div style="color:var(--n8n-text-3);">Noch keine Anmeldedaten hinterlegt.</div>'}
      </div>`;
  } else if (sec === 'data-tables') {
    const dtRes = await fetch('/rest/data-tables');
    const dtData = dtRes.ok ? await dtRes.json() : { data: [] };
    st.dataTables = dtData.data || [];
    if (!st.selectedDataTableId && st.dataTables.length) st.selectedDataTableId = st.dataTables[0].id;
    let activeTable = st.dataTables.find(t => t.id === st.selectedDataTableId) || st.dataTables[0];
    let rows = [];
    if (activeTable) {
      st.selectedDataTableId = activeTable.id;
      const rr = await fetch(`/rest/data-tables/${encodeURIComponent(activeTable.id)}/rows`);
      if (rr.ok) { const rd = await rr.json(); rows = rd.data || []; }
    }
    st.dataTableRows = rows;
    const colKeys = activeTable ? Array.from(new Set([
      ...((activeTable.columns || []).map(c => c.name)),
      ...rows.flatMap(r => Object.keys(r || {}).filter(k => k !== 'id'))
    ])) : [];
    alt.innerHTML = `
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:14px;flex-wrap:wrap;gap:10px;">
        <div><h2 style="margin:0;font-size:20px;">Datentabellen (${st.dataTables.length})</h2><div style="color:var(--n8n-text-2);font-size:12.5px;">Persistente Tabellen für Workflows &amp; Lookups · Doppelklick auf Zelle zum Bearbeiten</div></div>
        <div style="display:flex;gap:8px;align-items:center;">
          <input id="newDtName" class="n8n-input" style="width:190px;" placeholder="Tabellenname *">
          <button class="n8n-btn n8n-btn-coral" id="createDtBtn" onclick="createDataTable()">+ Neue Tabelle</button>
        </div>
      </div>
      <div style="display:grid;grid-template-columns:250px 1fr;gap:14px;">
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;padding:8px;">
          <strong style="display:block;padding:6px 8px;font-size:12px;color:var(--n8n-text-2);">Tabellen</strong>
          ${st.dataTables.map(t => `<div class="n8n-wf-card" style="padding:8px 10px;border-color:${activeTable?.id===t.id?'var(--n8n-coral)':'var(--n8n-border)'};" onclick="st.selectedDataTableId='${esc(t.id)}';st.editingDtRowId=null;renderSectionPage('data-tables')">
            <div><strong>${esc(t.name)}</strong><div style="font-size:11px;color:var(--n8n-text-3);">${t.rowCount || 0} Zeilen</div></div>
            <button class="n8n-btn" style="height:24px;color:var(--n8n-red);" onclick="event.stopPropagation();deleteDataTable('${esc(t.id)}')">×</button>
          </div>`).join('') || '<div style="padding:12px;color:var(--n8n-text-3);font-size:12px;">Noch keine Datentabellen.</div>'}
        </div>
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;padding:14px;overflow:auto;">
          ${activeTable ? `
            <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;flex-wrap:wrap;gap:8px;">
              <strong>${esc(activeTable.name)} · ${rows.length} Zeilen</strong>
              <div style="display:flex;gap:6px;flex:1;max-width:480px;">
                <input id="newDtRowJson" class="n8n-input mono" value='{"schluessel":"demo","wert":"aktiv"}' placeholder='{"schluessel":"...","wert":"..."}'>
                <button class="n8n-btn n8n-btn-coral" id="addDtRowBtn" onclick="insertDataTableRow('${esc(activeTable.id)}')">+ Zeile</button>
              </div>
            </div>
            ${rows.length ? `<table style="width:100%;border-collapse:collapse;font-size:12px;">
              <thead><tr style="border-bottom:1px solid var(--n8n-border);text-align:left;background:var(--n8n-subtle-bg);">
                <th style="padding:7px 8px;">ID</th>
                ${colKeys.map(k => `<th style="padding:7px 8px;">${esc(k)}</th>`).join('')}
                <th style="padding:7px 8px;text-align:right;">Aktionen</th>
              </tr></thead>
              <tbody>${rows.map(rw => `<tr style="border-bottom:1px solid var(--n8n-border);">
                <td class="mono" style="padding:6px 8px;color:var(--n8n-text-3);">${esc(rw.id)}</td>
                ${colKeys.map(k => `<td style="padding:4px 8px;">
                  <input class="n8n-input mono" style="padding:4px 6px;border-color:transparent;background:transparent;" value="${esc(typeof rw[k]==='object'?JSON.stringify(rw[k]):(rw[k]??''))}" onchange="inlineUpdateDataTableRow('${esc(activeTable.id)}',${Number(rw.id)},'${esc(k)}',this.value)">
                </td>`).join('')}
                <td style="padding:6px 8px;text-align:right;"><button class="n8n-btn" style="height:24px;color:var(--n8n-red);" onclick="deleteDataTableRow('${esc(activeTable.id)}',${Number(rw.id)})">Löschen</button></td>
              </tr>`).join('')}</tbody>
            </table>` : '<div style="color:var(--n8n-text-3);font-size:12.5px;">Noch keine Zeilen in dieser Tabelle.</div>'}
          ` : '<div style="color:var(--n8n-text-3);">Wähle links eine Tabelle aus oder erstelle eine neue.</div>'}
        </div>
      </div>`;
  } else if (sec === 'variables') {
    const vRes = await fetch('/rest/variables');
    const vData = vRes.ok ? await vRes.json() : { data: [] };
    st.variables = vData.data || [];
    alt.innerHTML = `<h2 style="margin:0 0 10px;font-size:20px;">Variablen (<code>$vars.*</code>)</h2>
      <div class="n8n-wf-card" style="cursor:default;gap:8px;flex-wrap:wrap;margin-bottom:14px;">
        <input id="newVarKey" class="n8n-input mono" style="width:200px;" placeholder="SCHLUESSEL *">
        <input id="newVarVal" class="n8n-input mono" style="flex:1;min-width:200px;" placeholder="Wert">
        <button class="n8n-btn n8n-btn-coral" id="createVarBtn" onclick="createVariable()">+ Variable speichern</button>
      </div>
      ${(st.variables || []).map(vr => `<div class="n8n-wf-card" style="cursor:default;gap:10px;">
        <span class="mono" style="font-weight:600;min-width:190px;cursor:pointer;" title="Ausdruck kopieren" onclick="navigator.clipboard?.writeText('{{ $vars.${esc(vr.key)} }}');toast('Kopiert: $vars.${esc(vr.key)}')">$vars.${esc(vr.key)}</span>
        <input class="n8n-input mono" style="flex:1;" value="${esc(vr.value)}" onchange="updateVariable('${esc(vr.id)}',this.value)">
        <button class="n8n-btn" style="color:var(--n8n-red);" onclick="deleteVariable('${esc(vr.id)}')">Löschen</button>
      </div>`).join('') || '<div style="color:var(--n8n-text-3);margin-bottom:12px;">Noch keine benutzerdefinierten Variablen gespeichert.</div>'}
      <h3 style="margin:18px 0 8px;font-size:14px;color:var(--n8n-text-2);">System-Laufzeitkonstanten (Conrad Automation Engine)</h3>
      <div class="n8n-wf-card" style="cursor:default;"><span class="mono">CONRAD_KI_ENGINE_URL</span><span class="mono" style="color:var(--n8n-green);">http://127.0.0.1:11434</span></div>
      <div class="n8n-wf-card" style="cursor:default;"><span class="mono">CONRAD_CRM_GATEWAY</span><span class="mono">In-Process Kurzschluss (&lt;2ms Latenz)</span></div>
      <div class="n8n-wf-card" style="cursor:default;"><span class="mono">EMAIL_SAFETY_MODE</span><span class="mono" style="color:var(--n8n-green);">draftOnly = true</span></div>`;
  } else if (sec === 'insights') {
    const r = await fetch('/api/showcase/metrics'); const m = await r.json();
    alt.innerHTML = `<h2 style="margin:0 0 14px;font-size:20px;">Statistiken · Conrad Automation Telemetrie</h2>
      <div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:12px;">
        <div class="n8n-wf-card" style="flex-direction:column;align-items:flex-start;"><span style="color:var(--n8n-text-2);">Workflows gesamt</span><strong style="font-size:26px;">${m.n8n?.workflows ?? st.workflows.length}</strong></div>
        <div class="n8n-wf-card" style="flex-direction:column;align-items:flex-start;"><span style="color:var(--n8n-text-2);">Aktive Workflows</span><strong style="font-size:26px;color:var(--n8n-green);">${m.n8n?.activeWorkflows ?? 0}</strong></div>
        <div class="n8n-wf-card" style="flex-direction:column;align-items:flex-start;"><span style="color:var(--n8n-text-2);">Ausführungen gesamt</span><strong style="font-size:26px;">${m.n8n?.executions ?? 0}</strong></div>
        <div class="n8n-wf-card" style="flex-direction:column;align-items:flex-start;"><span style="color:var(--n8n-text-2);">Effizienzfaktor vs. Enterprise</span><strong style="font-size:26px;color:var(--n8n-coral);">bis zu 100×</strong></div>
      </div>`;
  }
}

async function createNewCredential() {
  const type = document.getElementById('newCredTypeSel')?.value || 'httpHeaderAuth';
  const name = document.getElementById('newCredNameInp')?.value?.trim() || 'Neue Anmeldedaten';
  const data = {};
  document.querySelectorAll('.cred-dyn-inp').forEach(el => { if (el.value !== '') data[el.dataset.ck] = el.value; });
  const r = await fetch('/rest/credentials', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name, type, data }) });
  if (r.ok) { toast('AES-256-CBC verschlüsselt gespeichert ✓'); renderSectionPage('credentials'); }
  else toast('Fehler beim Speichern');
}
async function testNewCredential() {
  const type = document.getElementById('newCredTypeSel')?.value || 'httpHeaderAuth';
  await testSavedCredential(type);
}
async function testSavedCredential(type) {
  const r = await fetch('/rest/credentials/test', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ type }) });
  const d = await r.json().catch(() => ({}));
  toast(d.message || 'Anmeldedaten OK ✓');
}
async function deleteCredential(id) {
  if (!confirm('Anmeldedaten wirklich löschen?')) return;
  await fetch(`/rest/credentials/${encodeURIComponent(id)}`, { method: 'DELETE' });
  toast('Gelöscht'); renderSectionPage(st.section);
}
async function createDataTable() {
  const name = document.getElementById('newDtName')?.value?.trim();
  if (!name) { toast('Tabellenname eingeben'); return; }
  const r = await fetch('/rest/data-tables', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name }) });
  if (r.ok) { const t = await r.json(); st.selectedDataTableId = t.id; toast('Datentabelle angelegt ✓'); renderSectionPage('data-tables'); }
}
async function deleteDataTable(id) {
  if (!confirm('Datentabelle inkl. Zeilen löschen?')) return;
  await fetch(`/rest/data-tables/${encodeURIComponent(id)}`, { method: 'DELETE' });
  if (st.selectedDataTableId === id) st.selectedDataTableId = null;
  toast('Tabelle gelöscht'); renderSectionPage('data-tables');
}
async function insertDataTableRow(tid) {
  let body = {};
  try { body = JSON.parse(document.getElementById('newDtRowJson')?.value || '{}'); } catch { toast('Ungültiges Zeilen-JSON'); return; }
  const r = await fetch(`/rest/data-tables/${encodeURIComponent(tid)}/rows`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (r.ok) { toast('Zeile eingefügt ✓'); renderSectionPage('data-tables'); }
}
async function inlineUpdateDataTableRow(tid, rid, colKey, newVal) {
  const row = (st.dataTableRows || []).find(r => Number(r.id) === Number(rid));
  if (!row) return;
  const copy = { ...row };
  delete copy.id;
  let parsed = newVal;
  if (newVal !== '' && !isNaN(Number(newVal)) && !/^0\d/.test(newVal)) parsed = Number(newVal);
  else if (newVal === 'true') parsed = true;
  else if (newVal === 'false') parsed = false;
  copy[colKey] = parsed;
  const r1 = await fetch(`/rest/data-tables/${encodeURIComponent(tid)}/rows`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(copy) });
  if (r1.ok) {
    await fetch(`/rest/data-tables/${encodeURIComponent(tid)}/rows/${rid}`, { method: 'DELETE' });
    toast('Zelle gespeichert ✓');
    renderSectionPage('data-tables');
  }
}
async function deleteDataTableRow(tid, rid) {
  await fetch(`/rest/data-tables/${encodeURIComponent(tid)}/rows/${rid}`, { method: 'DELETE' });
  toast('Zeile entfernt'); renderSectionPage('data-tables');
}
async function createVariable() {
  const key = document.getElementById('newVarKey')?.value?.trim();
  const value = document.getElementById('newVarVal')?.value ?? '';
  if (!key) { toast('Schlüssel eingeben'); return; }
  const r = await fetch('/rest/variables', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ key, value }) });
  if (r.ok) { toast('Variable gespeichert ✓'); renderSectionPage('variables'); }
}
async function updateVariable(id, value) {
  await fetch(`/rest/variables/${encodeURIComponent(id)}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ value }) });
  toast('Aktualisiert ✓');
}
async function deleteVariable(id) {
  await fetch(`/rest/variables/${encodeURIComponent(id)}`, { method: 'DELETE' });
  toast('Variable gelöscht'); renderSectionPage('variables');
}

function filterOverviewList() {
  const q = (document.getElementById('ovSearch')?.value || '').toLowerCase();
  let rows = st.workflows.filter(w => {
    const qOk = !q || w.name.toLowerCase().includes(q) || wfWebhookPath(w).toLowerCase().includes(q);
    const tOk = st.overviewTag === 'Alle' || (w.tags || []).some(t => (typeof t === 'string' ? t : t.name) === st.overviewTag);
    return qOk && tOk;
  });
  if (st.overviewSort === 'name') rows.sort((a, b) => (a.name || '').localeCompare(b.name || ''));
  else if (st.overviewSort === 'active') rows.sort((a, b) => Number(!!b.active) - Number(!!a.active));
  else rows.sort((a, b) => String(b.updatedAt || '').localeCompare(String(a.updatedAt || '')));

  document.getElementById('ovList').innerHTML = rows.map(w => {
    const wp = wfWebhookPath(w);
    return `
    <div class="n8n-wf-card" onclick="selectWorkflow('${esc(w.id)}')">
      <div style="min-width:0;flex:1;">
        <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
          <strong style="font-size:14px;">${esc(w.name)}</strong>
          ${(w.tags||[]).map(t => `<span class="n8n-tag-pill">${esc(typeof t==='string'?t:t.name)}</span>`).join('')}
        </div>
        <div class="mono" style="font-size:11px;color:var(--n8n-text-3);margin-top:4px;">
          ${nodeCountOf(w)} Nodes · Trigger: ${triggerCountOf(w)} · Läufe: ${w.executionCount || 0}${wp ? ' · /webhook/' + esc(wp) : ''}
        </div>
      </div>
      <div style="display:flex;align-items:center;gap:10px;" onclick="event.stopPropagation()">
        <span style="font-size:12px;color:${w.active ? 'var(--n8n-green)' : 'var(--n8n-text-3)'};">${w.active ? 'Aktiv' : 'Inaktiv'}</span>
        <button class="n8n-switch ${w.active ? 'on' : ''}" onclick="toggleWfInList('${esc(w.id)}')"></button>
        <button class="n8n-btn" style="height:26px;padding:0 8px;" onclick="duplicateWfById('${esc(w.id)}')" title="Duplizieren">⧉</button>
        <button class="n8n-btn" style="height:26px;padding:0 8px;color:var(--n8n-red);" onclick="deleteWfById('${esc(w.id)}','${esc(w.name)}')" title="Löschen">🗑</button>
      </div>
    </div>`;
  }).join('') || '<div style="color:var(--n8n-text-3);">Keine passenden Workflows gefunden.</div>';
}
async function toggleWfInList(id) {
  await fetch(`/api/n8n/workflows/${encodeURIComponent(id)}/toggle`, { method: 'POST' });
  const r = await fetch('/api/n8n/workflows'); const d = await r.json(); st.workflows = d.data || [];
  filterOverviewList();
}
async function duplicateWfById(id) {
  const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(id)}/duplicate`, { method: 'POST' });
  if (r.ok) { toast('Workflow dupliziert'); renderSectionPage('overview'); }
}
async function deleteWfById(id, name) {
  if (!confirm(`Workflow "${name}" löschen?`)) return;
  await fetch(`/api/n8n/workflows/${encodeURIComponent(id)}`, { method: 'DELETE' });
  toast('Workflow gelöscht'); renderSectionPage('overview');
}
async function useTemplate(idx) {
  const tpl = window._n8nTpls[idx];
  const r = await fetch('/api/n8n/workflows', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name: tpl.name, active: true, nodes: tpl.nodes, connections: tpl.connections }) });
  const wf = await r.json();
  st.currentWf = null;
  await init();
  await selectWorkflow(wf.id);
}

async function openExecutionOnCanvas(wfId, execId) {
  if (!st.currentWf || st.currentWf.id !== wfId) {
    const wr = await fetch(`/api/n8n/workflows/${encodeURIComponent(wfId)}`);
    if (wr.ok) st.currentWf = await wr.json();
  }
  const er = await fetch(`/api/n8n/executions/${encodeURIComponent(execId)}`);
  if (!er.ok) { toast('Ausführung konnte nicht geladen werden'); return; }
  const ex = await er.json();
  loadExecutionIntoCanvas(ex);
}

function loadExecutionIntoCanvas(ex) {
  st.viewingExecutionId = ex.id;
  st.lastExec = ex;
  st.nodeStatusMap = {};
  const steps = ex.nodeLogs || ex.data?.steps || [];
  steps.forEach(l => {
    st.nodeStatusMap[l.node] = {
      status: l.status || 'success',
      durationMs: l.durationMs || 0,
      input: l.input ?? ex.data?.triggerPayload ?? null,
      output: l.output ?? (Array.isArray(l.items) && l.items.length === 1 ? l.items[0] : l.items) ?? null,
      items: l.items || (l.output !== undefined ? [l.output] : []),
      outputCount: l.outputCount ?? (Array.isArray(l.items) ? l.items.length : 1),
      error: l.error || null
    };
  });
  st.section = 'editor';
  st.editorTab = 'editor';
  document.querySelectorAll('#centerTabs .n8n-seg-btn').forEach(b => b.classList.toggle('active', b.dataset.tab === 'editor'));
  const banner = document.getElementById('execInspectBanner');
  if (banner) {
    document.getElementById('execInspectLabel').innerHTML = `<strong>Ausführung ${esc(ex.id.slice(0, 10))}</strong> · ${ex.status === 'success' ? '✓ Erfolgreich' : '✕ Fehler'} · ${ex.durationMs}ms · ${(ex.startedAt || '').replace('T', ' ').slice(0, 19)}`;
    banner.classList.add('open');
  }
  updateHeaderState();
  setSection('editor');
  populateBottomLogs(ex);
  toggleBottomLogs(true);
  fitToView();
}

function exitExecutionView() {
  st.viewingExecutionId = null;
  document.getElementById('execInspectBanner')?.classList.remove('open');
  st.nodeStatusMap = {};
  renderCanvas();
  toggleBottomLogs(false);
}

async function retryViewedExecution() {
  if (!st.viewingExecutionId) return;
  await retryExecution(st.viewingExecutionId);
}

async function retryExecution(execId) {
  try {
    const r = await fetch(`/rest/executions/${encodeURIComponent(execId)}/retry`, { method: 'POST' });
    const raw = await r.json();
    const res = raw?.data || raw;
    if (!r.ok || res?.error) { toast(res?.error || 'Wiederholung fehlgeschlagen'); return; }
    toast(`Ausführung wiederholt (${res.status || 'ok'} · ${res.durationMs ?? 0}ms)`);
    if (st.editorTab === 'executions') await renderAltEditorTab();
    else loadExecutionIntoCanvas({ ...res, id: res.executionId || execId });
  } catch (e) { toast('Fehler: ' + (e?.message || e)); }
}

async function renderAltEditorTab() {
  const alt = document.getElementById('altPage');
  alt.innerHTML = '<div style="padding:28px;color:var(--n8n-text-2);font-size:13px;">Lade Ausführungen…</div>';
  if (st.editorTab === 'executions') {
    const wid = st.currentWf?.id || '';
    const r = await fetch(`/api/n8n/executions?workflowId=${encodeURIComponent(wid)}&limit=50`);
    const d = await r.json();
    st.executions = (d.data || []).filter(e => st.execFilter === 'all' || e.status === st.execFilter);
    const ex = st.executions[st.selectedExecIdx] || st.executions[0];
    alt.innerHTML = `
      <div style="display:grid;grid-template-columns:320px 1fr;gap:14px;height:100%;">
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;overflow-y:auto;padding:8px;">
          <div style="display:flex;justify-content:space-between;align-items:center;padding:4px 6px 8px;">
            <strong style="font-size:12px;color:var(--n8n-text-2);">Ausführungen (${st.executions.length})</strong>
            <select class="n8n-select" style="width:auto;padding:3px 6px;font-size:11px;" onchange="st.execFilter=this.value;st.selectedExecIdx=0;renderAltEditorTab()">
              <option value="all" ${st.execFilter==='all'?'selected':''}>Alle</option>
              <option value="success" ${st.execFilter==='success'?'selected':''}>Erfolgreich</option>
              <option value="error" ${st.execFilter==='error'?'selected':''}>Fehler</option>
            </select>
          </div>
          ${st.executions.map((e, i) => `
            <div class="n8n-wf-card" style="padding:9px 10px;border-color:${i===st.selectedExecIdx?'var(--n8n-coral)':'var(--n8n-border)'};" onclick="st.selectedExecIdx=${i};renderAltEditorTab()">
              <div>
                <div style="font-weight:600;font-size:12px;color:${e.status==='success'?'var(--n8n-green)':'var(--n8n-red)'};">${e.status==='success'?'✓ Erfolgreich':'✕ Fehler'} · ${e.durationMs}ms</div>
                <div class="mono" style="font-size:10.5px;color:var(--n8n-text-3);">${esc(e.mode)} · ${esc((e.startedAt||'').replace('T',' ').slice(0,19))}</div>
              </div>
            </div>
          `).join('') || '<div style="padding:16px;color:var(--n8n-text-3);">Keine Ausführungen gefunden.</div>'}
        </div>
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;overflow:auto;padding:16px;">
          ${ex ? `<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;gap:8px;flex-wrap:wrap;">
            <strong>Ausführung ${esc(ex.id.slice(0,12))}</strong>
            <div style="display:flex;gap:8px;align-items:center;">
              <button class="n8n-btn n8n-btn-coral" onclick="loadExecutionIntoCanvas(st.executions[${st.selectedExecIdx}||0])">👁 Auf Canvas laden</button>
              <button class="n8n-btn" onclick="retryExecution('${esc(ex.id)}')">🔄 Wiederholen</button>
              <span class="n8n-tag-pill mono">${esc(cleanEngineName(ex.modelUsed))} · ${ex.durationMs}ms</span>
            </div>
          </div><pre class="mono" style="background:var(--n8n-subtle-bg);padding:12px;border-radius:6px;overflow:auto;font-size:11.5px;">${esc(cleanEngineName(JSON.stringify(ex, null, 2)))}</pre>` : '<div style="color:var(--n8n-text-3);">Wähle eine Ausführung aus.</div>'}
        </div>
      </div>
    `;
  } else if (st.editorTab === 'evaluations') {
    alt.innerHTML = `
      <h2 style="margin:0 0 8px;font-size:18px;">Auswertungen · Conrad KI Engine Test</h2>
      <p style="margin:0 0 14px;color:var(--n8n-text-2);">Führe <strong class="mono">${esc(st.currentWf?.name)}</strong> direkt gegen unsere spezialisierte <strong>Conrad KI Engine</strong> aus:</p>
      <div style="display:flex;gap:8px;margin-bottom:14px;">
        <input id="evalPrompt" class="n8n-input" value="Suche in Conrad CRM nach Elektro Weber und erstelle einen kurzen Entwurf" style="max-width:480px;">
        <button class="n8n-btn n8n-btn-coral" onclick="runEvalComparison()">▶ Conrad KI Engine testen</button>
      </div>
      <div id="evalResultsGrid" style="display:grid;grid-template-columns:repeat(auto-fit,minmax(320px,1fr));gap:12px;"></div>
    `;
  }
}

async function runEvalComparison() {
  const msg = document.getElementById('evalPrompt')?.value || 'Hallo';
  const m = document.getElementById('modelSelect')?.value || 'conrad-n8n-agent:0.6b';
  const grid = document.getElementById('evalResultsGrid');
  grid.innerHTML = `<div class="n8n-wf-card" id="eval-main" style="flex-direction:column;align-items:stretch;"><strong>⚡ Conrad KI Engine</strong><div style="color:var(--n8n-text-3);margin-top:6px;">läuft...</div></div>`;
  const card = document.getElementById('eval-main');
  try {
    const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/execute`, {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ payload: { message: msg, model: m }, model: m })
    });
    const d = await r.json();
    card.innerHTML = `<div style="display:flex;justify-content:space-between;"><strong>⚡ Conrad KI Engine</strong><span class="n8n-tag-pill" style="color:var(--n8n-green);">${d.durationMs}ms</span></div><pre class="mono" style="margin:8px 0 0;font-size:11px;max-height:260px;overflow:auto;">${esc(JSON.stringify(d.output || d.finalItems || d, null, 2))}</pre>`;
  } catch (e) { card.textContent = String(e); }
}

// Canvas Rendering & Interactions
function applyCanvasTransform() {
  document.getElementById('n8nViewport').style.transform = `translate(${st.panX}px, ${st.panY}px) scale(${st.zoom})`;
  const cvs = document.getElementById('n8nCanvas');
  if (cvs) {
    cvs.style.backgroundPosition = `${st.panX}px ${st.panY}px`;
    cvs.style.backgroundSize = `${20 * st.zoom}px ${20 * st.zoom}px`;
  }
  drawMinimap();
}
function zoomBy(delta, cx, cy) {
  const oldZ = st.zoom;
  const newZ = Math.max(0.3, Math.min(1.8, oldZ + delta));
  if (cx !== undefined && cy !== undefined) {
    const rect = document.getElementById('n8nCanvas').getBoundingClientRect();
    const ox = cx - rect.left, oy = cy - rect.top;
    st.panX = ox - ((ox - st.panX) / oldZ) * newZ;
    st.panY = oy - ((oy - st.panY) / oldZ) * newZ;
  }
  st.zoom = newZ;
  applyCanvasTransform();
}
function cleanEngineName(s) {
  return String(s || 'Conrad KI Engine').replace(/Local Rust Engine/gi, 'Conrad KI Engine').replace(/\bRust\b/gi, 'Conrad');
}
function fitToView() {
  const nodes = (st.currentWf?.nodes || []).filter(n => n.type !== 'n8n-nodes-base.stickyNote');
  if (!nodes.length) { st.panX = 60; st.panY = 70; st.zoom = 0.92; applyCanvasTransform(); return; }
  const xs = nodes.map(n => n.position?.[0] || 0), ys = nodes.map(n => n.position?.[1] || 0);
  const minX = Math.min(...xs);
  const maxX = Math.max(...nodes.map(n => (n.position?.[0] || 0) + nodeDimensions(n).w + 36));
  const minY = Math.min(...ys), maxY = Math.max(...ys) + 150;
  const cw = document.getElementById('n8nCanvas').clientWidth || 1000;
  const ch = document.getElementById('n8nCanvas').clientHeight || 620;
  st.zoom = Math.max(0.25, Math.min(1.0, Math.min((cw - 110) / Math.max(240, maxX - minX), (ch - 150) / Math.max(220, maxY - minY))));
  st.panX = (cw - (maxX - minX) * st.zoom) / 2 - minX * st.zoom;
  st.panY = (ch - (maxY - minY) * st.zoom) / 2 - minY * st.zoom;
  applyCanvasTransform();
}
function tidyUpLayout() {
  if (!st.currentWf?.nodes) return;
  pushHistory();
  const nodes = st.currentWf.nodes;
  const mains = nodes.filter(n => !isSubnode(n.type) && n.type !== 'n8n-nodes-base.stickyNote');
  const conns = st.currentWf.connections || {};
  // Topological column layout
  const inDeg = Object.fromEntries(mains.map(n => [n.name, 0]));
  const adj = Object.fromEntries(mains.map(n => [n.name, []]));
  for (const [s, ed] of Object.entries(conns)) {
    (ed?.main || []).forEach(arr => (arr || []).forEach(t => {
      if (adj[s] && inDeg[t.node] !== undefined) { adj[s].push(t.node); inDeg[t.node]++; }
    }));
  }
  const colOf = {};
  const q = mains.filter(n => inDeg[n.name] === 0).map(n => n.name);
  q.forEach(n => colOf[n] = 0);
  for (let i = 0; i < q.length; i++) {
    const u = q[i];
    for (const v of (adj[u] || [])) {
      colOf[v] = Math.max(colOf[v] || 0, (colOf[u] || 0) + 1);
      if (!q.includes(v)) q.push(v);
    }
  }
  const byCol = {};
  mains.forEach(n => { const c = colOf[n.name] ?? 0; (byCol[c] = byCol[c] || []).push(n); });
  Object.entries(byCol).forEach(([c, list]) => {
    list.forEach((n, row) => {
      const yOff = (row - (list.length - 1) / 2) * 160;
      n.position = [140 + Number(c) * 250, Math.round(250 + yOff)];
    });
  });
  const subs = nodes.filter(n => isSubnode(n.type));
  subs.forEach((n, idx) => { n.position = [380 + idx * 140, 460]; });
  st.dirty = true; updateHeaderState(); renderCanvas(); fitToView(); toast('Layout aufgeräumt');
}

function toggleSelectMode() {
  st.selectMode = !st.selectMode;
  document.getElementById('selectModeBtn')?.classList.toggle('active', st.selectMode);
  document.getElementById('n8nCanvas')?.classList.toggle('select-mode', st.selectMode);
}
function toggleMinimap() {
  st.showMinimap = !st.showMinimap;
  document.getElementById('n8nMinimap').style.display = st.showMinimap ? 'block' : 'none';
  document.getElementById('minimapToggleBtn')?.classList.toggle('active', st.showMinimap);
  if (st.showMinimap) drawMinimap();
}

function drawMinimap() {
  if (!st.showMinimap || !st.currentWf) return;
  const cvs = document.getElementById('minimapCanvas');
  if (!cvs) return;
  const ctx = cvs.getContext('2d');
  const W = cvs.width, H = cvs.height;
  ctx.clearRect(0, 0, W, H);
  const nodes = st.currentWf.nodes || [];
  if (!nodes.length) return;
  const xs = nodes.map(n => n.position?.[0] ?? 0), ys = nodes.map(n => n.position?.[1] ?? 0);
  const minX = Math.min(...xs) - 100, maxX = Math.max(...xs) + 320;
  const minY = Math.min(...ys) - 100, maxY = Math.max(...ys) + 260;
  const spanX = Math.max(600, maxX - minX), spanY = Math.max(400, maxY - minY);
  const sc = Math.min((W - 16) / spanX, (H - 16) / spanY);
  const ox = 8 - minX * sc, oy = 8 - minY * sc;
  st._minimapMeta = { minX, minY, sc, ox, oy };
  for (const n of nodes) {
    const x = (n.position?.[0] ?? 0) * sc + ox;
    const y = (n.position?.[1] ?? 0) * sc + oy;
    const d = nodeDimensions(n);
    ctx.fillStyle = st.selectedNodes.has(n.name) ? '#5b60e8' : (n.type === 'n8n-nodes-base.stickyNote' ? '#eab308' : (isTrigger(n.type) ? '#ff6d5a' : '#64748b'));
    ctx.fillRect(x, y, Math.max(5, d.w * sc), Math.max(4, d.h * sc));
  }
  const cw = document.getElementById('n8nCanvas')?.clientWidth || 1000;
  const ch = document.getElementById('n8nCanvas')?.clientHeight || 600;
  const vx = (-st.panX / st.zoom) * sc + ox;
  const vy = (-st.panY / st.zoom) * sc + oy;
  const vw = (cw / st.zoom) * sc;
  const vh = (ch / st.zoom) * sc;
  ctx.strokeStyle = '#ff6d5a';
  ctx.lineWidth = 1.2;
  ctx.strokeRect(vx, vy, vw, vh);
}

function nodeDimensions(n) {
  if (n.type === 'n8n-nodes-base.stickyNote') return { w: n.parameters?.width || 240, h: n.parameters?.height || 160 };
  if (isAgentRoot(n.type)) return { w: 224, h: 96 };
  if (isSubnode(n.type)) return { w: 68, h: 68 };
  return { w: 96, h: 96 };
}

function renderMarkdownMini(txt) {
  return esc(txt || '')
    .replace(/^### (.+)$/gm, '<h3>$1</h3>')
    .replace(/^## (.+)$/gm, '<h2>$1</h2>')
    .replace(/^# (.+)$/gm, '<h1>$1</h1>')
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    .replace(/`([^`]+)`/g, '<code class="mono" style="background:rgba(0,0,0,0.08);padding:1px 4px;border-radius:3px;">$1</code>')
    .replace(/\n/g, '<br>');
}

function outputPortCount(n, conns) {
  const outBranches = conns[n.name]?.main || [];
  let count = Math.max(1, outBranches.length);
  if (n.type === 'n8n-nodes-base.if') count = 2;
  else if (n.type === 'n8n-nodes-base.switch') {
    const ruleCnt = Array.isArray(n.parameters?.rules?.values) ? n.parameters.rules.values.length : 0;
    count = Math.max(2, ruleCnt + (n.parameters?.fallbackOutput === 'extra' ? 1 : 0), outBranches.length);
  } else if (n.type === 'n8n-nodes-base.stopAndError') count = 0;
  if (n.onError === 'continueErrorOutput') count = Math.max(count + 1, 2);
  return count;
}

function renderCanvas() {
  if (!st.currentWf) return;
  const layer = document.getElementById('nodesLayer');
  const nodes = st.currentWf.nodes || [];
  const conns = st.currentWf.connections || {};
  const nodeMap = Object.fromEntries(nodes.map(n => [n.name, n]));

  let html = '';
  for (const n of nodes) {
    const x = n.position?.[0] ?? 200;
    const y = n.position?.[1] ?? 200;
    const isSel = st.selectedNodes.has(n.name);
    if (n.type === 'n8n-nodes-base.stickyNote') {
      const w = n.parameters?.width || 240, h = n.parameters?.height || 160;
      const theme = STICKY_THEMES.find(t => t.id === Number(n.parameters?.color || 1)) || STICKY_THEMES[0];
      const isEditing = st.editingStickyName === n.name;
      html += `<div class="n8n-sticky ${isSel?'selected':''}" data-nodename="${esc(n.name)}" style="left:${x}px;top:${y}px;width:${w}px;height:${h}px;background:${theme.bg};border:1px solid ${theme.border};color:${theme.fg};" ondblclick="event.stopPropagation();st.editingStickyName='${esc(n.name)}';renderCanvas()">
        <div class="n8n-sticky-tb" onclick="event.stopPropagation()">
          ${STICKY_THEMES.map(t => `<button class="n8n-sticky-swatch" style="background:${t.bg};border-color:${t.border};" onclick="setStickyColor('${esc(n.name)}',${t.id})" title="Farbe"></button>`).join('')}
          <button class="n8n-tb-btn del" onclick="deleteNode('${esc(n.name)}')" title="Notiz löschen">🗑</button>
        </div>
        ${isEditing
          ? `<textarea class="n8n-sticky-ed" id="stickyEd-${esc(n.name)}" onblur="saveStickyText('${esc(n.name)}',this.value)" onkeydown="if(event.key==='Escape'){this.blur();}">${esc(n.parameters?.content || '')}</textarea>`
          : `<div class="n8n-sticky-content">${renderMarkdownMini(n.parameters?.content || 'Doppelklick zum Bearbeiten...')}</div>`}
        <div class="n8n-sticky-resize" data-resize-sticky="${esc(n.name)}" title="Größe ändern">⟋</div>
      </div>`;
      continue;
    }

    const m = metaFor(n.type);
    const trig = isTrigger(n.type);
    const sub = isSubnode(n.type);
    const root = isAgentRoot(n.type);
    const boxCls = root ? 'agent-root' : sub ? 'subnode' : trig ? 'trigger' : 'standard';
    const wrapCls = (root ? 'agent-root-wrap' : sub ? 'subnode-wrap' : '') + (isSel ? ' is-selected' : '');
    const stat = st.nodeStatusMap[n.name];
    const statCls = stat ? (stat.status === 'error' ? 'status-error' : 'status-success') : '';
    const selCls = isSel ? 'selected' : '';
    const disCls = n.disabled ? 'disabled-node' : '';
    const pinArr = st.currentWf.pinData?.[n.name];
    const isPinned = Array.isArray(pinArr) && pinArr.length > 0;
    const pinCls = isPinned ? 'pinned-node' : '';

    const portCount = outputPortCount(n, conns);
    const outBranches = conns[n.name]?.main || [];
    let portsHtml = '';
    if (!trig && !sub) portsHtml += `<div class="n8n-port-in" data-in-node="${esc(n.name)}"></div>`;
    if (!sub) {
      for (let b = 0; b < portCount; b++) {
        const topPct = portCount === 1 ? 50 : (portCount === 2 ? (b === 0 ? 32 : 68) : (18 + (64 * b) / (portCount - 1)));
        const isErrBranch = n.onError === 'continueErrorOutput' && b === portCount - 1;
        const lbl = isErrBranch ? 'error' : (n.type === 'n8n-nodes-base.if' ? (b === 0 ? 'true' : 'false') : (portCount > 1 ? String(b) : ''));
        const connected = Array.isArray(outBranches[b]) && outBranches[b].length > 0;
        if (lbl) portsHtml += `<span class="n8n-branch-lbl" style="top:calc(${topPct}% - 7px);${isErrBranch?'color:var(--n8n-red);':''}">${lbl}</span>`;
        portsHtml += `<div class="n8n-port-out" style="top:calc(${topPct}% - 5px);" data-out-node="${esc(n.name)}" data-out-branch="${b}" title="Ziehen zum Verbinden"></div>`;
        if (!connected) portsHtml += `<div class="n8n-port-stub" style="top:calc(${topPct}% - 1px);"></div>`;
        portsHtml += `<button class="n8n-port-add ${connected?'':'unconnected'}" style="top:calc(${topPct}% - 9px);" onclick="event.stopPropagation();openCreatorDrawer('${esc(n.name)}',${b})" title="Node anhängen">+</button>`;
      }
    } else {
      portsHtml += `<div class="n8n-port-out" style="top:-6px;right:calc(50% - 5px);" data-out-node="${esc(n.name)}" data-out-branch="0"></div>`;
    }

    const diamondHtml = root ? `<div class="n8n-diamond-ports" onclick="event.stopPropagation()">
      <div class="n8n-diamond" onclick="openAgentSubCreator('${esc(n.name)}','ai_languageModel')" title="Chat-Modell verknüpfen"><div class="n8n-diamond-sq"></div><span>Chat-Modell*</span></div>
      <div class="n8n-diamond" onclick="openAgentSubCreator('${esc(n.name)}','ai_memory')" title="Speicher verknüpfen"><div class="n8n-diamond-sq"></div><span>Speicher</span></div>
      <div class="n8n-diamond" onclick="openAgentSubCreator('${esc(n.name)}','ai_tool')" title="Werkzeug verknüpfen"><div class="n8n-diamond-sq"></div><span>Werkzeug</span></div>
    </div>` : '';

    const innerContent = root
      ? `${m.svg}<div style="min-width:0;"><div class="n8n-node-title">${esc(n.name)}</div><div class="n8n-node-sub">${esc(m.label)}</div></div>`
      : m.svg;

    const itemCount = stat?.outputCount ?? (Array.isArray(stat?.items) ? stat.items.length : 1);
    const pillTxt = stat ? (stat.status === 'error' ? 'Fehler' : `${itemCount} ${itemCount === 1 ? 'Element' : 'Elemente'}`) : '';

    html += `<div class="n8n-node-wrap ${wrapCls}" data-nodename="${esc(n.name)}" style="left:${x}px;top:${y}px;">
      <div class="n8n-node-toolbar">
        <button class="n8n-tb-btn" title="Schritt ausführen" onclick="event.stopPropagation();quickExecNode('${esc(n.name)}')">▶</button>
        <button class="n8n-tb-btn" title="${n.disabled?'Aktivieren':'Deaktivieren'}" onclick="event.stopPropagation();toggleDisableNode('${esc(n.name)}')">⏻</button>
        <button class="n8n-tb-btn" title="Duplizieren (⌘D)" onclick="event.stopPropagation();duplicateNode('${esc(n.name)}')">⧉</button>
        <button class="n8n-tb-btn del" title="Löschen (Entf)" onclick="event.stopPropagation();deleteNode('${esc(n.name)}')">🗑</button>
        <button class="n8n-tb-btn" title="Weitere Aktionen" onclick="event.stopPropagation();openNodeContextMenu(event,'${esc(n.name)}')">⋯</button>
      </div>
      <div class="n8n-node-box ${boxCls} ${statCls} ${selCls} ${disCls} ${pinCls}" ondblclick="event.stopPropagation();openNdv('${esc(n.name)}')" onclick="onNodeClick(event,'${esc(n.name)}')" oncontextmenu="event.preventDefault();event.stopPropagation();openNodeContextMenu(event,'${esc(n.name)}')">
        ${trig ? '<div class="n8n-trig-bolt">⚡</div>' : ''}
        ${isPinned ? '<div class="n8n-pin-pill">📌 Angeheftet</div>' : ''}
        ${n.disabled ? '<div class="n8n-disabled-strike"></div>' : ''}
        ${innerContent}
        ${stat ? `<div class="n8n-exec-badge ${stat.status==='error'?'err':''}" title="${esc(stat.error||'')}">${stat.status==='error'?'!':'✓'}</div><div class="n8n-items-pill ${stat.status==='error'?'err':''}">${pillTxt}</div>` : ''}
        ${portsHtml}
        ${diamondHtml}
      </div>
      <div class="n8n-node-label">
        <div class="n8n-node-title">${esc(n.name)}</div>
        ${n.disabled ? '<div class="n8n-node-sub">(Deaktiviert)</div>' : `<div class="n8n-node-sub">${esc(m.label)}</div>`}
      </div>
    </div>`;
  }
  layer.innerHTML = html;
  if (st.editingStickyName) {
    const ta = document.getElementById('stickyEd-' + st.editingStickyName);
    if (ta) { ta.focus(); ta.setSelectionRange(ta.value.length, ta.value.length); }
  }
  renderWires(nodeMap, conns);
  drawMinimap();
}

function setStickyColor(name, colorId) {
  const n = st.currentWf?.nodes?.find(x => x.name === name); if (!n) return;
  pushHistory();
  n.parameters = n.parameters || {}; n.parameters.color = colorId;
  st.dirty = true; updateHeaderState(); renderCanvas();
}
function saveStickyText(name, content) {
  const n = st.currentWf?.nodes?.find(x => x.name === name);
  if (n && (n.parameters?.content || '') !== content) {
    pushHistory();
    n.parameters = n.parameters || {}; n.parameters.content = content;
    st.dirty = true; updateHeaderState();
  }
  st.editingStickyName = null;
  renderCanvas();
}
function addStickyNoteAtCenter() {
  if (!st.currentWf) return;
  pushHistory();
  const cw = document.getElementById('n8nCanvas')?.clientWidth || 900;
  const ch = document.getElementById('n8nCanvas')?.clientHeight || 600;
  const x = Math.round(((cw / 2) - st.panX) / st.zoom - 120);
  const y = Math.round(((ch / 2) - st.panY) / st.zoom - 80);
  let name = 'Haftnotiz', idx = 1;
  while ((st.currentWf.nodes || []).some(n => n.name === name)) name = `Haftnotiz ${idx++}`;
  st.currentWf.nodes.push({
    id: 'sticky-' + Date.now(), name, type: 'n8n-nodes-base.stickyNote', position: [x, y],
    parameters: { content: '## Notiz\nDoppelklick zum Bearbeiten', width: 240, height: 160, color: 1 }
  });
  st.dirty = true; updateHeaderState(); renderCanvas();
}

let edgeHoverTimer = null;
function showEdgeToolbar(edgeIdx) {
  clearTimeout(edgeHoverTimer);
  document.querySelectorAll('.n8n-edge-tb').forEach(el => el.classList.remove('visible'));
  document.getElementById('edgeTb-' + edgeIdx)?.classList.add('visible');
}
function hideEdgeToolbar(edgeIdx) {
  edgeHoverTimer = setTimeout(() => {
    document.getElementById('edgeTb-' + edgeIdx)?.classList.remove('visible');
  }, 180);
}

function renderWires(nodeMap, conns) {
  let paths = `<defs>
    <marker id="n8nArrow" markerWidth="7" markerHeight="7" refX="6" refY="3.5" orient="auto"><path d="M0,0.5 L6,3.5 L0,6.5 z" fill="var(--n8n-wire)"/></marker>
    <marker id="n8nArrowOk" markerWidth="7" markerHeight="7" refX="6" refY="3.5" orient="auto"><path d="M0,0.5 L6,3.5 L0,6.5 z" fill="var(--n8n-green)"/></marker>
  </defs>`;
  let tbHtml = '';
  let edgeIdx = 0;

  for (const [srcName, edges] of Object.entries(conns || {})) {
    const src = nodeMap[srcName]; if (!src) continue;
    const sd = nodeDimensions(src);
    const sx = (src.position?.[0] ?? 0), sy = (src.position?.[1] ?? 0);

    for (const [portType, branches] of Object.entries(edges || {})) {
      if (!Array.isArray(branches)) continue;
      if (portType === 'main') {
        const totalB = outputPortCount(src, conns);
        branches.forEach((targets, bIdx) => {
          if (!Array.isArray(targets)) return;
          const yFrac = totalB === 1 ? 0.5 : (totalB === 2 ? (bIdx === 0 ? 0.32 : 0.68) : (0.18 + 0.64 * bIdx / (totalB - 1)));
          const x1 = sx + sd.w, y1 = sy + sd.h * yFrac;
          targets.forEach(t => {
            const dst = nodeMap[t.node]; if (!dst) return;
            const dd = nodeDimensions(dst);
            const x2 = (dst.position?.[0] ?? 0), y2 = (dst.position?.[1] ?? 0) + dd.h * 0.5;
            const dx = Math.max(58, Math.abs(x2 - x1) * 0.45);
            const mx = (x1 + x2) / 2, my = (y1 + y2) / 2;
            const isExec = !!(st.nodeStatusMap[srcName] && st.nodeStatusMap[t.node]);
            const isPin = !!(st.currentWf?.pinData?.[srcName]?.length);
            const cls = isPin ? 'pinned' : (isExec ? 'executed' : '');
            const marker = isExec ? 'url(#n8nArrowOk)' : 'url(#n8nArrow)';
            const idx = edgeIdx++;
            paths += `<g class="wire-group" onmouseenter="showEdgeToolbar(${idx})" onmouseleave="hideEdgeToolbar(${idx})">
              <path class="wire-hit" d="M ${x1} ${y1} C ${x1+dx} ${y1}, ${x2-dx} ${y2}, ${x2} ${y2}"/>
              <path class="wire-path ${cls}" marker-end="${marker}" d="M ${x1} ${y1} C ${x1+dx} ${y1}, ${x2-dx} ${y2}, ${x2} ${y2}"/>
            </g>`;
            tbHtml += `<div class="n8n-edge-tb" id="edgeTb-${idx}" style="left:${mx}px;top:${my}px;" onmouseenter="showEdgeToolbar(${idx})" onmouseleave="hideEdgeToolbar(${idx})">
              <button class="n8n-edge-btn" onclick="event.stopPropagation();openInsertOnEdge('${esc(srcName)}',${bIdx},'${esc(t.node)}',${t.index||0})" title="Node dazwischen einfügen">+</button>
              <button class="n8n-edge-btn del" onclick="event.stopPropagation();deleteEdge('${esc(srcName)}','main',${bIdx},'${esc(t.node)}')" title="Verbindung entfernen">🗑</button>
            </div>`;
          });
        });
      } else {
        branches.forEach((targets, bIdx) => {
          if (!Array.isArray(targets)) return;
          targets.forEach(t => {
            const dst = nodeMap[t.node]; if (!dst) return;
            const dd = nodeDimensions(dst);
            const x1 = sx + sd.w / 2, y1 = sy;
            const offset = portType.includes('languageModel') ? 0.22 : portType.includes('memory') ? 0.5 : 0.78;
            const x2 = (dst.position?.[0] ?? 0) + dd.w * offset, y2 = (dst.position?.[1] ?? 0) + dd.h + 10;
            const dy = Math.max(40, Math.abs(y1 - y2) * 0.45);
            const mx = (x1 + x2) / 2, my = (y1 + y2) / 2;
            const idx = edgeIdx++;
            paths += `<g class="wire-group" onmouseenter="showEdgeToolbar(${idx})" onmouseleave="hideEdgeToolbar(${idx})">
              <path class="wire-hit" d="M ${x1} ${y1} C ${x1} ${y1-dy}, ${x2} ${y2+dy}, ${x2} ${y2}"/>
              <path class="wire-path ai" d="M ${x1} ${y1} C ${x1} ${y1-dy}, ${x2} ${y2+dy}, ${x2} ${y2}"/>
            </g>`;
            tbHtml += `<div class="n8n-edge-tb" id="edgeTb-${idx}" style="left:${mx}px;top:${my}px;" onmouseenter="showEdgeToolbar(${idx})" onmouseleave="hideEdgeToolbar(${idx})">
              <button class="n8n-edge-btn del" onclick="event.stopPropagation();deleteEdge('${esc(srcName)}','${esc(portType)}',${bIdx},'${esc(t.node)}')" title="Verbindung entfernen">🗑</button>
            </div>`;
          });
        });
      }
    }
  }
  paths += `<path class="wire-path draft" id="draftWirePath" d="" style="display:none;"/>`;
  document.getElementById('wireSvg').innerHTML = paths;
  document.getElementById('edgeToolbarLayer').innerHTML = tbHtml;
}

function deleteEdge(srcName, portType, branchIdx, dstName) {
  if (!st.currentWf?.connections?.[srcName]?.[portType]?.[branchIdx]) return;
  pushHistory();
  const arr = st.currentWf.connections[srcName][portType][branchIdx];
  if (Array.isArray(arr)) st.currentWf.connections[srcName][portType][branchIdx] = arr.filter(t => t.node !== dstName);
  st.dirty = true; updateHeaderState(); renderCanvas(); toast('Verbindung entfernt');
}

function openInsertOnEdge(srcName, branchIdx, dstName, dstPort) {
  st.creatorMode = 'insertEdge';
  st.creatorInsertEdge = { srcName, branchIdx, dstName, dstPort };
  st.creatorSourceNode = null;
  document.getElementById('creatorTitle').innerHTML = `Node einfügen <span class="tag-pill mono" id="creatorCountBadge" style="font-size:11px;margin-left:4px;">${st.nodeCatalog.length}</span>`;
  document.getElementById('creatorDrawer').classList.add('open');
  document.getElementById('creatorSearch').value = '';
  renderCreatorList();
  setTimeout(() => document.getElementById('creatorSearch').focus(), 40);
}

function openAgentSubCreator(agentName, slotType) {
  st.creatorMode = 'agentSub';
  st.creatorAgentSlot = { agentName, slotType };
  st.creatorCat = slotType === 'ai_tool' ? 'KI-Werkzeuge' : 'KI-Sub-Node';
  document.getElementById('creatorTitle').innerHTML = `Sub-Node verknüpfen <span class="tag-pill mono" id="creatorCountBadge" style="font-size:11px;margin-left:4px;">0</span>`;
  document.getElementById('creatorDrawer').classList.add('open');
  document.getElementById('creatorSearch').value = '';
  renderCreatorList();
  setTimeout(() => document.getElementById('creatorSearch').focus(), 40);
}

function connectNodes(srcName, branchIdx, dstName) {
  if (!st.currentWf || !srcName || !dstName || srcName === dstName) return false;
  pushHistory();
  const srcNode = (st.currentWf.nodes || []).find(n => n.name === srcName);
  const dstNode = (st.currentWf.nodes || []).find(n => n.name === dstName);
  const conns = st.currentWf.connections = st.currentWf.connections || {};
  if (srcNode && isSubnode(srcNode.type) && dstNode && isAgentRoot(dstNode.type)) {
    const port = srcNode.type.includes('lmChat') ? 'ai_languageModel' : (srcNode.type.includes('memory') ? 'ai_memory' : 'ai_tool');
    conns[srcName] = conns[srcName] || {};
    conns[srcName][port] = [[{ node: dstName, type: port, index: 0 }]];
  } else {
    conns[srcName] = conns[srcName] || { main: [] };
    conns[srcName].main = Array.isArray(conns[srcName].main) ? conns[srcName].main : [];
    while (conns[srcName].main.length <= branchIdx) conns[srcName].main.push([]);
    const bucket = conns[srcName].main[branchIdx];
    if (!bucket.some(t => t.node === dstName)) bucket.push({ node: dstName, type: 'main', index: 0 });
  }
  st.dirty = true; updateHeaderState(); renderCanvas(); toast(`Verbunden: ${srcName} → ${dstName}`);
  return true;
}

function onNodeClick(e, name) {
  if (e && e.shiftKey) {
    if (st.selectedNodes.has(name)) st.selectedNodes.delete(name);
    else st.selectedNodes.add(name);
  } else {
    st.selectedNodes = new Set([name]);
  }
  renderCanvas();
}

function openNodeContextMenu(e, name) {
  const n = st.currentWf?.nodes?.find(x => x.name === name); if (!n) return;
  st.selectedNodes = new Set([name]);
  const menu = document.getElementById('ctxMenu');
  menu.innerHTML = `
    <button class="n8n-pop-item" onclick="closeCtxMenu();openNdv('${esc(name)}')"><span>⤢ Öffnen (NDV)</span></button>
    <button class="n8n-pop-item" onclick="closeCtxMenu();quickExecNode('${esc(name)}')"><span>▶ Schritt testen</span></button>
    <button class="n8n-pop-item" onclick="closeCtxMenu();promptRenameNode('${esc(name)}')"><span>✎ Umbenennen</span></button>
    <button class="n8n-pop-item" onclick="closeCtxMenu();copySelectedNodes()"><span>⧉ Kopieren (⌘C)</span></button>
    <button class="n8n-pop-item" onclick="closeCtxMenu();duplicateNode('${esc(name)}')"><span>❐ Duplizieren (⌘D)</span></button>
    <button class="n8n-pop-item" onclick="closeCtxMenu();toggleDisableNode('${esc(name)}')"><span>⏻ ${n.disabled?'Aktivieren':'Deaktivieren'}</span></button>
    <div style="border-top:1px solid var(--n8n-border);margin:4px 0;"></div>
    <button class="n8n-pop-item" style="color:var(--n8n-red);" onclick="closeCtxMenu();deleteNode('${esc(name)}')"><span>🗑 Löschen (Entf)</span></button>
  `;
  menu.style.left = Math.min(window.innerWidth - 220, e.clientX) + 'px';
  menu.style.top = Math.min(window.innerHeight - 260, e.clientY) + 'px';
  menu.classList.add('open');
}

function promptRenameNode(oldName) {
  const nn = prompt('Neuer Node-Name:', oldName);
  if (!nn || nn.trim() === oldName) return;
  st.ndvNodeName = oldName;
  renameNdvNode(nn);
}

// Mouse: Dragging Nodes (single & multi), Sticky Resize, Wire Dragging, Panning & Marquee Selection
(function setupCanvasMouse() {
  const cvs = document.getElementById('n8nCanvas');
  let dragGroup = null, dragWire = null, resizeSticky = null, panning = false, marquee = null;
  let startX = 0, startY = 0, origPanX = 0, origPanY = 0, moved = false;

  function clientToWorld(cx, cy) {
    const vp = document.getElementById('n8nViewport')?.getBoundingClientRect();
    if (!vp) return { x: cx, y: cy };
    return { x: (cx - vp.left) / st.zoom, y: (cy - vp.top) / st.zoom };
  }

  cvs.addEventListener('contextmenu', e => {
    if (e.target.closest('.n8n-node-wrap,.n8n-sticky')) return;
    e.preventDefault();
    const menu = document.getElementById('ctxMenu');
    menu.innerHTML = `
      <button class="n8n-pop-item" onclick="closeCtxMenu();openCreatorDrawer(null)"><span>+ Node hinzufügen</span></button>
      <button class="n8n-pop-item" onclick="closeCtxMenu();addStickyNoteAtCenter()"><span>📝 Haftnotiz hinzufügen</span></button>
      <button class="n8n-pop-item" onclick="closeCtxMenu();selectAllNodes()"><span>⬚ Alle auswählen (⌘A)</span></button>
      <button class="n8n-pop-item" onclick="closeCtxMenu();pasteFromInternalClipboard()"><span>📋 Einfügen (⌘V)</span></button>
      <button class="n8n-pop-item" onclick="closeCtxMenu();tidyUpLayout()"><span>🪄 Layout aufräumen</span></button>
      <button class="n8n-pop-item" onclick="closeCtxMenu();fitToView()"><span>⊡ An Ansicht anpassen</span></button>
    `;
    menu.style.left = Math.min(window.innerWidth - 220, e.clientX) + 'px';
    menu.style.top = Math.min(window.innerHeight - 220, e.clientY) + 'px';
    menu.classList.add('open');
  });

  cvs.addEventListener('mousedown', e => {
    if (e.button !== 0) return;
    closeCtxMenu();
    const resizer = e.target.closest('[data-resize-sticky]');
    if (resizer) {
      e.stopPropagation();
      const nm = resizer.dataset.resizeSticky;
      const node = (st.currentWf?.nodes || []).find(n => n.name === nm);
      if (node) {
        pushHistory();
        startX = e.clientX; startY = e.clientY;
        resizeSticky = { node, origW: node.parameters?.width || 240, origH: node.parameters?.height || 160 };
      }
      return;
    }
    const portOut = e.target.closest('.n8n-port-out[data-out-node]');
    if (portOut) {
      e.stopPropagation();
      const pr = portOut.getBoundingClientRect();
      const p1 = clientToWorld(pr.left + pr.width / 2, pr.top + pr.height / 2);
      startX = e.clientX; startY = e.clientY; moved = false;
      dragWire = { srcName: portOut.dataset.outNode, branch: parseInt(portOut.dataset.outBranch || '0', 10) || 0, x1: p1.x, y1: p1.y };
      return;
    }
    if (e.target.closest('button,.n8n-port-out,.n8n-port-add,.wire-hit,.n8n-edge-tb,.n8n-sticky-ed,.n8n-diamond')) return;
    const nodeEl = e.target.closest('.n8n-node-wrap, .n8n-sticky');
    startX = e.clientX; startY = e.clientY; moved = false;
    if (nodeEl) {
      const nm = nodeEl.dataset.nodename;
      if (!st.selectedNodes.has(nm) && !e.shiftKey) st.selectedNodes = new Set([nm]);
      else if (e.shiftKey) st.selectedNodes.add(nm);
      const targets = (st.currentWf?.nodes || []).filter(n => st.selectedNodes.has(n.name));
      if (targets.length) {
        dragGroup = targets.map(n => ({ node: n, ox: n.position?.[0] ?? 0, oy: n.position?.[1] ?? 0 }));
      }
    } else if (e.shiftKey || st.selectMode) {
      const rect = cvs.getBoundingClientRect();
      marquee = { sx: e.clientX - rect.left, sy: e.clientY - rect.top };
      const mb = document.getElementById('marqueeBox');
      mb.style.display = 'block';
      mb.style.left = marquee.sx + 'px'; mb.style.top = marquee.sy + 'px';
      mb.style.width = '0px'; mb.style.height = '0px';
    } else {
      if (!e.shiftKey && st.selectedNodes.size) { st.selectedNodes.clear(); document.querySelectorAll('.n8n-node-box.selected,.n8n-sticky.selected').forEach(el => el.classList.remove('selected')); }
      panning = true; origPanX = st.panX; origPanY = st.panY;
      cvs.classList.add('panning');
    }
  });

  window.addEventListener('mousemove', e => {
    if (resizeSticky) {
      const dx = (e.clientX - startX) / st.zoom, dy = (e.clientY - startY) / st.zoom;
      resizeSticky.node.parameters = resizeSticky.node.parameters || {};
      resizeSticky.node.parameters.width = Math.max(150, Math.round(resizeSticky.origW + dx));
      resizeSticky.node.parameters.height = Math.max(80, Math.round(resizeSticky.origH + dy));
      const el = document.querySelector(`.n8n-sticky[data-nodename="${CSS.escape(resizeSticky.node.name)}"]`);
      if (el) { el.style.width = resizeSticky.node.parameters.width + 'px'; el.style.height = resizeSticky.node.parameters.height + 'px'; }
      st.dirty = true;
    } else if (dragWire) {
      if (Math.hypot(e.clientX - startX, e.clientY - startY) > 5) moved = true;
      const draft = document.getElementById('draftWirePath');
      if (draft && moved) {
        const p2 = clientToWorld(e.clientX, e.clientY);
        const dx = Math.max(55, Math.abs(p2.x - dragWire.x1) * 0.45);
        draft.style.display = 'block';
        draft.setAttribute('d', `M ${dragWire.x1} ${dragWire.y1} C ${dragWire.x1 + dx} ${dragWire.y1}, ${p2.x - dx} ${p2.y}, ${p2.x} ${p2.y}`);
      }
      document.querySelectorAll('.n8n-port-in.snap').forEach(p => p.classList.remove('snap'));
      const hoverEl = document.elementFromPoint(e.clientX, e.clientY);
      const wrap = hoverEl?.closest('.n8n-node-wrap[data-nodename]');
      if (wrap && wrap.dataset.nodename !== dragWire.srcName) wrap.querySelector('.n8n-port-in')?.classList.add('snap');
    } else if (dragGroup) {
      const dx = (e.clientX - startX) / st.zoom, dy = (e.clientY - startY) / st.zoom;
      if (!moved && (Math.abs(dx) > 2 || Math.abs(dy) > 2)) { moved = true; pushHistory(); }
      if (moved) {
        dragGroup.forEach(item => {
          item.node.position = [Math.round(item.ox + dx), Math.round(item.oy + dy)];
          const wrap = document.querySelector(`[data-nodename="${CSS.escape(item.node.name)}"]`);
          if (wrap) { wrap.style.left = item.node.position[0] + 'px'; wrap.style.top = item.node.position[1] + 'px'; }
        });
        st.dirty = true;
        const nodeMap = Object.fromEntries((st.currentWf?.nodes || []).map(n => [n.name, n]));
        renderWires(nodeMap, st.currentWf?.connections || {});
      }
    } else if (marquee) {
      const rect = cvs.getBoundingClientRect();
      const cx = e.clientX - rect.left, cy = e.clientY - rect.top;
      const mb = document.getElementById('marqueeBox');
      const x = Math.min(marquee.sx, cx), y = Math.min(marquee.sy, cy);
      const w = Math.abs(cx - marquee.sx), h = Math.abs(cy - marquee.sy);
      mb.style.left = x + 'px'; mb.style.top = y + 'px'; mb.style.width = w + 'px'; mb.style.height = h + 'px';
      marquee.box = { x, y, w, h };
    } else if (panning) {
      st.panX = origPanX + (e.clientX - startX);
      st.panY = origPanY + (e.clientY - startY);
      applyCanvasTransform();
    }
  });

  window.addEventListener('mouseup', e => {
    if (resizeSticky) { resizeSticky = null; updateHeaderState(); drawMinimap(); return; }
    if (marquee) {
      const mb = document.getElementById('marqueeBox');
      mb.style.display = 'none';
      const b = marquee.box; marquee = null;
      if (b && (b.w > 6 || b.h > 6)) {
        const wx1 = (b.x - st.panX) / st.zoom, wy1 = (b.y - st.panY) / st.zoom;
        const wx2 = (b.x + b.w - st.panX) / st.zoom, wy2 = (b.y + b.h - st.panY) / st.zoom;
        st.selectedNodes = new Set();
        (st.currentWf?.nodes || []).forEach(n => {
          const nx = n.position?.[0] ?? 0, ny = n.position?.[1] ?? 0, d = nodeDimensions(n);
          if (nx + d.w >= wx1 && nx <= wx2 && ny + d.h >= wy1 && ny <= wy2) st.selectedNodes.add(n.name);
        });
        renderCanvas();
      }
      return;
    }
    if (dragWire) {
      document.querySelectorAll('.n8n-port-in.snap').forEach(p => p.classList.remove('snap'));
      const draft = document.getElementById('draftWirePath');
      if (draft) { draft.style.display = 'none'; draft.setAttribute('d', ''); }
      const dw = dragWire; dragWire = null;
      if (!moved) openCreatorDrawer(dw.srcName, dw.branch);
      else {
        const dropEl = document.elementFromPoint(e.clientX, e.clientY);
        const portIn = dropEl?.closest('.n8n-port-in[data-in-node]');
        const wrap = dropEl?.closest('.n8n-node-wrap[data-nodename]');
        const dstName = portIn?.dataset?.inNode || wrap?.dataset?.nodename;
        if (dstName && dstName !== dw.srcName) connectNodes(dw.srcName, dw.branch, dstName);
      }
      return;
    }
    if (dragGroup && moved) { updateHeaderState(); drawMinimap(); }
    dragGroup = null;
    if (panning) { panning = false; cvs.classList.remove('panning'); }
  });

  cvs.addEventListener('wheel', e => {
    e.preventDefault();
    zoomBy(e.deltaY < 0 ? 0.06 : -0.06, e.clientX, e.clientY);
  }, { passive: false });

  document.getElementById('n8nMinimap')?.addEventListener('click', e => {
    const meta = st._minimapMeta; if (!meta) return;
    const r = e.currentTarget.getBoundingClientRect();
    const wx = (e.clientX - r.left - meta.ox) / meta.sc;
    const wy = (e.clientY - r.top - meta.oy) / meta.sc;
    const cw = cvs.clientWidth || 1000, ch = cvs.clientHeight || 600;
    st.panX = cw / 2 - wx * st.zoom;
    st.panY = ch / 2 - wy * st.zoom;
    applyCanvasTransform();
  });
})();

// Resizable Bottom Logs
(function setupLogsResizer() {
  const handle = document.getElementById('logsResizer');
  const panel = document.getElementById('bottomLogs');
  if (!handle || !panel) return;
  let dragging = false, sy = 0, sh = 260;
  handle.addEventListener('mousedown', e => { dragging = true; sy = e.clientY; sh = panel.offsetHeight || 260; e.preventDefault(); });
  window.addEventListener('mousemove', e => {
    if (!dragging) return;
    const nh = Math.max(140, Math.min(520, sh + (sy - e.clientY)));
    panel.style.height = nh + 'px';
  });
  window.addEventListener('mouseup', () => { dragging = false; });
})();

// Node Creator Drawer
function openCreatorDrawer(srcName, branch = 0) {
  st.creatorMode = 'add';
  st.creatorSourceNode = srcName;
  st.creatorSourceBranch = branch;
  st.creatorInsertEdge = null;
  st.creatorAgentSlot = null;
  document.getElementById('creatorTitle').innerHTML = `Was passiert als Nächstes? <span class="tag-pill mono" id="creatorCountBadge" style="font-size:11px;margin-left:4px;">${st.nodeCatalog.length}</span>`;
  document.getElementById('creatorDrawer').classList.add('open');
  document.getElementById('creatorSearch').value = '';
  renderCreatorList();
  setTimeout(() => document.getElementById('creatorSearch').focus(), 40);
}
function closeCreatorDrawer() { document.getElementById('creatorDrawer').classList.remove('open'); }
function renderCreatorList() {
  const q = (document.getElementById('creatorSearch').value || '').toLowerCase();
  const byType = new Map();
  for (const sc of (st.nodeCatalog || [])) {
    byType.set(sc.name, {
      tp: sc.name,
      label: sc.displayName || sc.name,
      cat: sc.category || 'Kern',
      desc: sc.description || '',
      usableAsTool: !!sc.usableAsTool || sc.name.endsWith('Tool') || sc.name.includes('tool'),
      isSub: Array.isArray(sc.inputs) && sc.inputs.length === 0 && !(sc.group || []).includes('trigger') && sc.name.includes('langchain')
    });
  }
  for (const [tp, m] of Object.entries(NODE_META)) {
    if (!byType.has(tp)) byType.set(tp, { tp, label: m.label, cat: m.cat, desc: '', usableAsTool: false, isSub: !!m.subnode });
  }
  const all = Array.from(byType.values());
  const baseCats = Array.from(new Set(all.map(x => x.cat)));
  const cats = ['Alle', ...baseCats, 'KI-Sub-Node', 'KI-Werkzeuge'];
  if (!cats.includes(st.creatorCat)) st.creatorCat = 'Alle';

  const catBar = document.getElementById('creatorCatBar');
  if (catBar) {
    catBar.innerHTML = cats.map(c => {
      const act = st.creatorCat === c;
      return `<button class="n8n-btn" style="height:24px;padding:0 8px;font-size:11px;${act ? 'border-color:var(--n8n-coral);background:var(--n8n-coral-soft);color:var(--n8n-coral);' : ''}" onclick="st.creatorCat='${esc(c)}';renderCreatorList()">${esc(c)}</button>`;
    }).join('');
  }

  const filtered = all.filter(it => {
    const catOk = st.creatorCat === 'Alle'
      || (st.creatorCat === 'KI-Werkzeuge' ? it.usableAsTool : (st.creatorCat === 'KI-Sub-Node' ? it.isSub : it.cat === st.creatorCat));
    const qOk = !q || it.label.toLowerCase().includes(q) || it.tp.toLowerCase().includes(q) || it.desc.toLowerCase().includes(q) || it.cat.toLowerCase().includes(q);
    return catOk && qOk;
  });

  const badge = document.getElementById('creatorCountBadge');
  if (badge) badge.textContent = String(filtered.length);

  document.getElementById('creatorList').innerHTML = filtered.map(it => {
    const m = metaFor(it.tp);
    return `
    <button class="n8n-creator-item" onclick="addNodeFromCreator('${esc(it.tp)}')">
      <span style="width:30px;height:30px;flex-shrink:0;display:inline-flex;align-items:center;justify-content:center;">${m.svg}</span>
      <div style="min-width:0;flex:1;">
        <div style="display:flex;align-items:center;gap:6px;">
          <span style="font-weight:600;font-size:13px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">${esc(it.label)}</span>
          <span style="font-size:10px;color:var(--n8n-text-3);margin-left:auto;flex-shrink:0;">${esc(it.cat)}</span>
        </div>
        ${it.desc ? `<div style="font-size:11px;color:var(--n8n-text-3);white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">${esc(it.desc)}</div>` : ''}
      </div>
    </button>`;
  }).join('') || '<div style="padding:18px;color:var(--n8n-text-3);font-size:12.5px;text-align:center;">Keine passenden Nodes gefunden.</div>';
}

function addNodeFromCreator(tp) {
  if (!st.currentWf) return;
  pushHistory();
  const m = metaFor(tp);
  let name = m.label, idx = 1;
  while ((st.currentWf.nodes || []).some(n => n.name === name)) name = `${m.label} ${idx++}`;

  let x = 420, y = 240;
  const conns = st.currentWf.connections = st.currentWf.connections || {};

  if (st.creatorMode === 'insertEdge' && st.creatorInsertEdge) {
    const { srcName, branchIdx, dstName, dstPort } = st.creatorInsertEdge;
    const src = st.currentWf.nodes.find(n => n.name === srcName);
    const dst = st.currentWf.nodes.find(n => n.name === dstName);
    if (src && dst) {
      x = Math.round(((src.position?.[0] ?? 200) + (dst.position?.[0] ?? 500)) / 2);
      y = Math.round(((src.position?.[1] ?? 200) + (dst.position?.[1] ?? 200)) / 2);
      if (Math.abs((dst.position?.[0] ?? 0) - (src.position?.[0] ?? 0)) < 260) {
        dst.position[0] = (dst.position[0] ?? 0) + 180;
        x = Math.round(((src.position?.[0] ?? 200) + dst.position[0]) / 2);
      }
    }
    const bucket = conns[srcName]?.main?.[branchIdx];
    if (Array.isArray(bucket)) {
      conns[srcName].main[branchIdx] = bucket.map(t => t.node === dstName ? { node: name, type: 'main', index: 0 } : t);
    }
    conns[name] = { main: [[{ node: dstName, type: 'main', index: dstPort || 0 }]] };
  } else if (st.creatorMode === 'agentSub' && st.creatorAgentSlot) {
    const { agentName, slotType } = st.creatorAgentSlot;
    const ag = st.currentWf.nodes.find(n => n.name === agentName);
    if (ag) {
      const off = slotType === 'ai_languageModel' ? 10 : slotType === 'ai_memory' ? 95 : 180;
      x = (ag.position?.[0] ?? 300) + off;
      y = (ag.position?.[1] ?? 200) + 160;
    }
    conns[name] = { [slotType]: [[{ node: agentName, type: slotType, index: 0 }]] };
  } else if (st.creatorSourceNode) {
    const src = st.currentWf.nodes.find(n => n.name === st.creatorSourceNode);
    if (src) { x = (src.position?.[0] ?? 200) + 240; y = (src.position?.[1] ?? 200) + (st.creatorSourceBranch * 120); }
    conns[st.creatorSourceNode] = conns[st.creatorSourceNode] || { main: [] };
    while (conns[st.creatorSourceNode].main.length <= st.creatorSourceBranch) conns[st.creatorSourceNode].main.push([]);
    conns[st.creatorSourceNode].main[st.creatorSourceBranch].push({ node: name, type: 'main', index: 0 });
  }

  const defParams = tp === 'n8n-nodes-base.stickyNote' ? { content: '## Notiz\nDoppelklick zum Bearbeiten', width: 240, height: 160, color: 1 } : {};
  st.currentWf.nodes.push({ id: 'n-' + Date.now(), name, type: tp, position: [x, y], parameters: defParams });
  st.selectedNodes = new Set([name]);
  st.dirty = true; updateHeaderState(); closeCreatorDrawer(); renderCanvas(); toast(`${name} hinzugefügt`);
}

function toggleDisableNode(name) {
  const n = st.currentWf?.nodes?.find(x => x.name === name); if (!n) return;
  pushHistory();
  n.disabled = !n.disabled; st.dirty = true; updateHeaderState(); renderCanvas();
}
function duplicateNode(name) {
  const n = st.currentWf?.nodes?.find(x => x.name === name); if (!n) return;
  pushHistory();
  let newName = n.name + ' Kopie', idx = 2;
  while ((st.currentWf.nodes || []).some(x => x.name === newName)) newName = `${n.name} Kopie ${idx++}`;
  const clone = JSON.parse(JSON.stringify(n));
  clone.id = 'n-' + Date.now();
  clone.name = newName;
  clone.position = [(n.position?.[0] ?? 200) + 45, (n.position?.[1] ?? 200) + 45];
  st.currentWf.nodes.push(clone);
  st.selectedNodes = new Set([newName]);
  st.dirty = true; updateHeaderState(); renderCanvas(); toast(`${newName} dupliziert`);
}
function deleteNode(name) {
  deleteNodesList([name]);
}
function deleteNodesList(names) {
  if (!st.currentWf || !names.length) return;
  pushHistory();
  const set = new Set(names);
  st.currentWf.nodes = st.currentWf.nodes.filter(n => !set.has(n.name));
  for (const nm of names) {
    delete st.currentWf.connections?.[nm];
    if (st.currentWf.pinData) delete st.currentWf.pinData[nm];
    st.selectedNodes.delete(nm);
  }
  for (const edges of Object.values(st.currentWf.connections || {})) {
    for (const branches of Object.values(edges || {})) {
      if (Array.isArray(branches)) branches.forEach((arr, i) => { if (Array.isArray(arr)) branches[i] = arr.filter(t => !set.has(t.node)); });
    }
  }
  st.dirty = true; updateHeaderState(); renderCanvas(); toast(names.length > 1 ? `${names.length} Nodes entfernt` : 'Node entfernt');
}
function selectAllNodes() {
  st.selectedNodes = new Set((st.currentWf?.nodes || []).map(n => n.name));
  renderCanvas();
}
function copySelectedNodes() {
  if (!st.currentWf || !st.selectedNodes.size) return;
  const nodes = st.currentWf.nodes.filter(n => st.selectedNodes.has(n.name));
  const connections = {};
  for (const n of nodes) {
    if (st.currentWf.connections?.[n.name]) {
      const filtered = {};
      for (const [pt, br] of Object.entries(st.currentWf.connections[n.name])) {
        if (Array.isArray(br)) filtered[pt] = br.map(arr => (arr || []).filter(t => st.selectedNodes.has(t.node)));
      }
      connections[n.name] = filtered;
    }
  }
  const bundle = { nodes: JSON.parse(JSON.stringify(nodes)), connections };
  st.clipboard = bundle;
  navigator.clipboard?.writeText(JSON.stringify(bundle, null, 2)).catch(() => {});
  toast(`${nodes.length} Node(s) kopiert`);
}
function pasteWorkflowFragment(bundle) {
  if (!st.currentWf || !Array.isArray(bundle?.nodes) || !bundle.nodes.length) return false;
  pushHistory();
  const existing = new Set((st.currentWf.nodes || []).map(n => n.name));
  const renameMap = {};
  const added = [];
  for (const orig of bundle.nodes) {
    let nn = orig.name || 'Node';
    if (existing.has(nn)) {
      let i = 1;
      while (existing.has(`${nn} ${i}`)) i++;
      nn = `${nn} ${i}`;
    }
    existing.add(nn);
    renameMap[orig.name] = nn;
    const copy = JSON.parse(JSON.stringify(orig));
    copy.id = 'n-' + Date.now() + '-' + Math.random().toString(36).slice(2, 6);
    copy.name = nn;
    copy.position = [(orig.position?.[0] ?? 240) + 50, (orig.position?.[1] ?? 240) + 50];
    st.currentWf.nodes.push(copy);
    added.push(nn);
  }
  const conns = st.currentWf.connections = st.currentWf.connections || {};
  for (const [oldSrc, edges] of Object.entries(bundle.connections || {})) {
    const newSrc = renameMap[oldSrc];
    if (!newSrc) continue;
    conns[newSrc] = {};
    for (const [pt, br] of Object.entries(edges || {})) {
      if (Array.isArray(br)) {
        conns[newSrc][pt] = br.map(arr => (arr || []).filter(t => renameMap[t.node]).map(t => ({ ...t, node: renameMap[t.node] })));
      }
    }
  }
  st.selectedNodes = new Set(added);
  st.dirty = true; updateHeaderState(); renderCanvas();
  toast(`${added.length} Node(s) eingefügt`);
  return true;
}
async function pasteFromInternalClipboard() {
  try {
    const txt = await navigator.clipboard.readText();
    if (txt && txt.trim().startsWith('{')) {
      const parsed = JSON.parse(txt);
      if (pasteWorkflowFragment(parsed)) return;
    }
  } catch {}
  if (st.clipboard) pasteWorkflowFragment(st.clipboard);
}

// Execution & Bottom Logs Panel (n8n 2.x Split Style)
function togglePayloadPop() { document.getElementById('payloadModal').classList.add('open'); }
function toggleBottomLogs(forceOpen) {
  const bl = document.getElementById('bottomLogs');
  const stage = document.getElementById('canvasStage');
  if (forceOpen === true) bl.classList.add('open');
  else if (forceOpen === false) bl.classList.remove('open');
  else bl.classList.toggle('open');
  if (stage) stage.classList.toggle('logs-open', bl.classList.contains('open'));
}
function setLogsFmt(fmt, btn) {
  st.logsFmt = fmt;
  document.querySelectorAll('#logsFmtSeg .n8n-seg-btn').forEach(b => b.classList.remove('active'));
  btn.classList.add('active');
  showStepLog(st.selectedLogIdx);
}

function populateBottomLogs(d) {
  const steps = d.nodeLogs || d.steps || [];
  document.getElementById('logsHdrTxt').textContent = `Ausführung · ${d.durationMs || 0}ms · ${steps.length} Nodes`;
  const pill = document.getElementById('logsStatusPill');
  if (pill) {
    pill.style.display = 'inline-flex';
    pill.textContent = d.status === 'success' ? '✓ Erfolgreich' : '✕ Fehler';
    pill.style.color = d.status === 'success' ? 'var(--n8n-green)' : 'var(--n8n-red)';
  }
  document.getElementById('logsStepList').innerHTML = steps.map((l, idx) => {
    const cnt = l.outputCount ?? (Array.isArray(l.items) ? l.items.length : 1);
    return `<div class="n8n-pop-item ${idx===st.selectedLogIdx?'active':''}" onclick="showStepLog(${idx})">
      <span style="overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">${l.status==='success'?'🟢':'🔴'} ${esc(l.node)}</span>
      <span class="mono" style="font-size:10.5px;color:var(--n8n-text-3);flex-shrink:0;">${cnt}x · ${l.durationMs}ms</span>
    </div>`;
  }).join('') || '<div style="padding:12px;color:var(--n8n-text-3);">Keine Schritte aufgezeichnet.</div>';
  document.getElementById('logsToggleBtn').textContent = `Protokoll (${steps.length})`;
  showStepLog(0);
}

function showStepLog(idx) {
  const steps = st.lastExec?.nodeLogs || st.lastExec?.steps || [];
  st.selectedLogIdx = Math.max(0, Math.min(idx, steps.length - 1));
  const l = steps[st.selectedLogIdx];
  document.querySelectorAll('#logsStepList .n8n-pop-item').forEach((el, i) => el.classList.toggle('active', i === st.selectedLogIdx));
  const inEl = document.getElementById('logsInBody');
  const outEl = document.getElementById('logsOutBody');
  if (!l) {
    inEl.innerHTML = '<div style="color:var(--n8n-text-3);">Noch kein Lauf in dieser Sitzung.</div>';
    outEl.innerHTML = '<div style="color:var(--n8n-text-3);">Noch kein Lauf in dieser Sitzung.</div>';
    return;
  }
  if (l.node) {
    st.selectedNodes = new Set([l.node]);
    document.querySelectorAll('.n8n-node-box').forEach(b => b.classList.remove('selected'));
    document.querySelector(`.n8n-node-wrap[data-nodename="${CSS.escape(l.node)}"] .n8n-node-box`)?.classList.add('selected');
  }
  const inData = l.input ?? st.lastExec?.input ?? null;
  const outData = l.error ? { error: l.error } : (l.items ?? l.output ?? null);
  inEl.innerHTML = renderDataFmt(inData, st.logsFmt, false);
  outEl.innerHTML = (l.error ? `<div style="padding:8px 10px;border-radius:6px;background:var(--n8n-red-soft);color:var(--n8n-red);margin-bottom:8px;font-size:12px;">✕ ${esc(l.error)}</div>` : '') + renderDataFmt(outData, st.logsFmt, false);
}

async function runCurrentWorkflow() {
  if (!st.currentWf) return;
  const btn = document.getElementById('testWfBtn');
  btn.textContent = '⏳ Läuft...'; btn.disabled = true;
  if (st.dirty) await saveCurrentWorkflow();
  let payload = {};
  try { payload = JSON.parse(document.getElementById('globalPayloadInput').value); } catch { payload = { message: document.getElementById('globalPayloadInput').value }; }
  const model = document.getElementById('modelSelect').value;
  try {
    const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/execute`, {
      method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ payload, model })
    });
    const d = await r.json();
    st.lastExec = d;
    st.nodeStatusMap = {};
    (d.nodeLogs || []).forEach(l => {
      st.nodeStatusMap[l.node] = {
        status: l.status,
        durationMs: l.durationMs,
        input: l.input,
        output: l.output ?? (Array.isArray(l.items) && l.items.length === 1 ? l.items[0] : l.items),
        items: l.items || (l.output !== undefined ? [l.output] : []),
        outputCount: l.outputCount ?? (Array.isArray(l.items) ? l.items.length : 1),
        error: l.error || null
      };
    });
    renderCanvas();
    populateBottomLogs(d);
    toggleBottomLogs(true);
    toast(`Workflow abgeschlossen (${d.durationMs}ms)`);
  } catch (e) { toast('Fehler: ' + e); }
  btn.textContent = '🧪 Workflow testen'; btn.disabled = false;
}
async function quickExecNode(name) {
  openNdv(name);
  await executeNdvStep();
}

// 3-Column NDV Modal
function updatePinBtnState() {
  const btn = document.getElementById('ndvPinBtn');
  if (!btn || !st.ndvNodeName) return;
  const pinned = Array.isArray(st.currentWf?.pinData?.[st.ndvNodeName]) && st.currentWf.pinData[st.ndvNodeName].length > 0;
  btn.textContent = pinned ? '📌 Angeheftet lösen' : '📌 Daten anheften';
  btn.style.borderColor = pinned ? 'var(--n8n-violet)' : '';
  btn.style.color = pinned ? 'var(--n8n-violet)' : '';
}

function pinCurrentNodeOutput() {
  const nn = st.ndvNodeName;
  if (!st.currentWf || !nn) return;
  pushHistory();
  st.currentWf.pinData = st.currentWf.pinData || {};
  const cur = st.currentWf.pinData[nn];
  if (Array.isArray(cur) && cur.length > 0) {
    delete st.currentWf.pinData[nn];
    toast(`Angeheftete Daten von "${nn}" gelöst`);
  } else {
    let data = st.nodeStatusMap[nn]?.items || st.nodeStatusMap[nn]?.output;
    if (!data) {
      try { data = JSON.parse(document.getElementById('globalPayloadInput')?.value || '{}'); }
      catch { data = { pinned: true, node: nn }; }
    }
    const arr = Array.isArray(data)
      ? data.map(x => (x && typeof x === 'object' && x.json !== undefined) ? x : { json: x })
      : [{ json: data }];
    st.currentWf.pinData[nn] = arr;
    toast(`Ausgang für "${nn}" angeheftet 📌`);
  }
  st.dirty = true; updateHeaderState(); updatePinBtnState(); renderNdvOutput();
  if (st.ndvCenterTab === 'settings') renderNdvCenter();
  renderCanvas();
}

function findPredecessors(nodeName) {
  const preds = [];
  const conns = st.currentWf?.connections || {};
  for (const [src, edges] of Object.entries(conns)) {
    (edges?.main || []).forEach(arr => {
      if ((arr || []).some(t => t.node === nodeName)) preds.push(src);
    });
  }
  for (const nm of Object.keys(st.nodeStatusMap || {})) {
    if (nm !== nodeName && !preds.includes(nm)) preds.push(nm);
  }
  return preds;
}

function openNdv(nodeName) {
  const n = st.currentWf?.nodes?.find(x => x.name === nodeName); if (!n) return;
  st.ndvNodeName = nodeName;
  st.ndvInSourceNode = findPredecessors(nodeName)[0] || null;
  st.ndvCenterTab = 'params';
  document.getElementById('ndvTabParamsBtn').style.borderBottom = '2px solid var(--n8n-coral)';
  document.getElementById('ndvTabSettingsBtn').style.borderBottom = 'none';
  const m = metaFor(n.type);
  const spec = (st.catalog || []).find(c => c.name === n.type);
  document.getElementById('ndvNodeTypeLbl').textContent = `${spec?.displayName || m.label} · ${m.cat || 'Kern'}`;
  document.getElementById('ndvNodeIcon').innerHTML = m.svg;
  document.getElementById('ndvNodeNameInp').value = n.name;
  updatePinBtnState();
  document.getElementById('ndvBackdrop').classList.add('open');
  renderNdvPanels();
}
function closeNdv() { document.getElementById('ndvBackdrop').classList.remove('open'); }
function setNdvCenterTab(tab) {
  st.ndvCenterTab = tab;
  document.getElementById('ndvTabParamsBtn').style.borderBottom = tab === 'params' ? '2px solid var(--n8n-coral)' : 'none';
  document.getElementById('ndvTabSettingsBtn').style.borderBottom = tab === 'settings' ? '2px solid var(--n8n-coral)' : 'none';
  renderNdvCenter();
}
function setNdvInFmt(fmt, btn) { st.ndvInFmt = fmt; btn.parentElement.querySelectorAll('.n8n-seg-btn').forEach(b => b.classList.remove('active')); btn.classList.add('active'); renderNdvInput(); }
function setNdvOutFmt(fmt, btn) { st.ndvOutFmt = fmt; btn.parentElement.querySelectorAll('.n8n-seg-btn').forEach(b => b.classList.remove('active')); btn.classList.add('active'); renderNdvOutput(); }
function renderNdvPanels() { renderNdvInput(); renderNdvCenter(); renderNdvOutput(); }

function flattenSchemaFields(obj, prefix = '') {
  const out = [];
  if (obj === null || obj === undefined) return out;
  const sample = Array.isArray(obj) ? (obj[0] ?? {}) : obj;
  if (typeof sample !== 'object' || sample === null) {
    out.push({ path: prefix || 'value', type: typeof sample, sample });
    return out;
  }
  for (const [k, v] of Object.entries(sample)) {
    const p = prefix ? `${prefix}.${k}` : k;
    const tp = Array.isArray(v) ? 'array' : (v === null ? 'null' : typeof v);
    out.push({ path: p, type: tp, sample: v });
    if (v && typeof v === 'object' && !Array.isArray(v) && prefix.split('.').length < 3) {
      out.push(...flattenSchemaFields(v, p));
    }
  }
  return out;
}

function exprForField(path, sourceNode) {
  const directPreds = findPredecessors(st.ndvNodeName);
  if (!sourceNode || directPreds[0] === sourceNode) return `{{ $json.${path} }}`;
  return `{{ $('${sourceNode}').item.json.${path} }}`;
}

function onSchemaDragStart(ev, path, sourceNode) {
  const expr = exprForField(path, sourceNode || '');
  ev.dataTransfer.setData('text/plain', expr);
  ev.dataTransfer.effectAllowed = 'copy';
}

function renderDataFmt(obj, fmt, draggable = false, sourceNode = '') {
  if (!obj) return '<div style="color:var(--n8n-text-3);">Noch keine Daten. Klicke auf <strong>Schritt testen</strong>.</div>';
  if (fmt === 'schema') {
    const fields = flattenSchemaFields(obj);
    if (!fields.length) return '<div style="color:var(--n8n-text-3);">Leeres Schema</div>';
    const badgeMap = { string: 'Aa', number: '#', boolean: '☑', object: '{}', array: '[]', null: '∅' };
    return `<div style="display:flex;flex-direction:column;gap:5px;">
      ${draggable ? '<div style="font-size:11px;color:var(--n8n-text-3);margin-bottom:2px;">Felder per Drag &amp; Drop in Parameter ziehen:</div>' : ''}
      ${fields.map(f => {
        const valPreview = typeof f.sample === 'object' ? JSON.stringify(f.sample) : String(f.sample ?? '');
        return `<div class="n8n-schema-row" ${draggable ? `draggable="true" ondragstart="onSchemaDragStart(event,'${esc(f.path)}','${esc(sourceNode)}')"` : ''}>
          <span class="n8n-type-badge mono">${badgeMap[f.type] || '?'}</span>
          <strong class="mono" style="font-size:11.5px;">${esc(f.path)}</strong>
          <span class="mono" style="color:var(--n8n-text-3);font-size:11px;margin-left:auto;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;max-width:130px;">${esc(valPreview)}</span>
        </div>`;
      }).join('')}
    </div>`;
  }
  if (fmt === 'table' && typeof obj === 'object') {
    const flat = (Array.isArray(obj) ? obj : [obj]).map(x => (x && x.json !== undefined) ? x.json : x);
    const keys = Array.from(new Set(flat.flatMap(r => Object.keys(r || {}))));
    if (!keys.length) return '<div style="color:var(--n8n-text-3);">Leeres Objekt</div>';
    return `<table style="width:100%;border-collapse:collapse;font-size:11.5px;">
      <thead><tr>${keys.map(k => `<th ${draggable?`draggable="true" ondragstart="onSchemaDragStart(event,'${esc(k)}','${esc(sourceNode)}')" style="cursor:grab;text-align:left;padding:5px 7px;border-bottom:1px solid var(--n8n-border);background:var(--n8n-subtle-bg);"`:`style="text-align:left;padding:5px 7px;border-bottom:1px solid var(--n8n-border);background:var(--n8n-subtle-bg);"`}>${esc(k)}</th>`).join('')}</tr></thead>
      <tbody>${flat.map(r => `<tr>${keys.map(k => `<td style="padding:5px 7px;border-bottom:1px solid var(--n8n-border);word-break:break-word;">${esc(typeof r[k]==='object'?JSON.stringify(r[k]):r[k])}</td>`).join('')}</tr>`).join('')}</tbody>
    </table>`;
  }
  return `<pre class="mono" style="margin:0;white-space:pre-wrap;word-break:break-word;font-size:11.5px;">${esc(JSON.stringify(obj, null, 2))}</pre>`;
}

function renderNdvInput() {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  const body = document.getElementById('ndvInBody');
  const sel = document.getElementById('ndvInNodeSel');
  const stat = st.nodeStatusMap[n.name];
  if (isTrigger(n.type)) {
    if (sel) sel.style.display = 'none';
    const wp = n.parameters?.path || wfWebhookPath(st.currentWf) || st.currentWf.id;
    const url = `${window.location.origin}/webhook/${wp}`;
    body.innerHTML = `
      <div><span class="n8n-field-lbl">Webhook-URL</span><input class="n8n-input mono" readonly value="${esc(url)}" onclick="this.select()"></div>
      <div><span class="n8n-field-lbl">Testdaten (JSON)</span><textarea id="ndvTrigPayload" class="n8n-code-ed mono" rows="8" onchange="document.getElementById('globalPayloadInput').value=this.value">${esc(document.getElementById('globalPayloadInput').value)}</textarea></div>
      <div style="font-size:11.5px;color:var(--n8n-text-3);">cURL:&nbsp;<code class="mono" style="margin-left:4px;background:var(--n8n-subtle-bg);padding:2px 6px;border-radius:4px;">curl -X POST ${esc(url)} -d '{"message":"Hallo"}'</code></div>
    `;
  } else {
    const preds = findPredecessors(n.name);
    if (sel && preds.length) {
      sel.style.display = 'inline-block';
      sel.innerHTML = preds.map(p => `<option value="${esc(p)}" ${p===st.ndvInSourceNode?'selected':''}>${esc(p)}</option>`).join('');
    } else if (sel) {
      sel.style.display = 'none';
    }
    const srcStat = st.ndvInSourceNode ? st.nodeStatusMap[st.ndvInSourceNode] : null;
    const inObj = srcStat?.output || stat?.input || null;
    body.innerHTML = renderDataFmt(inObj, st.ndvInFmt, true, st.ndvInSourceNode || '');
  }
}

const ndvExprTimers = {};
function propEffectiveVal(params, allProps, key) {
  if (params && params[key] !== undefined) return params[key];
  const def = (allProps || []).find(x => x.name === key);
  return def ? def.default : undefined;
}
function shouldShowProperty(prop, params, allProps) {
  const dOpt = prop?.displayOptions;
  if (!dOpt) return true;
  if (dOpt.show) {
    for (const [k, vals] of Object.entries(dOpt.show)) {
      const cur = propEffectiveVal(params, allProps, k);
      if (!Array.isArray(vals) || !vals.some(v => String(cur) === String(v))) return false;
    }
  }
  if (dOpt.hide) {
    const entries = Object.entries(dOpt.hide);
    let matchedAll = true;
    for (const [k, vals] of entries) {
      const cur = propEffectiveVal(params, allProps, k);
      if (!Array.isArray(vals) || !vals.some(v => String(cur) === String(v))) { matchedAll = false; break; }
    }
    if (entries.length && matchedAll) return false;
  }
  return true;
}

function scheduleExprPreview(elId, rawVal) {
  clearTimeout(ndvExprTimers[elId]);
  const s = String(rawVal ?? '');
  const el = document.getElementById(elId);
  if (!el) return;
  if (!(s.startsWith('=') || s.includes('{{'))) {
    el.style.display = 'none'; el.textContent = ''; return;
  }
  el.style.display = 'block';
  el.style.background = 'var(--n8n-subtle-bg)';
  el.style.color = 'var(--n8n-text-2)';
  el.textContent = '⏳ QuickJS wird ausgewertet...';
  ndvExprTimers[elId] = setTimeout(() => runExprPreviewNow(s, elId), 160);
}

async function runExprPreviewNow(expr, elId) {
  const el = document.getElementById(elId);
  if (!el) return;
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName);
  const stat = n ? st.nodeStatusMap[n.name] : null;
  let testPayload = {};
  try { testPayload = JSON.parse(document.getElementById('globalPayloadInput')?.value || '{}'); } catch {}
  const nodeOutputs = {};
  for (const [k, v] of Object.entries(st.nodeStatusMap || {})) {
    if (v && v.output !== undefined) nodeOutputs[k] = v.output;
  }
  try {
    const r = await fetch('/api/n8n/eval-expression', {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ expression: expr, item: stat?.input || testPayload || {}, currentNode: n?.name || '', nodeOutputs, vars: {} })
    });
    const d = await r.json();
    const cur = document.getElementById(elId);
    if (!cur) return;
    cur.style.display = 'block';
    if (d.ok) {
      cur.style.background = 'var(--n8n-green-soft)'; cur.style.color = 'var(--n8n-green)';
      const txt = typeof d.value === 'object' ? JSON.stringify(d.value) : String(d.value ?? '');
      cur.textContent = `⚡ ${d.engine || 'QuickJS'}: ${txt}`;
    } else {
      cur.style.background = 'var(--n8n-red-soft)'; cur.style.color = 'var(--n8n-red)';
      cur.textContent = `✕ Expression-Fehler: ${d.error || 'ungültig'}`;
    }
  } catch {
    const cur = document.getElementById(elId);
    if (cur) { cur.style.display = 'block'; cur.textContent = '✕ Auswertung fehlgeschlagen'; }
  }
}

function toggleParamExprMode(pname, toExpr, prevId) {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  n.parameters = n.parameters || {};
  let cur = String(n.parameters[pname] ?? '');
  if (toExpr) {
    if (!cur.startsWith('=')) cur = '=' + cur;
  } else {
    if (cur.startsWith('=')) cur = cur.slice(1);
  }
  n.parameters[pname] = cur;
  st.dirty = true; updateHeaderState(); renderNdvCenter();
  if (toExpr) scheduleExprPreview(prevId, cur);
}

function handleParamDrop(ev, pname, prevId) {
  ev.preventDefault();
  ev.currentTarget.classList.remove('drop-over');
  const expr = ev.dataTransfer.getData('text/plain');
  if (!expr) return;
  const el = ev.currentTarget;
  const cur = el.value || '';
  const next = cur ? (cur.startsWith('=') ? cur + ' ' + expr : '=' + cur + ' ' + expr) : '=' + expr;
  el.value = next;
  updateNodeParam(pname, next, false);
  el.classList.add('is-expr');
  scheduleExprPreview(prevId, next);
}

function openNdvNewCredModal(credType) {
  const spec = (st.credCatalog || []).find(c => c.name === credType);
  const body = document.getElementById('ndvCredModalBody');
  const fields = (spec?.properties || [{ name: 'value', displayName: 'Schlüssel / Wert', typeOptions: { password: true } }]).map(fp => `
    <div><span class="n8n-field-lbl">${esc(fp.displayName || fp.name)}</span>
    <input class="n8n-input ndv-cred-inp" type="${fp.typeOptions?.password?'password':'text'}" data-ck="${esc(fp.name)}" value="${esc(fp.default||'')}"></div>
  `).join('');
  body.innerHTML = `
    <div class="mono" style="font-size:11.5px;color:var(--n8n-text-2);">Typ: ${esc(spec?.displayName || credType)} (${esc(credType)})</div>
    <div><span class="n8n-field-lbl">Bezeichnung *</span><input id="ndvNewCredName" class="n8n-input" value="${esc((spec?.displayName || credType) + ' Konto')}"></div>
    ${fields}
    <div style="display:flex;justify-content:flex-end;gap:8px;margin-top:6px;">
      <button class="n8n-btn" onclick="document.getElementById('ndvCredModal').classList.remove('open')">Abbrechen</button>
      <button class="n8n-btn n8n-btn-coral" onclick="submitNdvNewCredential('${esc(credType)}')">Speichern &amp; auswählen</button>
    </div>
  `;
  document.getElementById('ndvCredModal').classList.add('open');
}

async function submitNdvNewCredential(credType) {
  const name = document.getElementById('ndvNewCredName')?.value?.trim() || credType;
  const data = {};
  document.querySelectorAll('.ndv-cred-inp').forEach(el => { if (el.value !== '') data[el.dataset.ck] = el.value; });
  const r = await fetch('/rest/credentials', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name, type: credType, data }) });
  if (!r.ok) { toast('Fehler beim Speichern'); return; }
  const created = await r.json();
  st.savedCredentials.unshift(created);
  updateNodeCredential(credType, created.id);
  document.getElementById('ndvCredModal').classList.remove('open');
  renderNdvCenter();
  toast('Anmeldedaten angelegt & verknüpft ✓');
}

function updateNodeCredential(credType, credId) {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  n.credentials = n.credentials || {};
  if (!credId) delete n.credentials[credType];
  else {
    const found = (st.savedCredentials || []).find(c => c.id === credId);
    n.credentials[credType] = { id: credId, name: found ? found.name : credType };
  }
  st.dirty = true; updateHeaderState();
}

function applyPinnedDataEditor(str) {
  const nn = st.ndvNodeName;
  if (!st.currentWf || !nn) return;
  const trimmed = String(str || '').trim();
  st.currentWf.pinData = st.currentWf.pinData || {};
  if (!trimmed || trimmed === '[]' || trimmed === 'null') delete st.currentWf.pinData[nn];
  else {
    try {
      const parsed = JSON.parse(trimmed);
      st.currentWf.pinData[nn] = Array.isArray(parsed) ? parsed : [{ json: parsed }];
    } catch { toast('Ungültiges pinData-JSON'); return; }
  }
  st.dirty = true; updateHeaderState(); updatePinBtnState(); renderNdvOutput(); renderCanvas();
  toast('Angeheftete Daten aktualisiert');
}

function renderNdvCenter() {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  const p = n.parameters = n.parameters || {};
  const body = document.getElementById('ndvCenterBody');

  if (st.ndvCenterTab === 'settings') {
    const onErr = n.onError || (n.continueOnFail ? 'continueRegularOutput' : 'stopWorkflow');
    const pinJson = st.currentWf?.pinData?.[n.name] ? JSON.stringify(st.currentWf.pinData[n.name], null, 2) : '';
    body.innerHTML = `
      <div><span class="n8n-field-lbl">Notizen</span><textarea class="n8n-textarea" rows="3" onchange="updateNodeProp('notes',this.value)">${esc(n.notes||'')}</textarea></div>
      <label style="display:flex;align-items:center;justify-content:space-between;padding:5px 0;"><span>Immer Daten ausgeben (alwaysOutputData)</span><input type="checkbox" ${n.alwaysOutputData?'checked':''} onchange="updateNodeProp('alwaysOutputData',this.checked)"></label>
      <label style="display:flex;align-items:center;justify-content:space-between;padding:5px 0;"><span>Nur einmal ausführen (executeOnce)</span><input type="checkbox" ${n.executeOnce?'checked':''} onchange="updateNodeProp('executeOnce',this.checked)"></label>
      <div><span class="n8n-field-lbl">Bei Fehler (onError)</span>
        <select class="n8n-select" onchange="updateNodeProp('onError',this.value);updateNodeProp('continueOnFail',this.value!=='stopWorkflow');renderCanvas()">
          <option value="stopWorkflow" ${onErr==='stopWorkflow'?'selected':''}>Workflow stoppen</option>
          <option value="continueRegularOutput" ${onErr==='continueRegularOutput'?'selected':''}>Mit regulärer Ausgabe fortfahren</option>
          <option value="continueErrorOutput" ${onErr==='continueErrorOutput'?'selected':''}>Fehlerausgang nutzen</option>
        </select>
      </div>
      <div style="display:grid;grid-template-columns:1fr 110px 120px;gap:10px;align-items:center;padding:4px 0;">
        <label style="display:flex;align-items:center;gap:8px;"><input type="checkbox" ${n.retryOnFail?'checked':''} onchange="updateNodeProp('retryOnFail',this.checked)"><span>Bei Fehler wiederholen</span></label>
        <div><span class="n8n-field-lbl">Max. Versuche</span><input type="number" min="1" max="5" class="n8n-input mono" value="${esc(n.maxTries ?? 3)}" onchange="updateNodeProp('maxTries',Math.max(1,Math.min(5,Number(this.value)||3)))"></div>
        <div><span class="n8n-field-lbl">Pause (ms)</span><input type="number" min="0" max="5000" class="n8n-input mono" value="${esc(n.waitBetweenTries ?? 200)}" onchange="updateNodeProp('waitBetweenTries',Math.max(0,Number(this.value)||200))"></div>
      </div>
      <div>
        <span class="n8n-field-lbl">Angeheftete Ausgabedaten (pinData JSON)</span>
        <textarea class="n8n-code-ed mono" rows="5" placeholder="[]" onchange="applyPinnedDataEditor(this.value)">${esc(pinJson)}</textarea>
      </div>
    `;
    return;
  }

  let fields = '';
  if (n.type.toLowerCase().includes('gmail') || n.type.toLowerCase().includes('email') || n.type.includes('Outlook')) {
    fields += `<div style="padding:8px 10px;border-radius:6px;background:var(--n8n-green-soft);color:var(--n8n-green);font-size:12px;">Abgesicherter Modus aktiv: speichert nur als Entwurf (draftOnly = true).</div>`;
  }

  const schema = st.nodeSchemaByName[n.type];
  if (Array.isArray(schema?.credentials) && schema.credentials.length) {
    fields += schema.credentials.map(cr => {
      const cType = cr.name;
      const selId = n.credentials?.[cType]?.id || '';
      const matching = (st.savedCredentials || []).filter(sc => sc.type === cType);
      const spec = (st.credCatalog || []).find(cs => cs.name === cType);
      return `<div style="padding:8px 10px;border:1px solid var(--n8n-border);border-radius:7px;background:var(--n8n-subtle-bg);">
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:4px;">
          <span class="n8n-field-lbl" style="margin:0;">Anmeldedaten (${esc(spec?.displayName || cType)})</span>
          <button class="n8n-btn" style="height:22px;font-size:11px;padding:0 7px;" onclick="openNdvNewCredModal('${esc(cType)}')">+ Neue Anmeldedaten</button>
        </div>
        <select class="n8n-select" onchange="updateNodeCredential('${esc(cType)}',this.value)">
          <option value="">— Keine ausgewählt —</option>
          ${matching.map(sc => `<option value="${esc(sc.id)}" ${sc.id===selId?'selected':''}>${esc(sc.name)}</option>`).join('')}
        </select>
      </div>`;
    }).join('');
  }

  const schemaProps = Array.isArray(schema?.properties) ? schema.properties : [];
  const initExprChecks = [];
  if (schemaProps.length) {
    if (n.type === 'n8n-nodes-base.httpRequest' && p.jsonBody && p.sendBody === undefined) p.sendBody = true;
    schemaProps.forEach((prop, idx) => {
      if (!shouldShowProperty(prop, p, schemaProps)) return;
      const curVal = propEffectiveVal(p, schemaProps, prop.name);
      const prevId = `exprPrev-${idx}`;
      const pname = esc(prop.name);
      const lbl = esc(prop.displayName || prop.name);
      const descHtml = prop.description ? `<div style="font-size:11px;color:var(--n8n-text-3);margin-top:2px;">${esc(prop.description)}</div>` : '';

      if (prop.type === 'options' && Array.isArray(prop.options)) {
        fields += `<div><span class="n8n-field-lbl">${lbl}</span>
          <select class="n8n-select" onchange="updateNodeParam('${pname}',this.value,true)">
            ${prop.options.map(o => `<option value="${esc(o.value)}" ${String(curVal)===String(o.value)?'selected':''}>${esc(o.name)}</option>`).join('')}
          </select>${descHtml}</div>`;
      } else if (prop.type === 'boolean') {
        const checked = curVal === true || String(curVal) === 'true';
        fields += `<label style="display:flex;align-items:center;justify-content:space-between;padding:5px 0;">
          <span><strong style="font-size:12px;">${lbl}</strong>${descHtml}</span>
          <input type="checkbox" ${checked?'checked':''} onchange="updateNodeParam('${pname}',this.checked,true)">
        </label>`;
      } else if (prop.type === 'number') {
        fields += `<div><span class="n8n-field-lbl">${lbl}</span>
          <input type="number" class="n8n-input mono" value="${esc(curVal ?? 0)}" oninput="updateNodeParam('${pname}',Number(this.value))">${descHtml}</div>`;
      } else {
        const valStr = typeof curVal === 'object' ? JSON.stringify(curVal, null, 2) : String(curVal ?? '');
        const isCode = prop.type === 'json' || prop.typeOptions?.editor === 'codeNodeEditor' || (prop.typeOptions?.rows && prop.typeOptions.rows > 2);
        const isPw = !!prop.typeOptions?.password;
        const rows = prop.typeOptions?.rows || (prop.type === 'json' ? 5 : 4);
        const isExpr = valStr.startsWith('=') || valStr.includes('{{');
        if (isExpr) initExprChecks.push({ expr: valStr, elId: prevId });
        const modeToggle = prop.supportsExpression ? `<div class="n8n-mode-toggle">
          <button type="button" class="n8n-mode-btn ${!isExpr?'active':''}" onclick="toggleParamExprMode('${pname}',false,'${prevId}')">Fest</button>
          <button type="button" class="n8n-mode-btn ${isExpr?'active':''}" onclick="toggleParamExprMode('${pname}',true,'${prevId}')">Ausdruck</button>
        </div>` : '';
        const dropAttrs = `ondragover="event.preventDefault();this.classList.add('drop-over')" ondragleave="this.classList.remove('drop-over')" ondrop="handleParamDrop(event,'${pname}','${prevId}')"`;
        const inputEl = isCode
          ? `<textarea class="n8n-code-ed mono ${isExpr?'is-expr':''}" rows="${rows}" data-param-key="${pname}" ${dropAttrs} oninput="updateNodeParam('${pname}',this.value);scheduleExprPreview('${prevId}',this.value)">${esc(valStr)}</textarea>`
          : `<input type="${isPw?'password':'text'}" class="n8n-input mono ${isExpr?'is-expr':''}" data-param-key="${pname}" ${dropAttrs} value="${esc(valStr)}" oninput="updateNodeParam('${pname}',this.value);scheduleExprPreview('${prevId}',this.value)">`;
        fields += `<div>
          <div class="n8n-param-hdr"><span class="n8n-field-lbl">${lbl}</span>${modeToggle}</div>
          ${inputEl}
          <div class="n8n-expr-preview" id="${prevId}" style="${isExpr?'':'display:none;'}"></div>
          ${descHtml}
        </div>`;
      }
    });
    if ((n.type.includes('agent') || n.type.includes('chainLlm')) && !schemaProps.some(x => x.name === 'systemMessage')) {
      fields += `<div><span class="n8n-field-lbl">Systemnachricht</span><textarea class="n8n-textarea" rows="4" onchange="updateNodeOption('systemMessage',this.value)">${esc(p.options?.systemMessage || '')}</textarea></div>`;
    }
  } else {
    fields += `<div style="color:var(--n8n-text-3);font-size:12px;">Keine weiteren Parameter für diesen Node definiert.</div>`;
  }

  fields += `
    <details style="margin-top:8px;">
      <summary style="cursor:pointer;font-size:12px;font-weight:600;color:var(--n8n-text-2);">Rohparameter (JSON)</summary>
      <textarea id="ndvRawParams" class="n8n-code-ed mono" rows="6" style="margin-top:6px;" onchange="applyRawParams(this.value)">${esc(JSON.stringify(p, null, 2))}</textarea>
    </details>
  `;
  body.innerHTML = fields;
  initExprChecks.forEach(c => runExprPreviewNow(c.expr, c.elId));
}

function renderNdvOutput() {
  const stat = st.nodeStatusMap[st.ndvNodeName];
  const pinned = st.currentWf?.pinData?.[st.ndvNodeName];
  const hasPinned = Array.isArray(pinned) && pinned.length > 0;
  const badgeEl = document.getElementById('ndvOutBadge');
  const outBody = document.getElementById('ndvOutBody');
  if (stat) {
    const cnt = stat.outputCount ?? (Array.isArray(stat.items) ? stat.items.length : 1);
    badgeEl.textContent = `${cnt} ${cnt === 1 ? 'Element' : 'Elemente'} · ${stat.durationMs}ms${hasPinned ? ' · 📌 Angeheftet' : ''}`;
    badgeEl.style.color = stat.status === 'error' ? 'var(--n8n-red)' : (hasPinned ? 'var(--n8n-violet)' : 'var(--n8n-green)');
    const errBox = stat.error ? `<div style="padding:8px 10px;border-radius:6px;background:var(--n8n-red-soft);color:var(--n8n-red);font-size:12px;">✕ ${esc(stat.error)}</div>` : '';
    const payload = stat.items?.length > 1 ? stat.items : (stat.output ?? stat.items ?? null);
    outBody.innerHTML = errBox + renderDataFmt(payload, st.ndvOutFmt, false);
  } else if (hasPinned) {
    const unwrapped = pinned.map(it => (it && it.json !== undefined) ? it.json : it);
    badgeEl.textContent = `📌 Angeheftet (${unwrapped.length})`;
    badgeEl.style.color = 'var(--n8n-violet)';
    outBody.innerHTML = renderDataFmt(unwrapped.length === 1 ? unwrapped[0] : unwrapped, st.ndvOutFmt, false);
  } else {
    badgeEl.textContent = '';
    outBody.innerHTML = renderDataFmt(null, st.ndvOutFmt, false);
  }
}

function updateNodeParam(k, v, rerender) {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  n.parameters = n.parameters || {}; n.parameters[k] = v; st.dirty = true; updateHeaderState();
  const raw = document.getElementById('ndvRawParams'); if (raw) raw.value = JSON.stringify(n.parameters, null, 2);
  if (rerender) renderNdvCenter();
}
function updateNodeOption(k, v) {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  n.parameters = n.parameters || {}; n.parameters.options = n.parameters.options || {}; n.parameters.options[k] = v;
  st.dirty = true; updateHeaderState();
}
function updateNodeProp(k, v) {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  n[k] = v; st.dirty = true; updateHeaderState();
}
function applyRawParams(str) {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  try { n.parameters = JSON.parse(str); st.dirty = true; updateHeaderState(); renderNdvCenter(); toast('Parameter aktualisiert'); } catch { toast('Ungültiges JSON'); }
}
function renameNdvNode(newName) {
  const old = st.ndvNodeName; const nn = newName.trim();
  if (!nn || nn === old || !st.currentWf) return;
  const n = st.currentWf.nodes.find(x => x.name === old); if (!n) return;
  pushHistory();
  n.name = nn;
  if (st.currentWf.pinData?.[old]) { st.currentWf.pinData[nn] = st.currentWf.pinData[old]; delete st.currentWf.pinData[old]; }
  const conns = st.currentWf.connections || {};
  if (conns[old]) { conns[nn] = conns[old]; delete conns[old]; }
  for (const edges of Object.values(conns)) {
    for (const branches of Object.values(edges || {})) {
      if (Array.isArray(branches)) branches.forEach(arr => { if (Array.isArray(arr)) arr.forEach(t => { if (t.node === old) t.node = nn; }); });
    }
  }
  st.ndvNodeName = nn; st.selectedNodes = new Set([nn]); st.dirty = true; updateHeaderState(); renderCanvas();
}

async function executeNdvStep() {
  if (!st.currentWf || !st.ndvNodeName) return;
  const btn = document.getElementById('ndvTestStepBtn');
  btn.textContent = '⏳ Läuft...'; btn.disabled = true;
  const n = st.currentWf.nodes.find(x => x.name === st.ndvNodeName);
  if (st.dirty) await saveCurrentWorkflow();
  let payload = {};
  const predName = st.ndvInSourceNode || findPredecessors(st.ndvNodeName)[0];
  if (predName && st.nodeStatusMap[predName]?.output !== undefined) {
    payload = st.nodeStatusMap[predName].output;
  } else {
    try { payload = JSON.parse(document.getElementById('globalPayloadInput').value); } catch { payload = { message: 'Hallo' }; }
  }
  try {
    const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/execute-node`, {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ nodeName: st.ndvNodeName, payload, model: document.getElementById('modelSelect').value, parameters: n?.parameters })
    });
    const d = await r.json();
    const step = d.nodeLogs?.[0] || {};
    st.nodeStatusMap[st.ndvNodeName] = {
      status: d.status || 'success',
      durationMs: d.durationMs || 0,
      input: d.input ?? payload,
      output: d.output,
      items: step.items || (d.output !== undefined ? [d.output] : []),
      outputCount: step.outputCount ?? 1,
      error: d.error || step.error || null
    };
    renderNdvInput();
    renderNdvOutput();
    renderCanvas();
  } catch (e) { toast('Fehler: ' + e); }
  btn.textContent = '▶ Schritt testen'; btn.disabled = false;
}

// Workflow Settings Modal
function openWfSettingsModal() {
  if (!st.currentWf) return;
  const s = st.currentWf.settings = st.currentWf.settings || {};
  const tz = s.timezone || 'Europe/Berlin';
  const errWf = s.errorWorkflow || '';
  const saveErr = s.saveDataErrorExecution || 'all';
  const saveOk = s.saveDataSuccessExecution || 'all';
  const saveMan = s.saveManualExecutions !== false;
  const timeout = s.executionTimeout ?? 300;
  document.getElementById('wfSettingsBody').innerHTML = `
    <div><span class="n8n-field-lbl">Zeitzone</span>
      <select id="setTz" class="n8n-select">
        ${['Europe/Berlin','Europe/Vienna','Europe/Zurich','UTC'].map(z => `<option value="${z}" ${tz===z?'selected':''}>${z}</option>`).join('')}
      </select>
    </div>
    <div><span class="n8n-field-lbl">Fehler-Workflow (Error Workflow)</span>
      <select id="setErrWf" class="n8n-select">
        <option value="">— Keiner —</option>
        ${st.workflows.filter(w => w.id !== st.currentWf.id).map(w => `<option value="${esc(w.id)}" ${errWf===w.id?'selected':''}>${esc(w.name)}</option>`).join('')}
      </select>
    </div>
    <div><span class="n8n-field-lbl">Fehlgeschlagene Ausführungen speichern</span>
      <select id="setSaveErr" class="n8n-select">
        <option value="all" ${saveErr==='all'?'selected':''}>Alle</option>
        <option value="none" ${saveErr==='none'?'selected':''}>Keine</option>
      </select>
    </div>
    <div><span class="n8n-field-lbl">Erfolgreiche Ausführungen speichern</span>
      <select id="setSaveOk" class="n8n-select">
        <option value="all" ${saveOk==='all'?'selected':''}>Alle</option>
        <option value="none" ${saveOk==='none'?'selected':''}>Keine</option>
      </select>
    </div>
    <label style="display:flex;align-items:center;justify-content:space-between;"><span>Manuelle Ausführungen speichern</span><input id="setSaveMan" type="checkbox" ${saveMan?'checked':''}></label>
    <div><span class="n8n-field-lbl">Ausführungs-Timeout (Sekunden)</span><input id="setTimeout" type="number" class="n8n-input mono" value="${esc(timeout)}"></div>
    <div style="display:flex;justify-content:flex-end;gap:8px;margin-top:4px;">
      <button class="n8n-btn" onclick="document.getElementById('wfSettingsModal').classList.remove('open')">Abbrechen</button>
      <button class="n8n-btn n8n-btn-coral" onclick="saveWfSettings()">Einstellungen speichern</button>
    </div>
  `;
  document.getElementById('wfSettingsModal').classList.add('open');
}
async function saveWfSettings() {
  if (!st.currentWf) return;
  st.currentWf.settings = {
    ...(st.currentWf.settings || {}),
    timezone: document.getElementById('setTz').value,
    errorWorkflow: document.getElementById('setErrWf').value,
    saveDataErrorExecution: document.getElementById('setSaveErr').value,
    saveDataSuccessExecution: document.getElementById('setSaveOk').value,
    saveManualExecutions: document.getElementById('setSaveMan').checked,
    executionTimeout: Number(document.getElementById('setTimeout').value) || 300
  };
  document.getElementById('wfSettingsModal').classList.remove('open');
  await saveCurrentWorkflow();
}

// Workflow CRUD
async function saveCurrentWorkflow() {
  if (!st.currentWf) return;
  const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}`, {
    method: 'PATCH', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      name: st.currentWf.name,
      nodes: st.currentWf.nodes,
      connections: st.currentWf.connections,
      tags: st.currentWf.tags || [],
      pinData: st.currentWf.pinData || {},
      settings: st.currentWf.settings || {}
    })
  });
  if (r.ok) { st.currentWf = await r.json(); st.dirty = false; updateHeaderState(); toast('Gespeichert ✓'); }
}
async function renameWorkflow(val) { if (!st.currentWf || !val.trim()) return; st.currentWf.name = val.trim(); await saveCurrentWorkflow(); const r = await fetch('/api/n8n/workflows'); const d = await r.json(); st.workflows = d.data || []; renderWfPicker(); }
async function toggleActiveWf() {
  if (!st.currentWf) return;
  const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/toggle`, { method: 'POST' });
  const d = await r.json(); st.currentWf.active = d.active; updateHeaderState(); toast(d.active ? 'Workflow aktiviert' : 'Workflow deaktiviert');
}
async function promptAddTag() {
  const t = prompt('Tag hinzufügen:', 'produktion'); if (!t || !st.currentWf) return;
  st.currentWf.tags = Array.isArray(st.currentWf.tags) ? [...st.currentWf.tags, t.trim()] : [t.trim()];
  await saveCurrentWorkflow();
}
async function removeTag(i) {
  if (!st.currentWf) return;
  st.currentWf.tags.splice(i, 1); await saveCurrentWorkflow();
}
async function createBlankWorkflow() {
  const name = prompt('Workflow-Name:', 'Neuer Workflow'); if (!name) return;
  const r = await fetch('/api/n8n/workflows', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name, active: true, nodes: [{ id: 'tr-' + Date.now(), name: 'Manueller Trigger', type: 'n8n-nodes-base.manualTrigger', position: [240, 260], parameters: {} }], connections: {} }) });
  const wf = await r.json();
  st.currentWf = null;
  await init();
  await selectWorkflow(wf.id);
}
async function duplicateCurrentWorkflow() {
  if (!st.currentWf) return;
  const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/duplicate`, { method: 'POST' });
  if (r.ok) { const wf = await r.json(); st.currentWf = null; await init(); await selectWorkflow(wf.id); toast('Workflow dupliziert'); }
}
async function deleteCurrentWorkflow() {
  if (!st.currentWf || !confirm(`Workflow "${st.currentWf.name}" wirklich löschen?`)) return;
  await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}`, { method: 'DELETE' });
  st.currentWf = null; toast('Workflow gelöscht'); await init();
}
function downloadWorkflowJson() {
  if (!st.currentWf) return;
  const blob = new Blob([JSON.stringify(st.currentWf, null, 2)], { type: 'application/json' });
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = `${st.currentWf.id}.json`; a.click();
}
function openWfImportModal() {
  const ta = document.getElementById('wfImportText'); if (ta) ta.value = '';
  const f = document.getElementById('wfImportFile'); if (f) f.value = '';
  document.getElementById('wfImportModal').classList.add('open');
}
function onWfImportFile(ev) {
  const file = ev?.target?.files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = () => { document.getElementById('wfImportText').value = String(reader.result || ''); };
  reader.readAsText(file);
}
async function submitWfImport() {
  const txt = (document.getElementById('wfImportText')?.value || '').trim();
  if (!txt) { toast('Bitte JSON einfügen oder Datei wählen'); return; }
  let parsed;
  try { parsed = JSON.parse(txt); } catch { toast('Ungültiges Workflow-JSON'); return; }
  const payload = {
    name: parsed.name || 'Importierter Workflow',
    active: !!parsed.active,
    nodes: Array.isArray(parsed.nodes) ? parsed.nodes : [],
    connections: (parsed.connections && typeof parsed.connections === 'object') ? parsed.connections : {},
    tags: Array.isArray(parsed.tags) ? parsed.tags : [],
    pinData: (parsed.pinData && typeof parsed.pinData === 'object') ? parsed.pinData : {},
    settings: (parsed.settings && typeof parsed.settings === 'object') ? parsed.settings : {}
  };
  const r = await fetch('/api/n8n/workflows', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
  if (!r.ok) { toast('Import fehlgeschlagen'); return; }
  const wf = await r.json();
  document.getElementById('wfImportModal').classList.remove('open');
  toast(`Workflow "${wf.name}" importiert ✓`);
  st.currentWf = null; await init(); await selectWorkflow(wf.id);
}
function openShareModal() {
  if (!st.currentWf) return;
  const wp = wfWebhookPath(st.currentWf) || st.currentWf.id;
  const url = `${window.location.origin}/webhook/${wp}`;
  document.getElementById('shareModalBody').innerHTML = `
    <div><span class="n8n-field-lbl">Produktions-Webhook-URL</span><input class="n8n-input mono" readonly value="${esc(url)}" onclick="this.select()"></div>
    <div><span class="n8n-field-lbl">API-Ausführungsendpunkt</span><input class="n8n-input mono" readonly value="${esc(window.location.origin + '/api/n8n/workflows/' + st.currentWf.id + '/execute')}" onclick="this.select()"></div>
    <div style="display:flex;justify-content:flex-end;gap:8px;">
      <button class="n8n-btn n8n-btn-coral" onclick="navigator.clipboard?.writeText('${esc(url)}');toast('Webhook-URL kopiert ✓')">URL kopieren</button>
    </div>
  `;
  document.getElementById('shareModal').classList.add('open');
}
async function syncSqlite() {
  await fetch('/api/n8n/sync-sqlite', { method: 'POST' });
  st.currentWf = null;
  toast('Demo-Workflows zurückgesetzt ✓');
  await init();
}

// Paste JSON directly onto Canvas (⌘V)
document.addEventListener('paste', e => {
  if (st.section !== 'editor' || st.editorTab !== 'editor') return;
  if (e.target.closest('input,textarea,select,[contenteditable]')) return;
  const txt = e.clipboardData?.getData('text');
  if (!txt) return;
  try {
    const parsed = JSON.parse(txt);
    if (Array.isArray(parsed?.nodes)) {
      e.preventDefault();
      pasteWorkflowFragment(parsed);
    }
  } catch {}
});

// Keyboard Shortcuts
document.addEventListener('keydown', e => {
  if (e.key === 'Escape') {
    closeNdv(); closeCreatorDrawer(); closeN8nPops(); closeCtxMenu();
    document.getElementById('wfImportModal')?.classList.remove('open');
    document.getElementById('payloadModal')?.classList.remove('open');
    document.getElementById('wfSettingsModal')?.classList.remove('open');
    document.getElementById('ndvCredModal')?.classList.remove('open');
    document.getElementById('shareModal')?.classList.remove('open');
    return;
  }
  if (e.target.closest('input,textarea,select,[contenteditable]')) return;
  if (st.section !== 'editor' || st.editorTab !== 'editor') return;
  if (document.querySelector('.n8n-ndv-backdrop.open')) return;

  const mod = e.metaKey || e.ctrlKey;
  if (mod && e.key.toLowerCase() === 's') { e.preventDefault(); saveCurrentWorkflow(); }
  else if (mod && e.key.toLowerCase() === 'z' && e.shiftKey) { e.preventDefault(); redoCanvas(); }
  else if (mod && e.key.toLowerCase() === 'z') { e.preventDefault(); undoCanvas(); }
  else if (mod && e.key.toLowerCase() === 'y') { e.preventDefault(); redoCanvas(); }
  else if (mod && e.key.toLowerCase() === 'a') { e.preventDefault(); selectAllNodes(); }
  else if (mod && e.key.toLowerCase() === 'c') { e.preventDefault(); copySelectedNodes(); }
  else if (mod && e.key.toLowerCase() === 'v') { pasteFromInternalClipboard(); }
  else if (mod && e.key.toLowerCase() === 'd') {
    e.preventDefault();
    const first = Array.from(st.selectedNodes)[0];
    if (first) duplicateNode(first);
  }
  else if (e.key === 'Delete' || e.key === 'Backspace') {
    if (st.selectedNodes.size) { e.preventDefault(); deleteNodesList(Array.from(st.selectedNodes)); }
  }
  else if (e.shiftKey && e.key.toLowerCase() === 's') { e.preventDefault(); addStickyNoteAtCenter(); }
  else if (e.key.toLowerCase() === 'v' && !mod) { toggleSelectMode(); }
  else if (e.key === 'Tab') { e.preventDefault(); openCreatorDrawer(null); }
});

if (window.innerWidth <= 960) document.getElementById('n8nNav').classList.add('collapsed');
init();
</script>
</body>
</html>"######
}
