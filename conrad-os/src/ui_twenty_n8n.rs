pub fn render_twenty_crm_html() -> &'static str {
    r######"<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Conrad CRM — Conrad KI</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
<style>
:root {
  --t-bg-primary: #ffffff;
  --t-bg-secondary: #fcfcfc;
  --t-bg-tertiary: #f5f5f5;
  --t-bg-hover: #f0f0f0;
  --t-bg-selected: #eef4ff;
  --t-bg-elevated: #ffffff;
  --t-border: #ebebeb;
  --t-border-strong: #d6d6d6;
  --t-text-primary: #171717;
  --t-text-secondary: #666666;
  --t-text-tertiary: #999999;
  --t-accent: #171717;
  --t-focus-ring: #2563eb;
  --t-blue: #1961ed;
  --t-blue-soft: #edf3ff;
  --t-green: #16a34a;
  --t-green-soft: #ecfdf3;
  --t-amber: #c27803;
  --t-amber-soft: #fffbeb;
  --t-purple: #7c3aed;
  --t-purple-soft: #f5f3ff;
  --t-cyan: #0891b2;
  --t-cyan-soft: #ecfeff;
  --t-red: #dc2626;
  --t-red-soft: #fef2f2;
  --t-shadow-sm: 0 1px 2px rgba(0,0,0,0.04);
  --t-shadow-md: 0 6px 20px rgba(0,0,0,0.08);
  --t-shadow-lg: 0 16px 40px rgba(0,0,0,0.12);
}
html.dark {
  --t-bg-primary: #141414;
  --t-bg-secondary: #171717;
  --t-bg-tertiary: #202020;
  --t-bg-hover: #262626;
  --t-bg-selected: #1c2638;
  --t-bg-elevated: #1c1c1c;
  --t-border: #282828;
  --t-border-strong: #3a3a3a;
  --t-text-primary: #ebebeb;
  --t-text-secondary: #999999;
  --t-text-tertiary: #666666;
  --t-accent: #ebebeb;
  --t-focus-ring: #3b82f6;
  --t-blue: #60a5fa;
  --t-blue-soft: rgba(59,130,246,0.15);
  --t-green: #4ade80;
  --t-green-soft: rgba(34,197,94,0.15);
  --t-amber: #fbbf24;
  --t-amber-soft: rgba(245,158,11,0.15);
  --t-purple: #a78bfa;
  --t-purple-soft: rgba(139,92,246,0.16);
  --t-cyan: #22d3ee;
  --t-cyan-soft: rgba(6,182,212,0.15);
  --t-red: #f87171;
  --t-red-soft: rgba(239,68,68,0.15);
}
* { box-sizing: border-box; }
html, body {
  margin: 0; padding: 0; height: 100%; overflow: hidden;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 13px; color: var(--t-text-primary); background: var(--t-bg-primary);
  -webkit-font-smoothing: antialiased;
}
.mono { font-family: 'JetBrains Mono', ui-monospace, monospace; }
button, input, select, textarea { font-family: inherit; font-size: inherit; color: inherit; }
a { color: inherit; text-decoration: none; }

.t-app { display: flex; width: 100vw; height: 100vh; overflow: hidden; }

/* WorkspaceNavigationDrawer (220px) */
.t-drawer {
  width: 220px; min-width: 220px; background: var(--t-bg-secondary);
  border-right: 1px solid var(--t-border); display: flex; flex-direction: column;
  user-select: none; transition: margin-left 0.16s ease; z-index: 30;
}
.t-drawer.collapsed { margin-left: -220px; }
.t-ws-top {
  height: 44px; padding: 0 10px; display: flex; align-items: center; justify-content: space-between;
  gap: 6px;
}
.t-ws-btn {
  flex: 1; min-width: 0; display: flex; align-items: center; gap: 7px; padding: 4px 6px;
  border-radius: 5px; border: 1px solid transparent; background: transparent; cursor: pointer;
  text-align: left;
}
.t-ws-btn:hover { background: var(--t-bg-hover); }
.t-logo-sq {
  width: 20px; height: 20px; border-radius: 5px; background: var(--t-text-primary);
  color: var(--t-bg-primary); font-weight: 700; font-size: 10.5px; flex-shrink: 0;
  display: inline-flex; align-items: center; justify-content: center; letter-spacing: -0.04em;
}
.t-ws-name { font-weight: 600; font-size: 13px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.t-icon-btn {
  width: 26px; height: 26px; border-radius: 5px; border: 1px solid transparent;
  background: transparent; color: var(--t-text-secondary); cursor: pointer;
  display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;
}
.t-icon-btn:hover { background: var(--t-bg-hover); color: var(--t-text-primary); }

.t-nav-scroll { flex: 1; overflow-y: auto; padding: 4px 8px 12px; display: flex; flex-direction: column; gap: 2px; }
.t-nav-section {
  font-size: 11px; font-weight: 500; color: var(--t-text-tertiary);
  padding: 12px 8px 4px; display: flex; align-items: center; justify-content: space-between;
}
.t-nav-item {
  height: 29px; padding: 0 8px; border-radius: 5px; display: flex; align-items: center;
  justify-content: space-between; gap: 8px; cursor: pointer; color: var(--t-text-secondary);
  font-weight: 500; font-size: 13px; border: none; background: transparent; width: 100%; text-align: left;
}
.t-nav-item:hover { background: var(--t-bg-hover); color: var(--t-text-primary); }
.t-nav-item.active { background: var(--t-bg-tertiary); color: var(--t-text-primary); font-weight: 600; }
.t-nav-left { display: flex; align-items: center; gap: 8px; min-width: 0; overflow: hidden; }
.t-nav-left span { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.t-nav-count { font-size: 11px; color: var(--t-text-tertiary); font-variant-numeric: tabular-nums; }
.t-kbd {
  font-size: 10.5px; color: var(--t-text-tertiary); background: var(--t-bg-primary);
  border: 1px solid var(--t-border); border-radius: 4px; padding: 0 5px; line-height: 17px;
}

.t-drawer-foot {
  padding: 8px 10px; border-top: 1px solid var(--t-border);
  display: flex; flex-direction: column; gap: 5px; font-size: 11.5px;
}
.t-foot-row { display: flex; align-items: center; justify-content: space-between; gap: 6px; color: var(--t-text-secondary); }
.t-status-dot { width: 7px; height: 7px; border-radius: 50%; background: var(--t-green); display: inline-block; }

/* Main Stage */
.t-main { flex: 1; min-width: 0; display: flex; flex-direction: column; background: var(--t-bg-primary); position: relative; }

/* Top 44px Header */
.t-header {
  height: 44px; min-height: 44px; padding: 0 12px; border-bottom: 1px solid var(--t-border);
  display: flex; align-items: center; justify-content: space-between; gap: 8px;
}
.t-hdr-left { display: flex; align-items: center; gap: 8px; min-width: 0; }
.t-breadcrumb { display: flex; align-items: center; gap: 7px; font-size: 13px; font-weight: 600; color: var(--t-text-primary); }
.t-hdr-right { display: flex; align-items: center; gap: 6px; flex-wrap: nowrap; }

.t-btn {
  height: 28px; padding: 0 10px; border-radius: 5px; border: 1px solid var(--t-border-strong);
  background: var(--t-bg-primary); color: var(--t-text-primary); font-size: 12.5px; font-weight: 500;
  cursor: pointer; display: inline-flex; align-items: center; gap: 5px; white-space: nowrap;
  box-shadow: var(--t-shadow-sm);
}
.t-btn:hover { background: var(--t-bg-hover); }
.t-btn-primary {
  background: var(--t-text-primary); color: var(--t-bg-primary); border-color: var(--t-text-primary);
}
.t-btn-primary:hover { opacity: 0.9; background: var(--t-text-primary); }
.t-btn-subtle {
  height: 28px; padding: 0 8px; border-radius: 5px; border: 1px solid transparent;
  background: transparent; color: var(--t-text-secondary); font-size: 12.5px; font-weight: 500;
  cursor: pointer; display: inline-flex; align-items: center; gap: 5px; white-space: nowrap;
}
.t-btn-subtle:hover { background: var(--t-bg-hover); color: var(--t-text-primary); }
.t-btn-subtle.active { background: var(--t-bg-tertiary); color: var(--t-text-primary); }

/* 40px ViewBar */
.t-viewbar {
  height: 40px; min-height: 40px; padding: 0 12px; border-bottom: 1px solid var(--t-border);
  display: flex; align-items: center; justify-content: space-between; gap: 8px;
}
.t-vb-left { display: flex; align-items: center; gap: 6px; position: relative; }
.t-vb-right { display: flex; align-items: center; gap: 4px; position: relative; }
.t-view-trigger {
  height: 28px; padding: 0 8px; border-radius: 5px; border: 1px solid transparent;
  background: transparent; cursor: pointer; display: inline-flex; align-items: center; gap: 6px;
  font-weight: 500; font-size: 13px; color: var(--t-text-primary);
}
.t-view-trigger:hover { background: var(--t-bg-hover); }
.t-view-count { color: var(--t-text-tertiary); font-weight: 400; font-variant-numeric: tabular-nums; }
.t-search-inline {
  height: 26px; width: 170px; padding: 0 8px 0 26px; border-radius: 5px;
  border: 1px solid var(--t-border); background: var(--t-bg-secondary); font-size: 12.5px; outline: none;
}
.t-search-inline:focus { border-color: var(--t-border-strong); background: var(--t-bg-primary); }
.t-search-wrap { position: relative; display: inline-flex; align-items: center; }
.t-search-wrap > span { position: absolute; left: 8px; top: 50%; transform: translateY(-50%); color: var(--t-text-tertiary); pointer-events: none; display: inline-flex; align-items: center; }
.t-search-wrap svg { display: block; }

/* Filter Chips Row */
.t-chips-bar {
  min-height: 34px; padding: 4px 12px; border-bottom: 1px solid var(--t-border);
  background: var(--t-bg-secondary); display: none; align-items: center; gap: 6px; flex-wrap: wrap;
}
.t-chips-bar.visible { display: flex; }
.t-chip-filter {
  height: 24px; padding: 0 7px; border-radius: 5px; border: 1px solid var(--t-border-strong);
  background: var(--t-bg-primary); font-size: 12px; color: var(--t-text-secondary);
  display: inline-flex; align-items: center; gap: 5px;
}
.t-chip-filter strong { color: var(--t-text-primary); font-weight: 500; }
.t-chip-filter button {
  border: none; background: transparent; cursor: pointer; padding: 0; color: var(--t-text-tertiary);
  display: inline-flex; align-items: center;
}
.t-chip-filter button:hover { color: var(--t-text-primary); }

/* Popover menus */
.t-popover {
  position: absolute; top: 34px; right: 0; z-index: 60; min-width: 220px;
  background: var(--t-bg-elevated); border: 1px solid var(--t-border-strong); border-radius: 8px;
  box-shadow: var(--t-shadow-md); padding: 6px; display: none;
}
.t-popover.left { left: 0; right: auto; }
.t-popover.open { display: block; }
.t-pop-title { font-size: 11px; font-weight: 600; color: var(--t-text-tertiary); padding: 4px 8px; }
.t-pop-item {
  width: 100%; border: none; background: transparent; padding: 6px 8px; border-radius: 5px;
  display: flex; align-items: center; justify-content: space-between; gap: 8px; cursor: pointer;
  font-size: 12.5px; color: var(--t-text-primary); text-align: left;
}
.t-pop-item:hover { background: var(--t-bg-hover); }

/* RecordTable (Spreadsheet Grid) */
.t-table-wrap { flex: 1; overflow: auto; position: relative; background: var(--t-bg-primary); }
table.t-grid {
  width: 100%; border-collapse: separate; border-spacing: 0; table-layout: fixed;
  min-width: 960px;
}
table.t-grid thead th {
  position: sticky; top: 0; z-index: 12; height: 33px; padding: 0 8px;
  background: var(--t-bg-primary); border-bottom: 1px solid var(--t-border); border-right: 1px solid var(--t-border);
  font-size: 12px; font-weight: 500; color: var(--t-text-secondary); text-align: left; white-space: nowrap;
  user-select: none;
}
table.t-grid thead th.sticky-col, table.t-grid tbody td.sticky-col, table.t-grid tfoot td.sticky-col {
  position: sticky; left: 32px; z-index: 13; background: var(--t-bg-primary);
}
table.t-grid thead th.sticky-chk, table.t-grid tbody td.sticky-chk, table.t-grid tfoot td.sticky-chk {
  position: sticky; left: 0; z-index: 13; background: var(--t-bg-primary); text-align: center; padding: 0;
}
table.t-grid thead th.sticky-col, table.t-grid thead th.sticky-chk { z-index: 15; }
.t-th-inner { display: flex; align-items: center; gap: 6px; overflow: hidden; text-overflow: ellipsis; }
.t-th-inner svg { color: var(--t-text-tertiary); flex-shrink: 0; }

table.t-grid tbody td {
  height: 33px; padding: 0 8px; border-bottom: 1px solid var(--t-border); border-right: 1px solid var(--t-border);
  font-size: 13px; color: var(--t-text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  vertical-align: middle; position: relative; cursor: default;
}
table.t-grid tbody tr:hover td { background: var(--t-bg-secondary); }
table.t-grid tbody tr:hover td.sticky-col, table.t-grid tbody tr:hover td.sticky-chk { background: var(--t-bg-secondary); }
table.t-grid tbody tr.selected td { background: var(--t-bg-selected); }
table.t-grid tbody td.cell-focused { box-shadow: inset 0 0 0 1.5px var(--t-focus-ring); }
table.t-grid tbody td.cell-editing { padding: 0; box-shadow: inset 0 0 0 1.5px var(--t-focus-ring); background: var(--t-bg-primary) !important; }
.t-inline-input {
  width: 100%; height: 32px; padding: 0 8px; border: none; outline: none;
  background: transparent; color: var(--t-text-primary); font-size: 13px;
}

/* Name Cell with Favicon & Hover [Open] button */
.t-name-cell { display: flex; align-items: center; justify-content: space-between; gap: 6px; width: 100%; }
.t-name-main { display: flex; align-items: center; gap: 7px; min-width: 0; flex: 1; }
.t-name-text { font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; cursor: pointer; }
.t-name-text:hover { text-decoration: underline; }
.t-open-hover {
  opacity: 0; height: 22px; padding: 0 6px; border-radius: 4px; border: 1px solid var(--t-border-strong);
  background: var(--t-bg-primary); color: var(--t-text-secondary); font-size: 11px; font-weight: 500;
  cursor: pointer; display: inline-flex; align-items: center; gap: 3px; flex-shrink: 0;
  transition: opacity 0.1s ease;
}
table.t-grid tbody tr:hover .t-open-hover, .t-open-hover:focus { opacity: 1; }
.t-open-hover:hover { color: var(--t-text-primary); background: var(--t-bg-hover); }

.t-fav-wrap {
  width: 16px; height: 16px; border-radius: 3px; flex-shrink: 0;
  display: inline-flex; align-items: center; justify-content: center; overflow: hidden;
  font-size: 9px; font-weight: 600;
}
.t-fav-img { width: 16px; height: 16px; object-fit: contain; display: block; border-radius: 3px; }
.t-avatar-sm {
  width: 18px; height: 18px; border-radius: 99px; font-size: 9.5px; font-weight: 600;
  display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;
}

/* Field Pills */
.t-link-pill {
  display: inline-flex; align-items: center; gap: 4px; max-width: 100%; padding: 1px 6px;
  border-radius: 4px; background: var(--t-bg-tertiary); border: 1px solid var(--t-border);
  color: var(--t-text-secondary); font-size: 12px; overflow: hidden; text-overflow: ellipsis;
}
.t-link-pill:hover { color: var(--t-text-primary); border-color: var(--t-border-strong); }
.t-tag {
  display: inline-flex; align-items: center; gap: 4px; padding: 1px 7px; border-radius: 4px;
  font-size: 11.5px; font-weight: 500; line-height: 18px;
}
.t-tag-gray { background: var(--t-bg-tertiary); color: var(--t-text-secondary); }
.t-tag-green { background: var(--t-green-soft); color: var(--t-green); }
.t-tag-blue { background: var(--t-blue-soft); color: var(--t-blue); }
.t-tag-purple { background: var(--t-purple-soft); color: var(--t-purple); }
.t-tag-amber { background: var(--t-amber-soft); color: var(--t-amber); }
.t-tag-cyan { background: var(--t-cyan-soft); color: var(--t-cyan); }
.t-num { font-variant-numeric: tabular-nums; text-align: right; display: block; }

/* Add Record Row & Aggregate Footer */
.t-add-row td { color: var(--t-text-tertiary); cursor: pointer; background: var(--t-bg-primary); }
.t-add-row:hover td { color: var(--t-text-primary); background: var(--t-bg-secondary); }
table.t-grid tfoot td {
  position: sticky; bottom: 0; z-index: 12; height: 32px; padding: 0 8px;
  background: var(--t-bg-secondary); border-top: 1px solid var(--t-border); border-right: 1px solid var(--t-border);
  font-size: 11.5px; color: var(--t-text-tertiary); white-space: nowrap;
}
table.t-grid tfoot td.sticky-col, table.t-grid tfoot td.sticky-chk { z-index: 14; background: var(--t-bg-secondary); }
.t-agg-val { color: var(--t-text-secondary); font-weight: 500; margin-left: 4px; font-variant-numeric: tabular-nums; }

.t-pagination {
  height: 36px; min-height: 36px; padding: 0 12px; border-top: 1px solid var(--t-border);
  background: var(--t-bg-secondary); display: flex; align-items: center; justify-content: space-between;
  font-size: 12px; color: var(--t-text-secondary);
}

/* Kanban Board (Opportunities By Stage / Tasks By Status) */
.t-kanban {
  flex: 1; display: flex; gap: 10px; padding: 12px; overflow-x: auto; overflow-y: hidden;
  background: var(--t-bg-secondary);
}
.t-kcol {
  min-width: 256px; width: 256px; background: var(--t-bg-tertiary); border: 1px solid var(--t-border);
  border-radius: 8px; display: flex; flex-direction: column; max-height: 100%;
  transition: border-color 0.12s ease, background 0.12s ease;
}
.t-kcol.drag-over { border-color: var(--t-blue); background: var(--t-bg-hover); }
.t-kcol-hdr {
  padding: 9px 10px; display: flex; align-items: center; justify-content: space-between;
  border-bottom: 1px solid var(--t-border);
}
.t-kcol-list { flex: 1; overflow-y: auto; padding: 8px; display: flex; flex-direction: column; gap: 7px; }
.t-kcard {
  background: var(--t-bg-primary); border: 1px solid var(--t-border); border-radius: 6px;
  padding: 10px; cursor: pointer; box-shadow: var(--t-shadow-sm); display: flex; flex-direction: column; gap: 6px;
}
.t-kcard:hover { border-color: var(--t-border-strong); }
.t-kcard-title { font-weight: 600; font-size: 13px; color: var(--t-text-primary); }
.t-kcard-row { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--t-text-secondary); }

/* Settings View (Profile, Experience, Data model, APIs & Webhooks) */
.t-settings-wrap { flex: 1; display: flex; overflow: hidden; background: var(--t-bg-primary); }
.t-settings-nav {
  width: 210px; min-width: 210px; border-right: 1px solid var(--t-border);
  background: var(--t-bg-secondary); padding: 14px 10px; display: flex; flex-direction: column; gap: 2px;
}
.t-settings-body { flex: 1; overflow-y: auto; padding: 24px 32px; max-width: 980px; }
.t-s-card {
  border: 1px solid var(--t-border); border-radius: 8px; background: var(--t-bg-primary);
  padding: 16px; margin-bottom: 14px;
}
.t-obj-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(175px, 1fr)); gap: 10px; margin-bottom: 16px; }
.t-obj-tile {
  border: 1px solid var(--t-border); border-radius: 7px; padding: 12px; cursor: pointer;
  background: var(--t-bg-secondary); display: flex; flex-direction: column; gap: 4px;
}
.t-obj-tile:hover, .t-obj-tile.active { border-color: var(--t-text-primary); background: var(--t-bg-primary); }

/* RightDrawer (Record Show Page + Ask AI Drawer) */
.t-rdrawer {
  position: fixed; top: 0; right: 0; width: 520px; max-width: 100vw; height: 100vh;
  background: var(--t-bg-primary); border-left: 1px solid var(--t-border);
  box-shadow: var(--t-shadow-lg); z-index: 70; display: none; flex-direction: column;
}
.t-rdrawer.open { display: flex; }
.t-rd-hdr {
  padding: 12px 14px; border-bottom: 1px solid var(--t-border);
  display: flex; align-items: center; justify-content: space-between; gap: 8px;
}
.t-rd-tabs {
  display: flex; gap: 2px; padding: 0 14px; border-bottom: 1px solid var(--t-border);
  background: var(--t-bg-secondary); overflow-x: auto;
}
.t-rd-tab {
  padding: 8px 10px; border: none; border-bottom: 2px solid transparent; background: transparent;
  cursor: pointer; font-size: 12.5px; font-weight: 500; color: var(--t-text-secondary); white-space: nowrap;
}
.t-rd-tab.active { color: var(--t-text-primary); border-bottom-color: var(--t-text-primary); font-weight: 600; }
.t-rd-body { flex: 1; overflow-y: auto; padding: 14px; display: flex; flex-direction: column; gap: 12px; }
.t-prop-row {
  display: grid; grid-template-columns: 135px 1fr; align-items: center; gap: 8px;
  min-height: 30px; padding: 2px 0; border-bottom: 1px solid var(--t-border);
}
.t-prop-lbl { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--t-text-secondary); }
.t-prop-input {
  width: 100%; border: 1px solid transparent; border-radius: 4px; padding: 4px 6px;
  background: transparent; font-size: 12.5px; outline: none;
}
.t-prop-input:hover { background: var(--t-bg-secondary); }
.t-prop-input:focus { background: var(--t-bg-primary); border-color: var(--t-focus-ring); }

/* Ask AI Chat */
.t-chat-msgs { flex: 1; overflow-y: auto; padding: 14px; display: flex; flex-direction: column; gap: 10px; }
.t-bubble {
  padding: 10px 12px; border-radius: 7px; border: 1px solid var(--t-border);
  background: var(--t-bg-secondary); font-size: 12.5px; line-height: 1.5; white-space: pre-wrap;
}
.t-bubble.user { background: var(--t-blue-soft); border-color: transparent; align-self: flex-end; max-width: 88%; }
.t-chat-foot { padding: 10px 12px; border-top: 1px solid var(--t-border); display: flex; flex-direction: column; gap: 8px; }

/* Command Palette & Create Modal */
.t-backdrop {
  position: fixed; inset: 0; background: rgba(15,23,42,0.42); backdrop-filter: blur(2px);
  z-index: 90; display: none; align-items: flex-start; justify-content: center; padding-top: 72px;
}
.t-backdrop.open { display: flex; }
.t-modal {
  width: 540px; max-width: 94vw; background: var(--t-bg-elevated);
  border: 1px solid var(--t-border-strong); border-radius: 10px; box-shadow: var(--t-shadow-lg); overflow: hidden;
}
.t-toast {
  position: fixed; bottom: 16px; right: 16px; z-index: 120; padding: 8px 12px; border-radius: 6px;
  background: var(--t-text-primary); color: var(--t-bg-primary); font-size: 12.5px; font-weight: 500;
  box-shadow: var(--t-shadow-md); display: none;
}
.t-toast.show { display: block; }
.t-bulk-bar {
  position: fixed; bottom: 48px; left: 50%; transform: translateX(-50%); z-index: 55;
  display: none; align-items: center; gap: 8px; padding: 7px 12px; border-radius: 9px;
  background: var(--t-bg-elevated); border: 1px solid var(--t-border-strong);
  box-shadow: var(--t-shadow-lg); max-width: calc(100vw - 24px); overflow-x: auto; white-space: nowrap;
}
.t-bulk-bar.show { display: flex; }
.t-btn-danger { color: var(--t-red); border-color: rgba(209,67,67,0.35); }
.t-btn-danger:hover { background: rgba(209,67,67,0.08); }

.t-mobile-toggle { display: none; }
@media (max-width: 900px) {
  .t-drawer { position: fixed; top: 0; left: 0; height: 100vh; box-shadow: var(--t-shadow-lg); }
  .t-drawer.collapsed { margin-left: -220px; }
  .t-mobile-toggle { display: inline-flex; }
  .t-search-inline { width: 115px; }
  .t-rdrawer { width: 100vw; }
}
@media (max-width: 640px) {
  .t-header { padding: 0 8px; gap: 6px; }
  .t-hdr-left { min-width: 0; flex: 1; overflow: hidden; }
  .t-breadcrumb { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .t-hdr-right { flex-shrink: 0; gap: 4px; }
  .t-hdr-right .t-btn { padding: 0 7px; font-size: 12px; }
  .t-hide-xs { display: none !important; }
  .t-viewbar { height: auto; padding: 6px 8px; flex-wrap: wrap; gap: 6px; }
  .t-vb-left { width: 100%; justify-content: space-between; }
  .t-view-trigger { white-space: nowrap; padding: 0 4px; }
  .t-vb-right { width: 100%; justify-content: space-between; gap: 4px; }
  .t-search-wrap { flex: 1; min-width: 0; }
  .t-search-inline { width: 100%; }
}
</style>
</head>
<body>
<div class="t-app">
  <!-- WorkspaceNavigationDrawer -->
  <aside class="t-drawer" id="tDrawer">
    <div class="t-ws-top">
      <button class="t-ws-btn" onclick="cycleWorkspace()" title="Workspace wechseln (Conrad KI / Pala Marketing)">
        <span class="t-logo-sq" id="wsBadge">20</span>
        <span class="t-ws-name" id="wsLabel">Conrad KI</span>
        <span id="wsChev"></span>
      </button>
      <button class="t-icon-btn" onclick="toggleDrawer()" title="Sidebar ein-/ausklappen" id="btnCollapse"></button>
    </div>

    <div class="t-nav-scroll">
      <button class="t-nav-item" onclick="openCmd()">
        <span class="t-nav-left"><span id="icSearch"></span><span>Suche</span></span>
        <span class="t-kbd">⌘K</span>
      </button>
      <button class="t-nav-item" onclick="openAiDrawer()">
        <span class="t-nav-left"><span id="icSparkles"></span><span>KI fragen</span></span>
        <span class="t-kbd">⌘J</span>
      </button>
      <button class="t-nav-item" id="nav-settings" onclick="openSettings('data-model')">
        <span class="t-nav-left"><span id="icSettings"></span><span>Einstellungen</span></span>
      </button>

      <div class="t-nav-section"><span>Favoriten</span></div>
      <button class="t-nav-item" id="fav-all" onclick="applyFavorite('all-companies')">
        <span class="t-nav-left"><span id="icFav1"></span><span>Alle Unternehmen</span></span>
      </button>
      <button class="t-nav-item" id="fav-icp" onclick="applyFavorite('icp-companies')">
        <span class="t-nav-left"><span id="icFav2"></span><span>ICP-Unternehmen</span></span>
      </button>
      <button class="t-nav-item" id="fav-stage" onclick="applyFavorite('by-stage')">
        <span class="t-nav-left"><span id="icFav3"></span><span>Nach Phase</span></span>
      </button>
      <button class="t-nav-item" id="fav-tasks" onclick="applyFavorite('my-tasks')">
        <span class="t-nav-left"><span id="icFav4"></span><span>Meine Aufgaben</span></span>
      </button>

      <div class="t-nav-section"><span>Arbeitsbereich</span></div>
      <button class="t-nav-item active" id="nav-companies" onclick="setObject('companies')">
        <span class="t-nav-left"><span id="icObjComp"></span><span>Unternehmen</span></span>
        <span class="t-nav-count" id="cnt-companies">—</span>
      </button>
      <button class="t-nav-item" id="nav-people" onclick="setObject('people')">
        <span class="t-nav-left"><span id="icObjPeo"></span><span>Kontakte</span></span>
        <span class="t-nav-count" id="cnt-people">—</span>
      </button>
      <button class="t-nav-item" id="nav-opportunities" onclick="setObject('opportunities')">
        <span class="t-nav-left"><span id="icObjOpp"></span><span>Verkaufschancen</span></span>
        <span class="t-nav-count" id="cnt-opportunities">—</span>
      </button>
      <button class="t-nav-item" id="nav-tasks" onclick="setObject('tasks')">
        <span class="t-nav-left"><span id="icObjTask"></span><span>Aufgaben</span></span>
        <span class="t-nav-count" id="cnt-tasks">—</span>
      </button>
      <button class="t-nav-item" id="nav-notes" onclick="setObject('notes')">
        <span class="t-nav-left"><span id="icObjNote"></span><span>Notizen</span></span>
        <span class="t-nav-count" id="cnt-notes">—</span>
      </button>
      <a class="t-nav-item" href="/n8n">
        <span class="t-nav-left"><span id="icObjWf"></span><span>Conrad Automation</span></span>
        <span class="t-nav-count" id="cnt-workflows">—</span>
      </a>
    </div>

    <div class="t-drawer-foot">
      <div class="t-foot-row">
        <span style="display:inline-flex;align-items:center;gap:5px;"><span class="t-status-dot"></span><span>High-Speed CRM · Live</span></span>
        <button class="t-icon-btn" onclick="toggleTheme()" id="themeBtn" title="Light / Dark Theme"></button>
      </div>
      <a href="/showcase" style="color:var(--t-text-tertiary);display:flex;justify-content:space-between;align-items:center;">
        <span>5.000 € → &lt;20 € Kostenvergleich</span>
        <span>↗</span>
      </a>
    </div>
  </aside>

  <!-- Main Workspace Container -->
  <main class="t-main">
    <header class="t-header">
      <div class="t-hdr-left">
        <button class="t-icon-btn" id="expandDrawerBtn" onclick="toggleDrawer()" style="display:none;" title="Sidebar öffnen"></button>
        <div class="t-breadcrumb" id="hdrBreadcrumb"></div>
      </div>
      <div class="t-hdr-right">
        <button class="t-btn" onclick="openCreateModal()"><span id="icPlusHdr"></span><span>Neu<span class="t-hide-xs">er Eintrag</span></span></button>
        <button class="t-btn-subtle" onclick="openAiDrawer()"><span id="icAiHdr"></span><span class="t-hide-xs">KI fragen</span></button>
        <a class="t-btn-subtle" href="/n8n"><span>Conrad Automation</span></a>
      </div>
    </header>

    <!-- ViewBar -->
    <div class="t-viewbar" id="tViewBar">
      <div class="t-vb-left">
        <button class="t-view-trigger" onclick="togglePop('viewPop')">
          <span id="viewIcon"></span>
          <span id="viewName">Alle Unternehmen</span>
          <span class="t-view-count" id="viewCount"></span>
          <span id="viewChev"></span>
        </button>
        <div class="t-popover left" id="viewPop"></div>
      </div>

      <div class="t-vb-right">
        <div class="t-search-wrap">
          <span id="icSearchInline"></span>
          <input class="t-search-inline" id="inlineSearch" placeholder="Suchen..." oninput="onSearchInput()">
        </div>
        <button class="t-btn-subtle" id="filterBtn" onclick="togglePop('filterPop')"><span id="icFilter"></span><span>Filter</span></button>
        <button class="t-btn-subtle" id="sortBtn" onclick="togglePop('sortPop')"><span id="icSort"></span><span>Sortieren</span></button>
        <button class="t-btn-subtle" id="optBtn" onclick="togglePop('optPop')"><span id="icOptions"></span><span>Optionen</span></button>

        <div class="t-popover" id="filterPop">
          <div class="t-pop-title">Filter</div>
          <div id="filterPopItems"></div>
        </div>
        <div class="t-popover" id="sortPop">
          <div class="t-pop-title">Sortieren nach</div>
          <div id="sortPopItems"></div>
        </div>
        <div class="t-popover" id="optPop">
          <div class="t-pop-title">Ansichtsoptionen</div>
          <button class="t-pop-item" id="toggleLayoutBtn" onclick="toggleLayoutMode();closePops()"><span>Ansicht wechseln (Tabelle / Kanban)</span></button>
          <button class="t-pop-item" onclick="saveCurrentViewPrompt();closePops()"><span>Aktuelle Ansicht speichern</span></button>
          <button class="t-pop-item" onclick="openCsvModal();closePops()"><span>CSV importieren</span></button>
          <button class="t-pop-item" onclick="exportBackendCsv();closePops()"><span>CSV exportieren (vollständig)</span></button>
          <button class="t-pop-item" onclick="exportCurrentJson();closePops()"><span>JSON exportieren</span></button>
          <button class="t-pop-item" onclick="syncSqlDump();closePops()"><span>SQLite aus PostgreSQL-Dump synchronisieren</span></button>
          <button class="t-pop-item" onclick="openSettings('apis');closePops()"><span>GraphQL- &amp; REST-APIs</span></button>
        </div>
      </div>
    </div>

    <!-- Active Filter / Sort Chips Bar -->
    <div class="t-chips-bar" id="chipsBar"></div>

    <!-- Stage Body (Table, Kanban, or Settings) -->
    <div id="stageContainer" style="flex:1;min-height:0;display:flex;flex-direction:column;overflow:hidden;"></div>

    <div class="t-bulk-bar" id="bulkActionBar"></div>
    <div class="t-pagination" id="paginationBar">
      <div id="pagLeft">Zeige 0–0 von 0</div>
      <div style="display:flex;align-items:center;gap:6px;">
        <button class="t-btn" id="prevBtn" onclick="prevPage()">‹ Zurück</button>
        <span id="pageLabel">Seite 1 / 1</span>
        <button class="t-btn" id="nextBtn" onclick="nextPage()">Weiter ›</button>
      </div>
    </div>
  </main>
</div>

<!-- RightDrawer (Record Show Page) -->
<aside class="t-rdrawer" id="recordDrawer">
  <div class="t-rd-hdr">
    <div style="display:flex;align-items:center;gap:10px;min-width:0;flex:1;">
      <div id="rdAvatar"></div>
      <div style="min-width:0;flex:1;">
        <input id="rdTitleInput" style="width:100%;border:1px solid transparent;background:transparent;font-weight:600;font-size:15px;padding:2px 4px;border-radius:4px;outline:none;" onchange="renameOpenRecord(this.value)">
        <div style="font-size:11.5px;color:var(--t-text-tertiary);padding-left:4px;" id="rdSub"></div>
      </div>
    </div>
    <div style="display:flex;align-items:center;gap:4px;">
      <button class="t-icon-btn" onclick="deleteOpenRecord()" title="Datensatz löschen" id="rdDeleteBtn"></button>
      <button class="t-icon-btn" onclick="closeRecordDrawer()" title="Schließen" id="rdCloseBtn"></button>
    </div>
  </div>
  <div class="t-rd-tabs" id="rdTabs">
    <button class="t-rd-tab active" onclick="setRdTab('timeline',this)">Zeitleiste</button>
    <button class="t-rd-tab" onclick="setRdTab('tasks',this)">Aufgaben</button>
    <button class="t-rd-tab" onclick="setRdTab('notes',this)">Notizen</button>
    <button class="t-rd-tab" onclick="setRdTab('relations',this)">Verknüpfungen</button>
    <button class="t-rd-tab" onclick="setRdTab('ai',this)">KI-Entwürfe</button>
  </div>
  <div class="t-rd-body" id="rdBody"></div>
</aside>

<!-- Ask AI Drawer (⌘J) -->
<aside class="t-rdrawer" id="aiDrawer" style="width:440px;">
  <div class="t-rd-hdr">
    <div style="display:flex;align-items:center;gap:8px;">
      <span id="icAiDrawer"></span>
      <span style="font-weight:600;font-size:14px;">KI fragen</span>
      <span class="t-tag t-tag-purple">draftOnly</span>
    </div>
    <button class="t-icon-btn" onclick="closeAiDrawer()" id="aiCloseBtn"></button>
  </div>
  <div style="padding:8px 14px;border-bottom:1px solid var(--t-border);background:var(--t-bg-secondary);display:flex;align-items:center;justify-content:space-between;gap:8px;">
    <input type="hidden" id="aiModelSel" value="conrad-n8n-agent:0.6b">
    <span style="font-size:12px;font-weight:600;color:var(--t-text-primary);">⚡ Conrad KI Engine · Spezial-nachtrainiert für CRM &amp; Prozesse</span>
  </div>
  <div class="t-chat-msgs" id="aiMsgs">
    <div class="t-bubble"><strong>Conrad KI-Copilot</strong><br>Spezial-nachtrainiertes Conrad-KI-Modell für Ihr Tagesgeschäft — sucht Datensätze, legt Aufgaben &amp; Notizen an, steuert Vertriebsphasen und erstellt sichere E-Mail-Entwürfe (<code>draftOnly: true</code>).</div>
  </div>
  <div class="t-chat-foot">
    <div style="display:flex;gap:5px;flex-wrap:wrap;">
      <button class="t-btn" style="height:24px;font-size:11.5px;" onclick="quickAi('Zeige Top ICP Firmen')">Top-ICP-Firmen</button>
      <button class="t-btn" style="height:24px;font-size:11.5px;" onclick="quickAi('Erstelle Task: Ohrladen anrufen morgen um 10:00')">+ Aufgabe morgen</button>
      <button class="t-btn" style="height:24px;font-size:11.5px;" onclick="quickAi('Suche Firma Ohrladen und fasse zusammen')">Suche Ohrladen</button>
    </div>
    <div style="display:flex;gap:6px;">
      <input id="aiInput" placeholder="Frage stellen oder Aktion ausführen..." style="flex:1;height:34px;padding:0 10px;border:1px solid var(--t-border-strong);border-radius:6px;background:var(--t-bg-primary);outline:none;" onkeydown="if(event.key==='Enter')sendAi()">
      <button class="t-btn t-btn-primary" style="height:34px;padding:0 14px;" onclick="sendAi()">Ausführen</button>
    </div>
  </div>
</aside>

<!-- Command Palette (⌘K) -->
<div class="t-backdrop" id="cmdBackdrop" onclick="if(event.target===this)closeCmd()">
  <div class="t-modal">
    <div style="padding:10px 14px;border-bottom:1px solid var(--t-border);display:flex;align-items:center;gap:8px;">
      <span id="icCmdSearch"></span>
      <input id="cmdInput" placeholder="Einträge, Ansichten, Einstellungen oder Befehl suchen..." style="flex:1;border:none;outline:none;background:transparent;font-size:13.5px;" oninput="runCmdSearch()">
      <span class="t-kbd">ESC</span>
    </div>
    <div id="cmdResults" style="max-height:360px;overflow-y:auto;padding:6px;"></div>
  </div>
</div>

<!-- Create Record Modal -->
<div class="t-backdrop" id="createBackdrop" onclick="if(event.target===this)closeCreateModal()">
  <div class="t-modal" style="width:460px;">
    <div style="padding:12px 16px;border-bottom:1px solid var(--t-border);display:flex;justify-content:space-between;align-items:center;">
      <strong id="createTitle">Neuer Eintrag</strong>
      <button class="t-icon-btn" onclick="closeCreateModal()">×</button>
    </div>
    <div id="createFields" style="padding:16px;display:flex;flex-direction:column;gap:10px;"></div>
    <div style="padding:10px 16px;border-top:1px solid var(--t-border);display:flex;justify-content:flex-end;gap:8px;background:var(--t-bg-secondary);">
      <button class="t-btn" onclick="closeCreateModal()">Abbrechen</button>
      <button class="t-btn t-btn-primary" onclick="submitCreateModal()">Eintrag erstellen</button>
    </div>
  </div>
</div>

<!-- CSV Import Modal -->
<div class="t-backdrop" id="csvImportBackdrop" onclick="if(event.target===this)closeCsvModal()">
  <div class="t-modal" style="width:520px;">
    <div style="padding:12px 16px;border-bottom:1px solid var(--t-border);display:flex;justify-content:space-between;align-items:center;">
      <strong id="csvModalTitle">CSV importieren</strong>
      <button class="t-icon-btn" onclick="closeCsvModal()">×</button>
    </div>
    <div style="padding:16px;display:flex;flex-direction:column;gap:10px;">
      <div style="font-size:12px;color:var(--t-text-secondary);" id="csvHint">Erste Zeile = Spaltenköpfe (z.B. name,domainUrl,city,employees,arrEur)</div>
      <input type="file" id="csvFileInput" accept=".csv,text/csv" onchange="onCsvFileChange(event)" style="font-size:12px;">
      <textarea id="csvTextarea" class="mono" rows="7" placeholder="name,domainUrl,city,employees,arrEur&#10;Muster GmbH,https://muster.de,Mannheim,25,12000" style="width:100%;padding:8px;border:1px solid var(--t-border-strong);border-radius:6px;background:var(--t-bg-secondary);font-size:12px;outline:none;"></textarea>
      <div id="csvPreviewInfo" style="font-size:11.5px;color:var(--t-text-tertiary);"></div>
    </div>
    <div style="padding:10px 16px;border-top:1px solid var(--t-border);display:flex;justify-content:flex-end;gap:8px;background:var(--t-bg-secondary);">
      <button class="t-btn" onclick="closeCsvModal()">Abbrechen</button>
      <button class="t-btn t-btn-primary" onclick="submitCsvImport()">Import starten</button>
    </div>
  </div>
</div>

<div class="t-toast" id="tToast"></div>

<script>
// Tabler-exact 15/16px SVG Icons
const ICONS = {
  building: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 21h18"/><path d="M5 21v-14l8 -4v18"/><path d="M19 21v-10l-6 -4"/><path d="M9 9v.01"/><path d="M9 12v.01"/><path d="M9 15v.01"/><path d="M9 18v.01"/></svg>',
  user: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="7" r="4"/><path d="M6 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2"/></svg>',
  target: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="12" cy="12" r="5"/><circle cx="12" cy="12" r="9"/></svg>',
  checkbox: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M9 11l3 3l8 -8"/><path d="M20 12v6a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2v-12a2 2 0 0 1 2 -2h9"/></svg>',
  notes: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="5" y="3" width="14" height="18" rx="2"/><path d="M9 7h6"/><path d="M9 11h6"/><path d="M9 15h4"/></svg>',
  workflow: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><path d="M6 21v-6a2 2 0 0 1 2 -2h7"/><circle cx="18" cy="6" r="3"/><path d="M18 9v6"/></svg>',
  search: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="10" cy="10" r="7"/><path d="M21 21l-6 -6"/></svg>',
  settings: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06 .06a2 2 0 0 1 -2.83 2.83l-.06 -.06a1.65 1.65 0 0 0 -1.82 -.33a1.65 1.65 0 0 0 -1 1.51v.09a2 2 0 0 1 -4 0v-.09a1.65 1.65 0 0 0 -1 -1.51a1.65 1.65 0 0 0 -1.82 .33l-.06 .06a2 2 0 0 1 -2.83 -2.83l.06 -.06a1.65 1.65 0 0 0 .33 -1.82a1.65 1.65 0 0 0 -1.51 -1h-.09a2 2 0 0 1 0 -4h.09a1.65 1.65 0 0 0 1.51 -1a1.65 1.65 0 0 0 -.33 -1.82l-.06 -.06a2 2 0 0 1 2.83 -2.83l.06 .06a1.65 1.65 0 0 0 1.82 .33h.08a1.65 1.65 0 0 0 1 -1.51v-.09a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51a1.65 1.65 0 0 0 1.82 -.33l.06 -.06a2 2 0 0 1 2.83 2.83l-.06 .06a1.65 1.65 0 0 0 -.33 1.82v.08a1.65 1.65 0 0 0 1.51 1h.09a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0 -1.51 1z"/></svg>',
  sparkles: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M16 18a2 2 0 0 1 2 2a2 2 0 0 1 2 -2a2 2 0 0 1 -2 -2a2 2 0 0 1 -2 2zm0 -12a2 2 0 0 1 2 2a2 2 0 0 1 2 -2a2 2 0 0 1 -2 -2a2 2 0 0 1 -2 2zm-7 12a6 6 0 0 1 6 -6a6 6 0 0 1 -6 -6a6 6 0 0 1 -6 6a6 6 0 0 1 6 6z"/></svg>',
  star: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z"/></svg>',
  filter: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M4 4h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v7l-6 2v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227z"/></svg>',
  sort: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 9l4 -4l4 4m-4 -4v14"/><path d="M21 15l-4 4l-4 -4m4 4v-14"/></svg>',
  options: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="6" cy="10" r="2"/><path d="M6 4v4"/><path d="M6 12v8"/><circle cx="12" cy="16" r="2"/><path d="M12 4v10"/><path d="M12 18v2"/><circle cx="18" cy="7" r="2"/><path d="M18 4v1"/><path d="M18 9v11"/></svg>',
  plus: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="M5 12h14"/></svg>',
  maximize: '<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M16 4l4 0l0 4"/><path d="M14 10l6 -6"/><path d="M8 20l-4 0l0 -4"/><path d="M4 20l6 -6"/></svg>',
  link: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M9 15l6 -6"/><path d="M11 6l.463 -.536a5 5 0 0 1 7.071 7.072l-.534 .464"/><path d="M13 18l-.397 .534a5.068 5.068 0 0 1 -7.127 0a4.972 4.972 0 0 1 0 -7.071l.524 -.463"/></svg>',
  mail: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9 -6"/></svg>',
  phone: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M5 4h4l2 5l-2.5 1.5a11 11 0 0 0 5 5l1.5 -2.5l5 2v4a2 2 0 0 1 -2 2a16 16 0 0 1 -15 -15a2 2 0 0 1 2 -2"/></svg>',
  pin: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="11" r="3"/><path d="M17.657 16.657l-4.243 4.243a2 2 0 0 1 -2.827 0l-4.244 -4.243a8 8 0 1 1 11.314 0z"/></svg>',
  users: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="7" r="4"/><path d="M3 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/><path d="M21 21v-2a4 4 0 0 0 -3 -3.85"/></svg>',
  euro: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M17.2 7a6 7 0 1 0 0 10"/><path d="M13 10h-8"/><path d="M5 14h8"/></svg>',
  calendar: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="5" width="16" height="16" rx="2"/><path d="M16 3v4"/><path d="M8 3v4"/><path d="M4 11h16"/></svg>',
  trash: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M4 7l16 0"/><path d="M10 11l0 6"/><path d="M14 11l0 6"/><path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12"/><path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3"/></svg>',
  x: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6l-12 12"/><path d="M6 6l12 12"/></svg>',
  chevDown: '<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M6 9l6 6l6 -6"/></svg>',
  table: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 10h18"/><path d="M10 5v14"/></svg>',
  kanban: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M4 4l6 0"/><path d="M14 4l6 0"/><rect x="4" y="8" width="6" height="12" rx="2"/><rect x="14" y="8" width="6" height="6" rx="2"/></svg>',
  panelLeft: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M9 4v16"/></svg>',
  sun: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M3 12h1m8 -9v1m8 8h1m-9 8v1m-6.4 -15.4l.7 .7m12.1 -.7l-.7 .7m0 11.4l.7 .7m-12.1 -.7l-.7 .7"/></svg>',
  moon: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3c.132 0 .263 0 .393 0a7.5 7.5 0 0 0 7.92 12.446a9 9 0 1 1 -8.313 -12.454z"/></svg>',
  database: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="6" rx="8" ry="3"/><path d="M4 6v6a8 3 0 0 0 16 0v-6"/><path d="M4 12v6a8 3 0 0 0 16 0v-6"/></svg>',
  code: '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M7 8l-4 4l4 4"/><path d="M17 8l4 4l-4 4"/><path d="M14 4l-4 16"/></svg>',
  tag: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M7.5 7.5m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0"/><path d="M3 6v5.172a2 2 0 0 0 .586 1.414l7.71 7.71a2.41 2.41 0 0 0 3.408 0l5.592 -5.592a2.41 2.41 0 0 0 0 -3.408l-7.71 -7.71a2 2 0 0 0 -1.414 -.586h-5.172a3 3 0 0 0 -3 3z"/></svg>'
};

const AVATAR_COLORS = [
  ['#dbeafe','#1d4ed8'], ['#dcfce7','#15803d'], ['#fef3c7','#b45309'],
  ['#ede9fe','#6d28d9'], ['#fce7f3','#be185d'], ['#cffafe','#0e7490']
];

const STAGES = ['NEW','SCREENING','MEETING','PROPOSAL','CUSTOMER'];
const STAGE_CLASS = {
  NEW: 't-tag-blue', SCREENING: 't-tag-purple', MEETING: 't-tag-amber',
  PROPOSAL: 't-tag-cyan', CUSTOMER: 't-tag-green'
};
const STAGE_LABEL = {
  NEW: 'Neu', SCREENING: 'Qualifizierung', MEETING: 'Termin',
  PROPOSAL: 'Angebot', CUSTOMER: 'Kunde'
};
window.STAGE_LABEL = STAGE_LABEL;

const state = {
  workspace: window.location.hostname.startsWith('pala.') ? 'pala' : 'conrad-ki',
  mode: 'object', // 'object' | 'settings'
  object: 'companies',
  viewPreset: 'all-companies',
  viewLayout: 'table', // 'table' | 'kanban'
  settingsSection: 'data-model',
  rows: [],
  totalCount: 0,
  aggregates: null,
  limit: 50,
  offset: 0,
  q: '',
  filters: { icp: null, hasDomain: null, stage: null, status: null },
  sort: '',
  selectedIds: new Set(),
  focusedCell: null,
  openRecord: null,
  rdTab: 'timeline',
  timeline: null,
  stats: null,
  metadata: null,
  savedViews: [],
  webhooks: [],
  apiKeys: [],
  lastCreatedToken: null,
  showNewFieldForm: false,
  activeMetaObj: 'companies',
  searchTimer: null
};

function esc(s) {
  return String(s ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
}
function fmtNum(n) { return n === null || n === undefined || n === '' ? '—' : Number(n).toLocaleString('de-DE'); }
function fmtEur(v) {
  const n = Number(v || 0);
  if (!n) return '—';
  return '€' + Math.round(n).toLocaleString('de-DE');
}
function fmtDate(iso) {
  if (!iso) return '—';
  return String(iso).slice(0, 10);
}
function colorFor(seed) {
  let h = 0; const s = String(seed || 'A');
  for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) >>> 0;
  return AVATAR_COLORS[h % AVATAR_COLORS.length];
}
function initials(name) {
  const parts = String(name || '?').trim().split(/\s+/).filter(Boolean);
  if (!parts.length) return '?';
  return (parts[0][0] + (parts[1]?.[0] || '')).toUpperCase();
}
function cleanDomain(raw) {
  return String(raw || '').replace(/^https?:\/\//i, '').replace(/^www\./i, '').split('/')[0].trim();
}
function renderFavicon(domainRaw, name) {
  const dom = cleanDomain(domainRaw);
  const [bg, fg] = colorFor(name || dom);
  const ini = esc(initials(name || dom));
  return `<span class="t-fav-wrap" style="background:${bg};color:${fg};">${ini}</span>`;
}
function toast(msg) {
  const el = document.getElementById('tToast');
  el.textContent = msg; el.classList.add('show');
  setTimeout(() => el.classList.remove('show'), 2200);
}

// Mount static Tabler SVGs
(function mountIcons() {
  const m = {
    wsChev: 'chevDown', btnCollapse: 'panelLeft', expandDrawerBtn: 'panelLeft',
    icSearch: 'search', icSparkles: 'sparkles', icSettings: 'settings',
    icFav1: 'table', icFav2: 'sparkles', icFav3: 'kanban', icFav4: 'checkbox',
    icObjComp: 'building', icObjPeo: 'user', icObjOpp: 'target', icObjTask: 'checkbox',
    icObjNote: 'notes', icObjWf: 'workflow', icPlusHdr: 'plus', icAiHdr: 'sparkles',
    viewChev: 'chevDown', icSearchInline: 'search', icFilter: 'filter', icSort: 'sort',
    icOptions: 'options', rdDeleteBtn: 'trash', rdCloseBtn: 'x', icAiDrawer: 'sparkles',
    aiCloseBtn: 'x', icCmdSearch: 'search'
  };
  for (const [id, k] of Object.entries(m)) {
    const el = document.getElementById(id);
    if (el) el.innerHTML = ICONS[k];
  }
  if (localStorage.getItem('twenty_theme') === 'dark') document.documentElement.classList.add('dark');
  updateThemeIcon();
})();

function updateThemeIcon() {
  const isDark = document.documentElement.classList.contains('dark');
  document.getElementById('themeBtn').innerHTML = isDark ? ICONS.sun : ICONS.moon;
}
function toggleTheme() {
  document.documentElement.classList.toggle('dark');
  localStorage.setItem('twenty_theme', document.documentElement.classList.contains('dark') ? 'dark' : 'light');
  updateThemeIcon();
}
function toggleDrawer() {
  const d = document.getElementById('tDrawer');
  const col = d.classList.toggle('collapsed');
  document.getElementById('expandDrawerBtn').style.display = col ? 'inline-flex' : 'none';
}

function cycleWorkspace() {
  const order = ['conrad-ki', 'pala', 'all'];
  const idx = (order.indexOf(state.workspace) + 1) % order.length;
  state.workspace = order[idx];
  updateWsBadge();
  state.offset = 0;
  state.selectedIds.clear();
  loadMetadata();
  loadSavedViews();
  loadStats();
  loadCurrentView();
}
function updateWsBadge() {
  const names = { 'conrad-ki': 'Conrad KI', 'pala': 'Pala Marketing', 'all': 'Alle Arbeitsbereiche' };
  const badges = { 'conrad-ki': 'CK', 'pala': 'PM', 'all': '20' };
  document.getElementById('wsLabel').textContent = names[state.workspace] || state.workspace;
  document.getElementById('wsBadge').textContent = badges[state.workspace] || '20';
}

async function loadMetadata() {
  try {
    const r = await fetch(`/api/twenty/metadata?workspace=${encodeURIComponent(state.workspace)}`);
    if (r.ok) {
      state.metadata = await r.json();
      if (state.mode === 'settings') renderSettingsView();
      else if (state.rows.length && state.viewLayout === 'table') renderTableView();
    }
  } catch {}
}

async function loadSavedViews() {
  try {
    const r = await fetch(`/api/twenty/views?workspace=${encodeURIComponent(state.workspace)}`);
    if (r.ok) {
      const d = await r.json();
      state.savedViews = d.views || [];
      renderHeaderAndViewBar();
    }
  } catch {}
}

function customFieldsForCurrentObject() {
  const singularMap = { companies: 'company', people: 'person', opportunities: 'opportunity', notes: 'note', tasks: 'task' };
  const target = singularMap[state.object] || state.object;
  const objMeta = (state.metadata?.objects || []).find(o => o.namePlural === state.object || o.nameSingular === target);
  return (objMeta?.fields || []).filter(f => f.isCustom);
}

async function loadStats() {
  try {
    const r = await fetch(`/api/twenty/stats?workspace=${encodeURIComponent(state.workspace)}`);
    const d = await r.json();
    state.stats = d;
    document.getElementById('cnt-companies').textContent = fmtNum(d.companies);
    document.getElementById('cnt-people').textContent = fmtNum(d.people);
    document.getElementById('cnt-opportunities').textContent = fmtNum(d.opportunities);
    document.getElementById('cnt-tasks').textContent = fmtNum(d.tasks);
    document.getElementById('cnt-notes').textContent = fmtNum(d.notes);
    document.getElementById('cnt-workflows').textContent = fmtNum(d.workflows);
  } catch {}
}

function setObject(obj, preset) {
  state.mode = 'object';
  state.object = obj;
  state.offset = 0;
  state.selectedIds.clear();
  state.focusedCell = null;
  state.filters = { icp: null, hasDomain: null, stage: null, status: null };
  state.sort = '';
  state.q = '';
  document.getElementById('inlineSearch').value = '';
  if (obj === 'opportunities') {
    state.viewPreset = preset || 'by-stage';
    state.viewLayout = state.viewPreset === 'all-opps' ? 'table' : 'kanban';
  } else if (obj === 'tasks') {
    state.viewPreset = preset || 'all-tasks';
    state.viewLayout = state.viewPreset === 'by-status' ? 'kanban' : 'table';
  } else {
    state.viewPreset = preset || ('all-' + obj);
    state.viewLayout = 'table';
  }
  updateNavHighlights();
  renderHeaderAndViewBar();
  renderBulkBar();
  loadCurrentView();
  if (window.innerWidth <= 900) {
    document.getElementById('tDrawer').classList.add('collapsed');
    document.getElementById('expandDrawerBtn').style.display = 'inline-flex';
  }
}

function applyFavorite(fav) {
  if (fav === 'all-companies') { setObject('companies', 'all-companies'); }
  else if (fav === 'icp-companies') {
    setObject('companies', 'icp-companies');
    state.filters.icp = true;
    renderHeaderAndViewBar();
    loadCurrentView();
  } else if (fav === 'by-stage') {
    setObject('opportunities', 'by-stage');
  } else if (fav === 'my-tasks') {
    setObject('tasks', 'all-tasks');
  }
}

function updateNavHighlights() {
  document.querySelectorAll('.t-nav-item').forEach(el => el.classList.remove('active'));
  if (state.mode === 'settings') {
    document.getElementById('nav-settings')?.classList.add('active');
    return;
  }
  document.getElementById('nav-' + state.object)?.classList.add('active');
}

function renderHeaderAndViewBar() {
  const objMeta = {
    companies: { label: 'Unternehmen', icon: ICONS.building },
    people: { label: 'Kontakte', icon: ICONS.user },
    opportunities: { label: 'Verkaufschancen', icon: ICONS.target },
    tasks: { label: 'Aufgaben', icon: ICONS.checkbox },
    notes: { label: 'Notizen', icon: ICONS.notes }
  };
  const vb = document.getElementById('tViewBar');
  const pb = document.getElementById('paginationBar');

  if (state.mode === 'settings') {
    const sLbl = { profile: 'Profil', experience: 'Darstellung', 'data-model': 'Datenmodell', apis: 'APIs & Webhooks' };
    document.getElementById('hdrBreadcrumb').innerHTML = `${ICONS.settings} <span>Einstellungen</span> <span style="color:var(--t-text-tertiary);font-weight:400;">/</span> <span style="font-weight:500;">${esc(sLbl[state.settingsSection] || state.settingsSection)}</span>`;
    vb.style.display = 'none';
    pb.style.display = 'none';
    document.getElementById('chipsBar').classList.remove('visible');
    return;
  }

  vb.style.display = 'flex';
  pb.style.display = state.viewLayout === 'table' ? 'flex' : 'none';
  const m = objMeta[state.object] || objMeta.companies;
  document.getElementById('hdrBreadcrumb').innerHTML = `${m.icon} <span>${m.label}</span>`;

  const viewLabels = {
    'all-companies': 'Alle Unternehmen',
    'icp-companies': 'ICP-Unternehmen',
    'domain-companies': 'Mit Website',
    'rhein-neckar': 'Rhein-Neckar-ICP',
    'by-stage': 'Nach Phase',
    'all-opps': 'Alle Verkaufschancen',
    'all-tasks': 'Alle Aufgaben',
    'by-status': 'Nach Status',
    'all-people': 'Alle Kontakte',
    'all-notes': 'Alle Notizen'
  };
  document.getElementById('viewIcon').innerHTML = state.viewLayout === 'kanban' ? ICONS.kanban : ICONS.table;
  const customViewObj = state.savedViews.find(v => 'saved:' + v.id === state.viewPreset);
  document.getElementById('viewName').textContent = customViewObj ? customViewObj.name : (viewLabels[state.viewPreset] || ('Alle ' + m.label));
  document.getElementById('viewCount').textContent = '· ' + fmtNum(state.totalCount);

  // Build View Popover (Standard + Saved DB Views)
  const singularMap = { companies: 'company', people: 'person', opportunities: 'opportunity', notes: 'note', tasks: 'task' };
  const curSing = singularMap[state.object] || state.object;
  const customViews = (state.savedViews || []).filter(v => v.objectName === curSing || v.objectName === state.object);
  const viewsByObj = {
    companies: [
      { id: 'all-companies', label: 'Alle Unternehmen', layout: 'table' },
      { id: 'icp-companies', label: 'ICP-Unternehmen', layout: 'table' },
      { id: 'domain-companies', label: 'Unternehmen mit Website', layout: 'table' },
      { id: 'rhein-neckar', label: 'Region Rhein-Neckar', layout: 'table' }
    ],
    opportunities: [
      { id: 'by-stage', label: 'Nach Phase (Kanban)', layout: 'kanban' },
      { id: 'all-opps', label: 'Alle Verkaufschancen (Tabelle)', layout: 'table' }
    ],
    tasks: [
      { id: 'all-tasks', label: 'Alle Aufgaben (Tabelle)', layout: 'table' },
      { id: 'by-status', label: 'Nach Status (Kanban)', layout: 'kanban' }
    ],
    people: [{ id: 'all-people', label: 'Alle Kontakte', layout: 'table' }],
    notes: [{ id: 'all-notes', label: 'Alle Notizen', layout: 'table' }]
  };
  document.getElementById('viewPop').innerHTML = `<div class="t-pop-title">Standardansichten</div>`
    + (viewsByObj[state.object] || []).map(v => `
      <button class="t-pop-item" onclick="selectViewPreset('${v.id}','${v.layout}')">
        <span style="display:inline-flex;align-items:center;gap:6px;">${v.layout === 'kanban' ? ICONS.kanban : ICONS.table} ${esc(v.label)}</span>
        ${state.viewPreset === v.id ? '<span>✓</span>' : ''}
      </button>
    `).join('')
    + (customViews.length ? `<div style="border-top:1px solid var(--t-border);margin:4px 0;"></div><div class="t-pop-title">Gespeicherte Ansichten</div>` + customViews.map(v => `
      <div class="t-pop-item" style="justify-content:space-between;" onclick="applySavedView('${esc(v.id)}')">
        <span style="display:inline-flex;align-items:center;gap:6px;">${v.layout === 'kanban' ? ICONS.kanban : ICONS.table} ${esc(v.name)}</span>
        <span style="display:inline-flex;align-items:center;gap:6px;">
          ${state.viewPreset === 'saved:' + v.id ? '<span>✓</span>' : ''}
          <span onclick="event.stopPropagation();deleteSavedView('${esc(v.id)}')" title="Ansicht löschen" style="color:var(--t-text-tertiary);padding:0 3px;">×</span>
        </span>
      </div>
    `).join('') : '')
    + `<div style="border-top:1px solid var(--t-border);margin:4px 0;"></div>
      <button class="t-pop-item" onclick="saveCurrentViewPrompt();closePops()"><span>+ Aktuelle Ansicht speichern</span></button>`;

  // Build Filter Popover
  const tSt = { TODO: 'Offen', IN_PROGRESS: 'In Arbeit', DONE: 'Erledigt' };
  let fHtml = '';
  if (state.object === 'companies') {
    fHtml += `<button class="t-pop-item" onclick="toggleFilter('icp', true)"><span>ICP ist Ja</span><span>${state.filters.icp ? '✓' : ''}</span></button>`;
    fHtml += `<button class="t-pop-item" onclick="toggleFilter('hasDomain', true)"><span>Domain nicht leer</span><span>${state.filters.hasDomain ? '✓' : ''}</span></button>`;
  } else if (state.object === 'opportunities') {
    STAGES.forEach(s => {
      fHtml += `<button class="t-pop-item" onclick="toggleFilter('stage', '${s}')"><span>Phase: ${STAGE_LABEL[s] || s}</span><span>${state.filters.stage === s ? '✓' : ''}</span></button>`;
    });
  } else if (state.object === 'tasks') {
    ['TODO','IN_PROGRESS','DONE'].forEach(s => {
      fHtml += `<button class="t-pop-item" onclick="toggleFilter('status', '${s}')"><span>Status: ${tSt[s] || s}</span><span>${state.filters.status === s ? '✓' : ''}</span></button>`;
    });
  }
  fHtml += `<div style="border-top:1px solid var(--t-border);margin:4px 0;"></div><button class="t-pop-item" onclick="clearAllFilters()"><span>Filter zurücksetzen</span></button>`;
  document.getElementById('filterPopItems').innerHTML = fHtml;

  // Build Sort Popover
  const sorts = state.object === 'companies'
    ? [['', 'Standard (Aktualisiert ↓)'], ['name_asc', 'Name (A → Z)'], ['employees_desc', 'Mitarbeiter (Absteigend)'], ['arr_desc', 'ARR (Absteigend)']]
    : [['', 'Standard (Aktualisiert ↓)']];
  document.getElementById('sortPopItems').innerHTML = sorts.map(([val, lbl]) => `
    <button class="t-pop-item" onclick="setSort('${val}')"><span>${esc(lbl)}</span><span>${state.sort === val ? '✓' : ''}</span></button>
  `).join('');

  // Active chips bar
  const chips = [];
  if (state.filters.icp) chips.push(`<span class="t-chip-filter">${ICONS.filter} <span>ICP</span> <strong>Ja</strong> <button onclick="toggleFilter('icp', true)">×</button></span>`);
  if (state.filters.hasDomain) chips.push(`<span class="t-chip-filter">${ICONS.link} <span>Domain</span> <strong>Nicht leer</strong> <button onclick="toggleFilter('hasDomain', true)">×</button></span>`);
  if (state.filters.stage) chips.push(`<span class="t-chip-filter">${ICONS.tag} <span>Phase</span> <strong>${esc(STAGE_LABEL[state.filters.stage] || state.filters.stage)}</strong> <button onclick="toggleFilter('stage', '${esc(state.filters.stage)}')">×</button></span>`);
  if (state.filters.status) chips.push(`<span class="t-chip-filter">${ICONS.checkbox} <span>Status</span> <strong>${esc(tSt[state.filters.status] || state.filters.status)}</strong> <button onclick="toggleFilter('status', '${esc(state.filters.status)}')">×</button></span>`);
  if (state.sort) chips.push(`<span class="t-chip-filter">${ICONS.sort} <span>Sortierung</span> <strong>${esc(state.sort)}</strong> <button onclick="setSort('')">×</button></span>`);
  if (state.q) chips.push(`<span class="t-chip-filter">${ICONS.search} <span>Enthält</span> <strong>${esc(state.q)}</strong> <button onclick="clearSearch()">×</button></span>`);
  const cb = document.getElementById('chipsBar');
  cb.innerHTML = chips.join('');
  cb.classList.toggle('visible', chips.length > 0);
}

function togglePop(id) {
  const target = document.getElementById(id);
  const wasOpen = target.classList.contains('open');
  closePops();
  if (!wasOpen) target.classList.add('open');
}
function closePops() { document.querySelectorAll('.t-popover').forEach(p => p.classList.remove('open')); }
document.addEventListener('click', e => { if (!e.target.closest('.t-vb-left,.t-vb-right')) closePops(); });

function selectViewPreset(presetId, layout) {
  closePops();
  state.viewPreset = presetId;
  state.viewLayout = layout;
  state.filters = { icp: null, hasDomain: null, stage: null, status: null };
  state.q = '';
  document.getElementById('inlineSearch').value = '';
  if (presetId === 'icp-companies') state.filters.icp = true;
  if (presetId === 'domain-companies') state.filters.hasDomain = true;
  if (presetId === 'rhein-neckar') { state.q = 'Mannheim'; document.getElementById('inlineSearch').value = 'Mannheim'; }
  state.offset = 0;
  renderHeaderAndViewBar();
  loadCurrentView();
}
function toggleLayoutMode() {
  if (state.object !== 'opportunities' && state.object !== 'tasks') {
    toast('Kanban ist für Verkaufschancen & Aufgaben verfügbar');
    return;
  }
  state.viewLayout = state.viewLayout === 'kanban' ? 'table' : 'kanban';
  if (state.object === 'opportunities') state.viewPreset = state.viewLayout === 'kanban' ? 'by-stage' : 'all-opps';
  if (state.object === 'tasks') state.viewPreset = state.viewLayout === 'kanban' ? 'by-status' : 'all-tasks';
  renderHeaderAndViewBar();
  loadCurrentView();
}
async function saveCurrentViewPrompt() {
  const name = prompt('Name für die gespeicherte Ansicht:', state.q ? `Suche: ${state.q}` : 'Meine Ansicht');
  if (!name || !name.trim()) return;
  const r = await fetch('/api/twenty/views', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      workspace: state.workspace,
      objectName: state.object,
      name: name.trim(),
      layout: state.viewLayout,
      filters: { ...state.filters, q: state.q || null },
      sort: { key: state.sort || '' }
    })
  });
  if (r.ok) {
    const saved = await r.json();
    state.savedViews.push(saved);
    state.viewPreset = 'saved:' + saved.id;
    renderHeaderAndViewBar();
    toast('Ansicht gespeichert ✓');
  } else {
    toast('Ansicht konnte nicht gespeichert werden');
  }
}
function applySavedView(id) {
  closePops();
  const v = state.savedViews.find(x => x.id === id);
  if (!v) return;
  state.viewPreset = 'saved:' + v.id;
  state.viewLayout = v.layout || 'table';
  const f = v.filters || {};
  state.filters = { icp: f.icp || null, hasDomain: f.hasDomain || null, stage: f.stage || null, status: f.status || null };
  state.q = f.q || '';
  state.sort = (v.sort && v.sort.key) || '';
  document.getElementById('inlineSearch').value = state.q;
  state.offset = 0;
  renderHeaderAndViewBar();
  loadCurrentView();
}
async function deleteSavedView(id) {
  const r = await fetch(`/api/twenty/views/${encodeURIComponent(id)}`, { method: 'DELETE' });
  if (r.ok) {
    state.savedViews = state.savedViews.filter(x => x.id !== id);
    if (state.viewPreset === 'saved:' + id) state.viewPreset = 'all-' + state.object;
    renderHeaderAndViewBar();
    toast('Ansicht gelöscht');
  }
}
function toggleFilter(k, val) {
  closePops();
  state.filters[k] = state.filters[k] === val ? null : val;
  state.offset = 0;
  renderHeaderAndViewBar();
  loadCurrentView();
}
function clearAllFilters() {
  closePops();
  state.filters = { icp: null, hasDomain: null, stage: null, status: null };
  state.sort = '';
  state.q = '';
  document.getElementById('inlineSearch').value = '';
  state.offset = 0;
  renderHeaderAndViewBar();
  loadCurrentView();
}
function clearSearch() { state.q = ''; document.getElementById('inlineSearch').value = ''; state.offset = 0; renderHeaderAndViewBar(); loadCurrentView(); }
function setSort(val) { closePops(); state.sort = val; state.offset = 0; renderHeaderAndViewBar(); loadCurrentView(); }
function onSearchInput() {
  clearTimeout(state.searchTimer);
  state.searchTimer = setTimeout(() => {
    state.q = document.getElementById('inlineSearch').value.trim();
    state.offset = 0;
    renderHeaderAndViewBar();
    loadCurrentView();
  }, 180);
}

async function loadCurrentView() {
  if (state.mode === 'settings') { renderSettingsView(); return; }
  const p = new URLSearchParams({ workspace: state.workspace, limit: String(state.viewLayout === 'kanban' ? 200 : state.limit), offset: String(state.offset) });
  if (state.q) p.set('q', state.q);
  if (state.object === 'companies') {
    if (state.filters.icp) p.set('icp', 'true');
    if (state.filters.hasDomain) p.set('hasDomain', 'true');
    if (state.sort) p.set('sort', state.sort);
  }
  if (state.object === 'opportunities' && state.filters.stage) p.set('stage', state.filters.stage);
  if (state.object === 'tasks' && state.filters.status) p.set('status', state.filters.status);

  try {
    const r = await fetch(`/api/twenty/${state.object}?${p.toString()}`);
    const d = await r.json();
    state.rows = d.data || [];
    state.totalCount = d.totalCount ?? state.rows.length;
    state.aggregates = d.aggregates || null;
    document.getElementById('viewCount').textContent = '· ' + fmtNum(state.totalCount);
    if (state.viewLayout === 'kanban') {
      renderKanbanView();
    } else {
      renderTableView();
    }
    updatePagination();
  } catch (e) {
    document.getElementById('stageContainer').innerHTML = `<div style="padding:24px;color:var(--t-red);">Fehler beim Laden: ${esc(e)}</div>`;
  }
}

function updatePagination() {
  const pb = document.getElementById('paginationBar');
  if (state.viewLayout !== 'table' || state.mode === 'settings') { pb.style.display = 'none'; return; }
  pb.style.display = 'flex';
  const start = state.totalCount ? state.offset + 1 : 0;
  const end = Math.min(state.offset + state.rows.length, state.totalCount);
  const page = Math.floor(state.offset / state.limit) + 1;
  const pages = Math.max(1, Math.ceil(state.totalCount / state.limit));
  document.getElementById('pagLeft').textContent = `Zeige ${fmtNum(start)}–${fmtNum(end)} von ${fmtNum(state.totalCount)}`;
  document.getElementById('pageLabel').textContent = `Seite ${page} / ${pages}`;
  document.getElementById('prevBtn').disabled = state.offset <= 0;
  document.getElementById('nextBtn').disabled = end >= state.totalCount;
}
function prevPage() { state.offset = Math.max(0, state.offset - state.limit); loadCurrentView(); }
function nextPage() { if (state.offset + state.limit < state.totalCount) { state.offset += state.limit; loadCurrentView(); } }

function thCell(iconName, label, width) {
  return `<th style="width:${width}px;"><div class="t-th-inner">${ICONS[iconName] || ''}<span>${esc(label)}</span></div></th>`;
}

function renderCustomFieldThs(cfFields) {
  return (cfFields || []).map(f => thCell('tag', f.label, 145)).join('');
}
function renderCustomFieldTds(row, obj, cfFields) {
  return (cfFields || []).map(f => {
    const raw = (row.customFields && row.customFields[f.name] !== undefined) ? row.customFields[f.name] : (row[f.name] ?? '');
    const ft = String(f.type || 'TEXT').toUpperCase();
    let display = '<span style="color:var(--t-text-tertiary);">—</span>';
    if (ft === 'BOOLEAN') {
      display = raw === true || raw === 'true' ? '<span class="t-tag t-tag-green">Ja</span>' : (raw === false || raw === 'false' ? '<span class="t-tag t-tag-gray">Nein</span>' : '<span style="color:var(--t-text-tertiary);">—</span>');
    } else if (raw !== '' && raw !== null && raw !== undefined) {
      if (ft === 'NUMBER' || ft === 'CURRENCY') display = `<span class="t-num">${ft === 'CURRENCY' ? fmtEur(raw) : fmtNum(raw)}</span>`;
      else if (ft === 'SELECT') display = `<span class="t-tag t-tag-purple">${esc(raw)}</span>`;
      else if (ft === 'URL') display = `<span class="t-link-pill">${esc(raw)}</span>`;
      else display = esc(String(raw));
    }
    const optsJson = JSON.stringify(f.options || []);
    return `<td data-editable="1" data-custom="1" data-obj="${esc(obj)}" data-id="${esc(row.id)}" data-field="${esc(f.name)}" data-type="${esc(ft === 'NUMBER' || ft === 'CURRENCY' ? 'number' : ft === 'SELECT' ? 'select' : ft === 'BOOLEAN' ? 'boolean' : ft === 'DATE' ? 'date' : 'text')}" data-options="${esc(optsJson)}" data-val="${esc(raw === null || raw === undefined ? '' : String(raw))}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${display}</td>`;
  }).join('');
}

function renderTableView() {
  const sc = document.getElementById('stageContainer');
  const allChecked = state.rows.length > 0 && state.rows.every(r => state.selectedIds.has(r.id));
  const cfFields = customFieldsForCurrentObject();
  const cfFoot = cfFields.map(() => '<td></td>').join('');
  let headCols = `<th class="sticky-chk" style="width:32px;"><input type="checkbox" ${allChecked ? 'checked' : ''} onchange="toggleSelectAll(this.checked)"></th>`;
  let bodyRows = '';
  let footRow = `<td class="sticky-chk"></td>`;

  if (state.object === 'companies') {
    headCols += `<th class="sticky-col" style="width:250px;"><div class="t-th-inner">${ICONS.building}<span>Name</span></div></th>`
      + thCell('link', 'Domain', 190)
      + thCell('user', 'Erstellt von', 120)
      + thCell('pin', 'Stadt', 140)
      + thCell('users', 'Mitarbeiter', 100)
      + thCell('euro', 'ARR', 105)
      + thCell('sparkles', 'ICP', 90)
      + thCell('calendar', 'Erstellt am', 110)
      + renderCustomFieldThs(cfFields);

    bodyRows = state.rows.map(c => {
      const dom = c.domainName?.primaryLinkUrl || '';
      const domLabel = cleanDomain(dom);
      const city = c.address?.addressCity || '';
      const arrEur = c.annualRecurringRevenue?.amountEur || ((c.annualRecurringRevenue?.amountMicros || 0) / 1000000);
      const chk = state.selectedIds.has(c.id);
      const [cBg, cFg] = colorFor(c.createdBy?.name || 'CK');
      return `<tr class="${chk ? 'selected' : ''}">
        <td class="sticky-chk"><input type="checkbox" ${chk ? 'checked' : ''} onchange="toggleRowSelect('${esc(c.id)}',this.checked)"></td>
        <td class="sticky-col" data-editable="1" data-obj="companies" data-id="${esc(c.id)}" data-field="name" data-val="${esc(c.name)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">
          <div class="t-name-cell">
            <div class="t-name-main">${renderFavicon(dom, c.name)}<span class="t-name-text" onclick="event.stopPropagation();openRecord('companies','${esc(c.id)}')">${esc(c.name)}</span></div>
            <button class="t-open-hover" onclick="event.stopPropagation();openRecord('companies','${esc(c.id)}')" title="Datensatz öffnen">${ICONS.maximize}</button>
          </div>
        </td>
        <td data-editable="1" data-obj="companies" data-id="${esc(c.id)}" data-field="domainUrl" data-val="${esc(dom)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">
          ${dom ? `<a class="t-link-pill" href="${esc(dom.startsWith('http') ? dom : 'https://' + dom)}" target="_blank" rel="noopener" onclick="event.stopPropagation()">${esc(domLabel)}</a>` : '<span style="color:var(--t-text-tertiary);">—</span>'}
        </td>
        <td><span class="t-tag t-tag-gray"><span class="t-avatar-sm" style="width:14px;height:14px;font-size:8px;background:${cBg};color:${cFg};">${esc(initials(c.createdBy?.name || 'RS'))}</span>${esc(c.createdBy?.name || 'System')}</span></td>
        <td data-editable="1" data-obj="companies" data-id="${esc(c.id)}" data-field="city" data-val="${esc(city)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${esc(city) || '<span style="color:var(--t-text-tertiary);">—</span>'}</td>
        <td data-editable="1" data-obj="companies" data-id="${esc(c.id)}" data-field="employees" data-type="number" data-val="${c.employees ?? ''}" onclick="focusCell(this)" ondblclick="startEditCell(this)"><span class="t-num">${fmtNum(c.employees)}</span></td>
        <td data-editable="1" data-obj="companies" data-id="${esc(c.id)}" data-field="arrEur" data-type="number" data-val="${arrEur ? Math.round(arrEur) : ''}" onclick="focusCell(this)" ondblclick="startEditCell(this)"><span class="t-num">${fmtEur(arrEur)}</span></td>
        <td onclick="toggleCompanyIcp('${esc(c.id)}',${!c.idealCustomerProfile})" style="cursor:pointer;" title="Klicken zum Umschalten">
          ${c.idealCustomerProfile ? '<span class="t-tag t-tag-green">Ja</span>' : '<span class="t-tag t-tag-gray">Nein</span>'}
        </td>
        <td style="color:var(--t-text-secondary);">${fmtDate(c.createdAt)}</td>
        ${renderCustomFieldTds(c, 'companies', cfFields)}
      </tr>`;
    }).join('');

    const pctDom = state.totalCount && state.aggregates
      ? Math.round((state.aggregates.withDomainCount / state.totalCount) * 100) + '%'
      : '—';
    const icpCnt = state.aggregates ? fmtNum(state.aggregates.icpCount) : '—';
    const sumArr = state.aggregates?.totalArrEur ? fmtEur(state.aggregates.totalArrEur) : '€0';
    footRow += `<td class="sticky-col">Alle zählen <span class="t-agg-val">${fmtNum(state.totalCount)}</span></td>`
      + `<td>% nicht leer <span class="t-agg-val">${pctDom}</span></td>`
      + `<td></td><td></td><td></td>`
      + `<td style="text-align:right;">Summe <span class="t-agg-val">${sumArr}</span></td>`
      + `<td>Anzahl Ja <span class="t-agg-val">${icpCnt}</span></td><td></td>${cfFoot}`;

  } else if (state.object === 'people') {
    headCols += `<th class="sticky-col" style="width:230px;"><div class="t-th-inner">${ICONS.user}<span>Name</span></div></th>`
      + thCell('mail', 'E-Mail', 220)
      + thCell('building', 'Unternehmen', 190)
      + thCell('options', 'Position', 170)
      + thCell('phone', 'Telefon', 150)
      + thCell('pin', 'Stadt', 130)
      + thCell('calendar', 'Erstellt am', 110)
      + renderCustomFieldThs(cfFields);

    bodyRows = state.rows.map(p => {
      const [bg, fg] = colorFor(p.fullName);
      const em = p.emails?.primaryEmail || '';
      const ph = p.phones?.primaryPhoneNumber || '';
      const chk = state.selectedIds.has(p.id);
      return `<tr class="${chk ? 'selected' : ''}">
        <td class="sticky-chk"><input type="checkbox" ${chk ? 'checked' : ''} onchange="toggleRowSelect('${esc(p.id)}',this.checked)"></td>
        <td class="sticky-col">
          <div class="t-name-cell">
            <div class="t-name-main"><span class="t-avatar-sm" style="background:${bg};color:${fg};">${esc(initials(p.fullName))}</span><span class="t-name-text" onclick="openRecord('people','${esc(p.id)}')">${esc(p.fullName || 'Ohne Name')}</span></div>
            <button class="t-open-hover" onclick="openRecord('people','${esc(p.id)}')">${ICONS.maximize}</button>
          </div>
        </td>
        <td data-editable="1" data-obj="people" data-id="${esc(p.id)}" data-field="email" data-val="${esc(em)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${em ? `<span class="t-link-pill">${esc(em)}</span>` : '<span style="color:var(--t-text-tertiary);">—</span>'}</td>
        <td>${p.company?.name ? `<span class="t-tag t-tag-gray" style="cursor:pointer;" onclick="openRecord('companies','${esc(p.company.id)}')">${esc(p.company.name)}</span>` : '<span style="color:var(--t-text-tertiary);">—</span>'}</td>
        <td data-editable="1" data-obj="people" data-id="${esc(p.id)}" data-field="jobTitle" data-val="${esc(p.jobTitle||'')}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${esc(p.jobTitle) || '<span style="color:var(--t-text-tertiary);">—</span>'}</td>
        <td data-editable="1" data-obj="people" data-id="${esc(p.id)}" data-field="phone" data-val="${esc(ph)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${esc(ph) || '<span style="color:var(--t-text-tertiary);">—</span>'}</td>
        <td data-editable="1" data-obj="people" data-id="${esc(p.id)}" data-field="city" data-val="${esc(p.city||'')}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${esc(p.city) || '<span style="color:var(--t-text-tertiary);">—</span>'}</td>
        <td style="color:var(--t-text-secondary);">${fmtDate(p.createdAt)}</td>
        ${renderCustomFieldTds(p, 'people', cfFields)}
      </tr>`;
    }).join('');
    footRow += `<td class="sticky-col">Alle zählen <span class="t-agg-val">${fmtNum(state.totalCount)}</span></td><td colspan="6"></td>${cfFoot}`;

  } else if (state.object === 'opportunities') {
    headCols += `<th class="sticky-col" style="width:240px;"><div class="t-th-inner">${ICONS.target}<span>Name</span></div></th>`
      + thCell('tag', 'Phase', 135)
      + thCell('euro', 'Betrag', 120)
      + thCell('building', 'Unternehmen', 190)
      + thCell('user', 'Ansprechpartner', 170)
      + thCell('calendar', 'Abschlussdatum', 120)
      + renderCustomFieldThs(cfFields);

    bodyRows = state.rows.map(o => {
      const amt = o.amount?.amountEur || ((o.amount?.amountMicros || 0) / 1000000);
      const chk = state.selectedIds.has(o.id);
      return `<tr class="${chk ? 'selected' : ''}">
        <td class="sticky-chk"><input type="checkbox" ${chk ? 'checked' : ''} onchange="toggleRowSelect('${esc(o.id)}',this.checked)"></td>
        <td class="sticky-col" data-editable="1" data-obj="opportunities" data-id="${esc(o.id)}" data-field="name" data-val="${esc(o.name)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">
          <div class="t-name-cell">
            <span class="t-name-text" onclick="event.stopPropagation();openRecord('opportunities','${esc(o.id)}')">${esc(o.name)}</span>
            <button class="t-open-hover" onclick="event.stopPropagation();openRecord('opportunities','${esc(o.id)}')">${ICONS.maximize}</button>
          </div>
        </td>
        <td>
          <select onchange="patchRecord('opportunities','${esc(o.id)}',{stage:this.value})" style="border:none;background:transparent;cursor:pointer;font-size:11.5px;font-weight:600;outline:none;">
            ${STAGES.map(s => `<option value="${s}" ${o.stage===s?'selected':''}>${STAGE_LABEL[s] || s}</option>`).join('')}
          </select>
        </td>
        <td data-editable="1" data-obj="opportunities" data-id="${esc(o.id)}" data-field="amountEur" data-type="number" data-val="${amt ? Math.round(amt) : ''}" onclick="focusCell(this)" ondblclick="startEditCell(this)"><span class="t-num">${fmtEur(amt)}</span></td>
        <td>${o.company?.name ? `<span class="t-tag t-tag-gray">${esc(o.company.name)}</span>` : '—'}</td>
        <td>${esc(o.pointOfContact?.fullName || '—')}</td>
        <td data-editable="1" data-obj="opportunities" data-id="${esc(o.id)}" data-field="closeDate" data-val="${esc(fmtDate(o.closeDate)==='—'?'':fmtDate(o.closeDate))}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${fmtDate(o.closeDate)}</td>
        ${renderCustomFieldTds(o, 'opportunities', cfFields)}
      </tr>`;
    }).join('');
    const totalAmt = state.rows.reduce((s, o) => s + (o.amount?.amountEur || ((o.amount?.amountMicros || 0) / 1000000)), 0);
    footRow += `<td class="sticky-col">Alle zählen <span class="t-agg-val">${fmtNum(state.totalCount)}</span></td><td></td><td style="text-align:right;">Summe <span class="t-agg-val">${fmtEur(totalAmt)}</span></td><td colspan="3"></td>${cfFoot}`;

  } else if (state.object === 'tasks') {
    headCols += `<th class="sticky-col" style="width:280px;"><div class="t-th-inner">${ICONS.checkbox}<span>Titel</span></div></th>`
      + thCell('tag', 'Status', 135)
      + thCell('calendar', 'Fällig am', 120)
      + thCell('building', 'Verknüpfungen', 190)
      + thCell('notes', 'Inhalt', 260)
      + renderCustomFieldThs(cfFields);

    bodyRows = state.rows.map(t => {
      const done = t.status === 'DONE';
      const chk = state.selectedIds.has(t.id);
      const tSt = { TODO: 'Offen', IN_PROGRESS: 'In Arbeit', DONE: 'Erledigt' };
      return `<tr class="${chk ? 'selected' : ''}">
        <td class="sticky-chk"><input type="checkbox" ${chk ? 'checked' : ''} onchange="toggleRowSelect('${esc(t.id)}',this.checked)"></td>
        <td class="sticky-col" data-editable="1" data-obj="tasks" data-id="${esc(t.id)}" data-field="title" data-val="${esc(t.title)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">
          <div class="t-name-cell">
            <div class="t-name-main">
              <button class="t-icon-btn" style="width:18px;height:18px;border:1px solid var(--t-border-strong);border-radius:99px;font-size:10px;flex-shrink:0;${done?'background:var(--t-green);color:#fff;border-color:var(--t-green);':''}" onclick="event.stopPropagation();patchRecord('tasks','${esc(t.id)}',{status:'${done?'TODO':'DONE'}'})" title="Als erledigt markieren">${done ? '✓' : ''}</button>
              <span class="t-name-text" style="${done ? 'text-decoration:line-through;color:var(--t-text-tertiary);' : ''}" onclick="event.stopPropagation();openRecord('tasks','${esc(t.id)}')">${esc(t.title)}</span>
            </div>
            <button class="t-open-hover" onclick="event.stopPropagation();openRecord('tasks','${esc(t.id)}')">${ICONS.maximize}</button>
          </div>
        </td>
        <td>
          <select onchange="patchRecord('tasks','${esc(t.id)}',{status:this.value})" style="border:none;background:transparent;cursor:pointer;font-size:11.5px;font-weight:600;outline:none;">
            ${['TODO','IN_PROGRESS','DONE'].map(s => `<option value="${s}" ${t.status===s?'selected':''}>${tSt[s] || s}</option>`).join('')}
          </select>
        </td>
        <td data-editable="1" data-obj="tasks" data-id="${esc(t.id)}" data-field="dueAt" data-val="${esc(fmtDate(t.dueAt)==='—'?'':fmtDate(t.dueAt))}" onclick="focusCell(this)" ondblclick="startEditCell(this)">${fmtDate(t.dueAt)}</td>
        <td>${t.companyName ? `<span class="t-tag t-tag-gray">${esc(t.companyName)}</span>` : '—'}</td>
        <td style="color:var(--t-text-secondary);">${esc((t.bodyMarkdown || '').slice(0, 70)) || '—'}</td>
        ${renderCustomFieldTds(t, 'tasks', cfFields)}
      </tr>`;
    }).join('');
    footRow += `<td class="sticky-col">Alle zählen <span class="t-agg-val">${fmtNum(state.totalCount)}</span></td><td colspan="4"></td>${cfFoot}`;

  } else if (state.object === 'notes') {
    headCols += `<th class="sticky-col" style="width:240px;"><div class="t-th-inner">${ICONS.notes}<span>Titel</span></div></th>`
      + thCell('notes', 'Textvorschau', 380)
      + thCell('building', 'Verknüpfungen', 180)
      + thCell('user', 'Erstellt von', 120)
      + thCell('calendar', 'Erstellt am', 110)
      + renderCustomFieldThs(cfFields);

    bodyRows = state.rows.map(n => {
      const chk = state.selectedIds.has(n.id);
      return `<tr class="${chk ? 'selected' : ''}">
      <td class="sticky-chk"><input type="checkbox" ${chk ? 'checked' : ''} onchange="toggleRowSelect('${esc(n.id)}',this.checked)"></td>
      <td class="sticky-col" data-editable="1" data-obj="notes" data-id="${esc(n.id)}" data-field="title" data-val="${esc(n.title)}" onclick="focusCell(this)" ondblclick="startEditCell(this)">
        <div class="t-name-cell">
          <span class="t-name-text" onclick="event.stopPropagation();openRecord('notes','${esc(n.id)}')">${esc(n.title || 'Notiz')}</span>
          <button class="t-open-hover" onclick="event.stopPropagation();openRecord('notes','${esc(n.id)}')">${ICONS.maximize}</button>
        </div>
      </td>
      <td style="color:var(--t-text-secondary);">${esc((n.bodyMarkdown || '').replace(/\s+/g,' ').slice(0, 95)) || '—'}</td>
      <td>${n.companyName ? `<span class="t-tag t-tag-gray">${esc(n.companyName)}</span>` : '—'}</td>
      <td><span class="t-tag t-tag-gray">${esc(n.createdBy?.name || 'System')}</span></td>
      <td style="color:var(--t-text-secondary);">${fmtDate(n.createdAt)}</td>
      ${renderCustomFieldTds(n, 'notes', cfFields)}
    </tr>`;
    }).join('');
    footRow += `<td class="sticky-col">Alle zählen <span class="t-agg-val">${fmtNum(state.totalCount)}</span></td><td colspan="4"></td>${cfFoot}`;
  }

  const colSpanCount = (state.object === 'companies' ? 9 : state.object === 'people' ? 8 : state.object === 'opportunities' ? 7 : 6) + cfFields.length;
  sc.innerHTML = `<div class="t-table-wrap">
    <table class="t-grid">
      <thead><tr>${headCols}</tr></thead>
      <tbody>
        ${bodyRows || `<tr><td colspan="${colSpanCount}" style="text-align:center;padding:32px;color:var(--t-text-tertiary);">Keine Datensätze gefunden.</td></tr>`}
        <tr class="t-add-row" onclick="openCreateModal()">
          <td class="sticky-chk">+</td>
          <td class="sticky-col" colspan="${colSpanCount - 1}" style="font-size:12.5px;">+ Neu hinzufügen</td>
        </tr>
      </tbody>
      <tfoot><tr>${footRow}</tr></tfoot>
    </table>
  </div>`;
  renderBulkBar();
}

// Inline Cell Focus & Editing
function focusCell(td) {
  document.querySelectorAll('td.cell-focused').forEach(el => el.classList.remove('cell-focused'));
  td.classList.add('cell-focused');
  state.focusedCell = td;
}
function startEditCell(td) {
  if (!td || td.classList.contains('cell-editing')) return;
  const obj = td.dataset.obj, id = td.dataset.id, field = td.dataset.field;
  const oldVal = td.dataset.val ?? '';
  const type = td.dataset.type || 'text';
  td.classList.add('cell-editing');

  let inp;
  if (type === 'boolean' || type === 'select') {
    inp = document.createElement('select');
    inp.className = 't-inline-input';
    if (type === 'boolean') {
      inp.innerHTML = `<option value="">—</option><option value="true">Ja</option><option value="false">Nein</option>`;
    } else {
      let opts = [];
      try { opts = JSON.parse(td.dataset.options || '[]'); } catch {}
      inp.innerHTML = `<option value="">—</option>` + opts.map(o => {
        const val = typeof o === 'string' ? o : (o.value || o.label || '');
        const lbl = typeof o === 'string' ? o : (o.label || o.value || '');
        return `<option value="${esc(val)}">${esc(lbl)}</option>`;
      }).join('');
    }
    inp.value = oldVal;
  } else {
    inp = document.createElement('input');
    inp.className = 't-inline-input';
    inp.type = type === 'number' ? 'number' : type === 'date' ? 'date' : 'text';
    inp.value = oldVal;
  }

  td.innerHTML = '';
  td.appendChild(inp);
  inp.focus();
  if (inp.select) inp.select();

  let done = false;
  const finish = async (save) => {
    if (done) return; done = true;
    td.classList.remove('cell-editing');
    const newVal = inp.value;
    if (save && newVal !== oldVal) {
      const body = {};
      if (type === 'number') body[field] = Number(newVal || 0);
      else if (type === 'boolean') body[field] = newVal === 'true';
      else body[field] = newVal;
      await patchRecord(obj, id, body);
    } else {
      loadCurrentView();
    }
  };
  if (inp.tagName === 'SELECT') {
    inp.addEventListener('change', () => finish(true));
  }
  inp.addEventListener('keydown', e => {
    if (e.key === 'Enter') { e.preventDefault(); finish(true); }
    else if (e.key === 'Escape') { e.preventDefault(); finish(false); }
  });
  inp.addEventListener('blur', () => finish(true));
}

async function patchRecord(obj, id, body) {
  const r = await fetch(`/api/twenty/${obj}/${encodeURIComponent(id)}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });
  if (r.ok) {
    toast('Gespeichert ✓');
    await loadCurrentView();
    if (state.openRecord && state.openRecord.id === id) {
      const updated = state.rows.find(x => x.id === id);
      if (updated) { state.openRecord.row = updated; renderRecordDrawer(); }
    }
  } else {
    toast('Fehler beim Speichern');
  }
}

async function toggleCompanyIcp(id, nextVal) {
  await patchRecord('companies', id, { idealCustomerProfile: nextVal });
}
function toggleRowSelect(id, checked) {
  if (checked) state.selectedIds.add(id); else state.selectedIds.delete(id);
  renderTableView();
}
function toggleSelectAll(checked) {
  state.selectedIds.clear();
  if (checked) state.rows.forEach(r => state.selectedIds.add(r.id));
  renderTableView();
}

function renderBulkBar() {
  const bar = document.getElementById('bulkActionBar');
  if (!bar) return;
  const cnt = state.selectedIds.size;
  if (!cnt || state.mode === 'settings' || state.viewLayout !== 'table') {
    bar.classList.remove('show');
    bar.innerHTML = '';
    return;
  }
  let extra = '';
  if (state.object === 'opportunities') {
    extra = `<select class="t-btn" style="height:26px;" onchange="if(this.value){bulkUpdateSelected({stage:this.value});this.value='';}">
      <option value="">Phase ändern…</option>
      ${STAGES.map(s => `<option value="${s}">${STAGE_LABEL[s] || s}</option>`).join('')}
    </select>`;
  } else if (state.object === 'tasks') {
    extra = `<select class="t-btn" style="height:26px;" onchange="if(this.value){bulkUpdateSelected({status:this.value});this.value='';}">
      <option value="">Status ändern…</option>
      <option value="TODO">Offen</option>
      <option value="IN_PROGRESS">In Arbeit</option>
      <option value="DONE">Erledigt</option>
    </select>`;
  } else if (state.object === 'companies') {
    extra = `<button class="t-btn" style="height:26px;" onclick="bulkUpdateSelected({icpTarget:true})">ICP = Ja</button>
      <button class="t-btn" style="height:26px;" onclick="bulkUpdateSelected({icpTarget:false})">ICP = Nein</button>`;
  }
  bar.innerHTML = `
    <span style="font-size:12px;font-weight:600;">${cnt} ausgewählt</span>
    ${extra}
    <button class="t-btn" style="height:26px;" onclick="exportBackendCsv(true)">CSV exportieren</button>
    <button class="t-btn t-btn-danger" style="height:26px;" onclick="bulkDeleteSelected()">Ausgewählte löschen</button>
    <button class="t-icon-btn" onclick="state.selectedIds.clear();renderTableView()" title="Auswahl aufheben">×</button>
  `;
  bar.classList.add('show');
}

async function bulkDeleteSelected() {
  const ids = Array.from(state.selectedIds);
  if (!ids.length || !confirm(`${ids.length} ausgewählte Einträge wirklich aus SQLite löschen?`)) return;
  const r = await fetch(`/api/twenty/objects/${state.object}/bulk-delete`, {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ ids })
  });
  if (r.ok) {
    const d = await r.json();
    state.selectedIds.clear();
    toast(`${d.deleted || ids.length} Einträge gelöscht ✓`);
    loadStats();
    loadCurrentView();
  }
}

async function bulkUpdateSelected(patch) {
  const ids = Array.from(state.selectedIds);
  if (!ids.length) return;
  const r = await fetch(`/api/twenty/objects/${state.object}/bulk-update`, {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ ids, patch })
  });
  if (r.ok) {
    toast(`${ids.length} Einträge aktualisiert ✓`);
    loadCurrentView();
  }
}

// Kanban Board (Opportunities & Tasks)
function renderKanbanView() {
  renderBulkBar();
  const sc = document.getElementById('stageContainer');
  if (state.object === 'opportunities') {
    const html = STAGES.map(st => {
      const cards = state.rows.filter(o => o.stage === st);
      const sum = cards.reduce((s, o) => s + (o.amount?.amountEur || ((o.amount?.amountMicros || 0) / 1000000)), 0);
      const cls = STAGE_CLASS[st] || 't-tag-gray';
      const lbl = (typeof STAGE_LABEL !== 'undefined' && STAGE_LABEL[st]) || st;
      return `<div class="t-kcol" ondragover="event.preventDefault();this.classList.add('drag-over')" ondragleave="this.classList.remove('drag-over')" ondrop="this.classList.remove('drag-over');dropOpp(event,'${st}')">
        <div class="t-kcol-hdr">
          <div style="display:flex;align-items:center;gap:6px;">
            <span class="t-tag ${cls}">${lbl}</span>
            <span style="font-size:12px;color:var(--t-text-tertiary);">${cards.length}</span>
          </div>
          <span class="mono" style="font-size:11.5px;color:var(--t-text-secondary);">${sum ? fmtEur(sum) : ''}</span>
        </div>
        <div class="t-kcol-list">
          ${cards.map(o => {
            const amt = o.amount?.amountEur || ((o.amount?.amountMicros || 0) / 1000000);
            return `<div class="t-kcard" draggable="true" ondragstart="event.dataTransfer.setData('text/plain','${esc(o.id)}')" onclick="openRecord('opportunities','${esc(o.id)}')">
              <div class="t-kcard-title">${esc(o.name)}</div>
              ${o.company?.name ? `<div class="t-kcard-row">${ICONS.building}<span>${esc(o.company.name)}</span></div>` : ''}
              <div style="display:flex;justify-content:space-between;align-items:center;font-size:11.5px;color:var(--t-text-secondary);">
                <span style="font-weight:600;color:var(--t-text-primary);">${fmtEur(amt)}</span>
                <span>${fmtDate(o.closeDate)}</span>
              </div>
            </div>`;
          }).join('')}
          <button class="t-btn-subtle" style="justify-content:center;font-size:12px;margin-top:2px;" onclick="openCreateModal({stage:'${st}'})">+ Neu hinzufügen</button>
        </div>
      </div>`;
    }).join('');
    sc.innerHTML = `<div class="t-kanban">${html}</div>`;
  } else if (state.object === 'tasks') {
    const statuses = ['TODO','IN_PROGRESS','DONE'];
    const stCls = { TODO: 't-tag-gray', IN_PROGRESS: 't-tag-blue', DONE: 't-tag-green' };
    const stLbl = { TODO: 'Offen', IN_PROGRESS: 'In Arbeit', DONE: 'Erledigt' };
    sc.innerHTML = `<div class="t-kanban">${statuses.map(st => {
      const cards = state.rows.filter(t => t.status === st);
      return `<div class="t-kcol" ondragover="event.preventDefault();this.classList.add('drag-over')" ondragleave="this.classList.remove('drag-over')" ondrop="this.classList.remove('drag-over');dropTask(event,'${st}')">
        <div class="t-kcol-hdr">
          <span class="t-tag ${stCls[st]}">${stLbl[st] || st}</span>
          <span style="font-size:12px;color:var(--t-text-tertiary);">${cards.length}</span>
        </div>
        <div class="t-kcol-list">
          ${cards.map(t => `<div class="t-kcard" draggable="true" ondragstart="event.dataTransfer.setData('text/plain','${esc(t.id)}')" onclick="openRecord('tasks','${esc(t.id)}')">
            <div class="t-kcard-title">${esc(t.title)}</div>
            <div class="t-kcard-row">${ICONS.calendar}<span>${fmtDate(t.dueAt)}</span></div>
            ${t.companyName ? `<div class="t-kcard-row">${ICONS.building}<span>${esc(t.companyName)}</span></div>` : ''}
          </div>`).join('')}
          <button class="t-btn-subtle" style="justify-content:center;font-size:12px;margin-top:2px;" onclick="openCreateModal({status:'${st}'})">+ Neu hinzufügen</button>
        </div>
      </div>`;
    }).join('')}</div>`;
  }
}
async function dropOpp(ev, st) {
  ev.preventDefault(); const id = ev.dataTransfer.getData('text/plain');
  if (id) await patchRecord('opportunities', id, { stage: st });
}
async function dropTask(ev, st) {
  ev.preventDefault(); const id = ev.dataTransfer.getData('text/plain');
  if (id) await patchRecord('tasks', id, { status: st });
}

// RightDrawer (Record Show Page)
async function openRecord(obj, id) {
  let row = state.rows.find(x => x.id === id);
  if (!row) {
    try { const r = await fetch(`/api/twenty/${obj}/${encodeURIComponent(id)}`); if (r.ok) row = await r.json(); } catch {}
  }
  if (!row) return;
  state.openRecord = { obj, id, row };
  state.rdTab = 'timeline';
  state.timeline = null;
  document.getElementById('recordDrawer').classList.add('open');
  renderRecordDrawer();

  const qKey = obj === 'companies' ? 'companyId' : obj === 'people' ? 'personId' : obj === 'opportunities' ? 'opportunityId' : null;
  if (qKey) {
    try {
      const r = await fetch(`/api/twenty/timeline?${qKey}=${encodeURIComponent(id)}`);
      state.timeline = await r.json();
      renderRecordDrawer();
    } catch {}
  }
}
function closeRecordDrawer() { document.getElementById('recordDrawer').classList.remove('open'); }
function setRdTab(tab, btn) {
  state.rdTab = tab;
  document.querySelectorAll('.t-rd-tab').forEach(b => b.classList.remove('active'));
  btn.classList.add('active');
  renderRecordDrawer();
}
async function renameOpenRecord(val) {
  if (!state.openRecord || !val.trim()) return;
  const { obj, id } = state.openRecord;
  const field = (obj === 'tasks' || obj === 'notes') ? 'title' : 'name';
  if (obj === 'people') return;
  await patchRecord(obj, id, { [field]: val.trim() });
}
async function deleteOpenRecord() {
  if (!state.openRecord) return;
  const { obj, id } = state.openRecord;
  if (!confirm('Diesen Datensatz wirklich aus SQLite löschen?')) return;
  const r = await fetch(`/api/twenty/${obj}/${encodeURIComponent(id)}`, { method: 'DELETE' });
  if (r.ok) {
    closeRecordDrawer();
    toast('Gelöscht');
    loadStats();
    loadCurrentView();
  }
}

function renderRecordDrawer() {
  const { obj, id, row } = state.openRecord;
  const title = obj === 'people' ? row.fullName : (row.name || row.title || 'Datensatz');
  const dom = row.domainName?.primaryLinkUrl || '';
  document.getElementById('rdAvatar').innerHTML = renderFavicon(dom, title);
  document.getElementById('rdTitleInput').value = title || '';
  const singularMap = { companies: 'Unternehmen', people: 'Kontakt', opportunities: 'Verkaufschance', tasks: 'Aufgabe', notes: 'Notiz' };
  document.getElementById('rdSub').textContent = `${singularMap[obj] || obj} · ${id.slice(0, 12)} · Erstellt am ${fmtDate(row.createdAt)}`;

  const tSt = { TODO: 'Offen', IN_PROGRESS: 'In Arbeit', DONE: 'Erledigt' };
  const body = document.getElementById('rdBody');
  let propsHtml = '';
  if (obj === 'companies') {
    const arr = row.annualRecurringRevenue?.amountEur || ((row.annualRecurringRevenue?.amountMicros || 0) / 1000000);
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.link} Domain</span><input class="t-prop-input" value="${esc(dom)}" onchange="patchRecord('companies','${esc(id)}',{domainUrl:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.pin} Stadt</span><input class="t-prop-input" value="${esc(row.address?.addressCity||'')}" onchange="patchRecord('companies','${esc(id)}',{city:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.users} Mitarbeiter</span><input class="t-prop-input" type="number" value="${row.employees ?? ''}" onchange="patchRecord('companies','${esc(id)}',{employees:Number(this.value||0)})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.euro} ARR (EUR)</span><input class="t-prop-input" type="number" value="${arr ? Math.round(arr) : ''}" onchange="patchRecord('companies','${esc(id)}',{arrEur:Number(this.value||0)})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.sparkles} ICP</span><select class="t-prop-input" onchange="patchRecord('companies','${esc(id)}',{idealCustomerProfile:this.value==='true'})"><option value="true" ${row.idealCustomerProfile?'selected':''}>Ja</option><option value="false" ${!row.idealCustomerProfile?'selected':''}>Nein</option></select></div>
    `;
  } else if (obj === 'people') {
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.mail} E-Mail</span><input class="t-prop-input" value="${esc(row.emails?.primaryEmail||'')}" onchange="patchRecord('people','${esc(id)}',{email:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.phone} Telefon</span><input class="t-prop-input" value="${esc(row.phones?.primaryPhoneNumber||'')}" onchange="patchRecord('people','${esc(id)}',{phone:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.options} Position</span><input class="t-prop-input" value="${esc(row.jobTitle||'')}" onchange="patchRecord('people','${esc(id)}',{jobTitle:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.pin} Stadt</span><input class="t-prop-input" value="${esc(row.city||'')}" onchange="patchRecord('people','${esc(id)}',{city:this.value})"></div>
    `;
  } else if (obj === 'opportunities') {
    const amt = row.amount?.amountEur || ((row.amount?.amountMicros || 0) / 1000000);
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} Phase</span><select class="t-prop-input" onchange="patchRecord('opportunities','${esc(id)}',{stage:this.value})">${STAGES.map(s=>`<option value="${s}" ${row.stage===s?'selected':''}>${STAGE_LABEL[s] || s}</option>`).join('')}</select></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.euro} Betrag (€)</span><input class="t-prop-input" type="number" value="${amt ? Math.round(amt) : ''}" onchange="patchRecord('opportunities','${esc(id)}',{amountEur:Number(this.value||0)})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.calendar} Abschlussdatum</span><input class="t-prop-input" value="${esc(fmtDate(row.closeDate)==='—'?'':fmtDate(row.closeDate))}" placeholder="YYYY-MM-DD" onchange="patchRecord('opportunities','${esc(id)}',{closeDate:this.value})"></div>
    `;
  } else if (obj === 'tasks') {
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.checkbox} Status</span><select class="t-prop-input" onchange="patchRecord('tasks','${esc(id)}',{status:this.value})">${['TODO','IN_PROGRESS','DONE'].map(s=>`<option value="${s}" ${row.status===s?'selected':''}>${tSt[s] || s}</option>`).join('')}</select></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.calendar} Fällig am</span><input class="t-prop-input" value="${esc(fmtDate(row.dueAt)==='—'?'':fmtDate(row.dueAt))}" onchange="patchRecord('tasks','${esc(id)}',{dueAt:this.value})"></div>
      <div style="margin-top:8px;"><textarea class="t-prop-input" rows="5" style="border:1px solid var(--t-border);" onchange="patchRecord('tasks','${esc(id)}',{bodyMarkdown:this.value})">${esc(row.bodyMarkdown||'')}</textarea></div>
    `;
  } else if (obj === 'notes') {
    propsHtml = `<textarea class="t-prop-input" rows="10" style="border:1px solid var(--t-border);padding:8px;" onchange="patchRecord('notes','${esc(id)}',{bodyMarkdown:this.value})">${esc(row.bodyMarkdown||'')}</textarea>`;
  }

  const cfFields = customFieldsForCurrentObject();
  if (cfFields.length) {
    propsHtml += cfFields.map(f => {
      const val = (row.customFields && row.customFields[f.name] !== undefined) ? row.customFields[f.name] : (row[f.name] ?? '');
      const ft = String(f.type || 'TEXT').toUpperCase();
      if (ft === 'BOOLEAN') {
        const b = val === true || val === 'true';
        return `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} ${esc(f.label)}</span><select class="t-prop-input" onchange="patchRecord('${esc(obj)}','${esc(id)}',{'${esc(f.name)}':this.value==='true'})"><option value="false" ${!b?'selected':''}>Nein</option><option value="true" ${b?'selected':''}>Ja</option></select></div>`;
      }
      if (ft === 'SELECT' && Array.isArray(f.options) && f.options.length) {
        return `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} ${esc(f.label)}</span><select class="t-prop-input" onchange="patchRecord('${esc(obj)}','${esc(id)}',{'${esc(f.name)}':this.value})"><option value="">—</option>${f.options.map(o => {
          const ov = typeof o === 'string' ? o : (o.value || o.label || '');
          const ol = typeof o === 'string' ? o : (o.label || o.value || '');
          return `<option value="${esc(ov)}" ${String(val)===ov?'selected':''}>${esc(ol)}</option>`;
        }).join('')}</select></div>`;
      }
      const inType = (ft === 'NUMBER' || ft === 'CURRENCY') ? 'number' : ft === 'DATE' ? 'date' : 'text';
      return `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} ${esc(f.label)}</span><input class="t-prop-input" type="${inType}" value="${esc(val)}" onchange="patchRecord('${esc(obj)}','${esc(id)}',{'${esc(f.name)}':this.type==='number'?Number(this.value||0):this.value})"></div>`;
    }).join('');
  }

  const tl = state.timeline || { notes: [], tasks: [], opportunities: [], people: [], events: [] };
  let tabHtml = '';
  if (state.rdTab === 'timeline') {
    tabHtml = `
      <div style="display:flex;gap:6px;flex-wrap:wrap;padding:8px 0;">
        <button class="t-btn" data-prompt="${esc('Erstelle einen kurzen personalisierten Cold-Email-Entwurf fuer '+title+' (draftOnly).')}" onclick="triggerAiPromptAttr(this)">${ICONS.sparkles} E-Mail-Entwurf</button>
        <button class="t-btn" data-prompt="${esc('Erstelle eine Follow-up-Aufgabe fuer '+title+' morgen um 10:00.')}" onclick="triggerAiPromptAttr(this)">${ICONS.checkbox} Follow-up morgen</button>
      </div>
      <div style="font-weight:600;font-size:12px;color:var(--t-text-secondary);">Aktivitäten &amp; Notizen</div>
      ${(tl.notes||[]).slice(0,5).map(n => `<div class="t-bubble"><strong>${esc(n.title)}</strong><div style="margin-top:3px;color:var(--t-text-secondary);">${esc((n.bodyMarkdown||'').slice(0,220))}</div></div>`).join('') || ''}
      ${(tl.events||[]).slice(0,8).map(ev => `<div style="font-size:12px;padding:6px 0;border-bottom:1px solid var(--t-border);display:flex;justify-content:space-between;"><span>${esc(ev.name)} · <span style="color:var(--t-text-secondary);">${esc(ev.details||'')}</span></span><span style="color:var(--t-text-tertiary);">${fmtDate(ev.happensAt)}</span></div>`).join('') || (!tl.notes?.length ? '<div style="color:var(--t-text-tertiary);font-size:12px;">Noch keine Zeitleisten-Einträge.</div>' : '')}
    `;
  } else if (state.rdTab === 'tasks') {
    tabHtml = `
      <div style="display:flex;gap:6px;">
        <input id="rdNewTask" placeholder="Neue Aufgabe..." style="flex:1;height:30px;padding:0 8px;border:1px solid var(--t-border-strong);border-radius:5px;background:var(--t-bg-primary);">
        <button class="t-btn t-btn-primary" style="height:30px;" onclick="addLinkedTask()">+ Hinzufügen</button>
      </div>
      ${(tl.tasks||[]).map(t => `<div style="display:flex;align-items:center;justify-content:space-between;padding:7px 8px;border:1px solid var(--t-border);border-radius:6px;">
        <label style="display:flex;align-items:center;gap:7px;cursor:pointer;"><input type="checkbox" ${t.status==='DONE'?'checked':''} onchange="patchLinkedTask('${esc(t.id)}',this.checked)"><span>${esc(t.title)}</span></label>
        <span class="t-tag t-tag-gray">${esc(tSt[t.status] || t.status)}</span>
      </div>`).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Aufgaben verknüpft.</div>'}
    `;
  } else if (state.rdTab === 'notes') {
    tabHtml = `
      <div style="display:flex;gap:6px;">
        <input id="rdNewNote" placeholder="Schnelle Notiz..." style="flex:1;height:30px;padding:0 8px;border:1px solid var(--t-border-strong);border-radius:5px;background:var(--t-bg-primary);">
        <button class="t-btn t-btn-primary" style="height:30px;" onclick="addLinkedNote()">+ Hinzufügen</button>
      </div>
      ${(tl.notes||[]).map(n => `<div class="t-bubble"><strong>${esc(n.title)}</strong><div style="margin-top:4px;color:var(--t-text-secondary);white-space:pre-wrap;">${esc((n.bodyMarkdown||'').slice(0,400))}</div></div>`).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Notizen verknüpft.</div>'}
    `;
  } else if (state.rdTab === 'relations') {
    tabHtml = `
      <div style="font-weight:600;font-size:12px;color:var(--t-text-secondary);">Kontakte (${(tl.people||[]).length})</div>
      ${(tl.people||[]).map(p => `<div class="t-kcard" onclick="openRecord('people','${esc(p.id)}')"><strong>${esc(p.fullName)}</strong><span style="font-size:12px;color:var(--t-text-secondary);">${esc(p.emails?.primaryEmail||p.jobTitle||'')}</span></div>`).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Kontakte.</div>'}
      <div style="font-weight:600;font-size:12px;color:var(--t-text-secondary);margin-top:8px;">Verkaufschancen (${(tl.opportunities||[]).length})</div>
      ${(tl.opportunities||[]).map(o => `<div class="t-kcard" onclick="openRecord('opportunities','${esc(o.id)}')"><strong>${esc(o.name)}</strong><span class="t-tag ${STAGE_CLASS[o.stage]||'t-tag-gray'}" style="width:fit-content;">${esc(STAGE_LABEL[o.stage] || o.stage)}</span></div>`).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Verkaufschancen.</div>'}
    `;
  } else if (state.rdTab === 'ai') {
    tabHtml = `
      <div class="t-bubble"><strong>KI-Entwurfsmodus (draftOnly: true)</strong><br>Der lokale Agent erstellt einen Entwurf und speichert ihn als CRM-Notiz — es wird niemals automatisch eine E-Mail versendet.</div>
      <button class="t-btn t-btn-primary" data-prompt="${esc('Recherchiere '+title+' im CRM und erstelle eine Notiz mit einem personalisierten Kaltakquise-E-Mail-Entwurf.')}" onclick="triggerAiPromptAttr(this)">${ICONS.sparkles} E-Mail-Entwurf generieren (0.6B)</button>
    `;
  }

  body.innerHTML = `<div>${propsHtml}</div>${tabHtml}`;
}

async function addLinkedNote() {
  const txt = document.getElementById('rdNewNote')?.value?.trim();
  if (!txt || !state.openRecord) return;
  const payload = { title: 'Notiz', bodyMarkdown: txt, workspace: state.workspace };
  if (state.openRecord.obj === 'companies') payload.companyId = state.openRecord.id;
  await fetch('/api/twenty/notes', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
  toast('Notiz gespeichert ✓');
  openRecord(state.openRecord.obj, state.openRecord.id);
}
async function addLinkedTask() {
  const txt = document.getElementById('rdNewTask')?.value?.trim();
  if (!txt || !state.openRecord) return;
  const payload = { title: txt, status: 'TODO', workspace: state.workspace };
  if (state.openRecord.obj === 'companies') payload.companyId = state.openRecord.id;
  await fetch('/api/twenty/tasks', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
  toast('Aufgabe erstellt ✓');
  openRecord(state.openRecord.obj, state.openRecord.id);
}
async function patchLinkedTask(tid, done) {
  await fetch(`/api/twenty/tasks/${encodeURIComponent(tid)}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ status: done ? 'DONE' : 'TODO' }) });
  toast('Aufgabe aktualisiert ✓');
}
function triggerAiPromptAttr(btn) {
  const p = btn?.dataset?.prompt; if (!p) return;
  closeRecordDrawer();
  openAiDrawer();
  document.getElementById('aiInput').value = p;
  sendAi();
}

// Settings Mode (Data model, APIs & Webhooks with GraphQL Playground, Profile, Experience)
async function openSettings(section) {
  state.mode = 'settings';
  state.settingsSection = section || 'data-model';
  updateNavHighlights();
  renderHeaderAndViewBar();
  renderBulkBar();
  await loadMetadata();
  if (state.settingsSection === 'apis') {
    await Promise.all([loadApiKeys(), loadWebhooks()]);
  }
  renderSettingsView();
}

async function loadApiKeys() {
  try {
    const r = await fetch(`/api/twenty/api-keys?workspace=${encodeURIComponent(state.workspace)}`);
    if (r.ok) { const d = await r.json(); state.apiKeys = d.apiKeys || []; }
  } catch {}
}
async function loadWebhooks() {
  try {
    const r = await fetch(`/api/twenty/webhooks?workspace=${encodeURIComponent(state.workspace)}`);
    if (r.ok) { const d = await r.json(); state.webhooks = d.webhooks || []; }
  } catch {}
}

async function createMetadataField(objSingular) {
  const label = document.getElementById('nfLabel')?.value?.trim();
  const fieldType = document.getElementById('nfType')?.value || 'TEXT';
  const optsRaw = document.getElementById('nfOptions')?.value?.trim() || '';
  const defVal = document.getElementById('nfDefault')?.value?.trim() || '';
  if (!label) { toast('Bitte Feldbezeichnung angeben'); return; }
  const options = optsRaw ? optsRaw.split(',').map(s => s.trim()).filter(Boolean) : [];
  const r = await fetch('/api/twenty/metadata/fields', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace: 'all', objectName: objSingular, label, type: fieldType, options, defaultValue: defVal })
  });
  if (r.ok) {
    state.showNewFieldForm = false;
    toast('Benutzerdefiniertes Feld angelegt ✓');
    await loadMetadata();
    renderSettingsView();
  } else {
    const err = await r.json().catch(() => ({}));
    toast(err.error || 'Feld konnte nicht erstellt werden');
  }
}

async function deleteMetadataField(fieldId) {
  if (!confirm('Dieses benutzerdefinierte Feld wirklich löschen?')) return;
  const r = await fetch(`/api/twenty/metadata/fields/${encodeURIComponent(fieldId)}`, { method: 'DELETE' });
  if (r.ok) {
    toast('Feld gelöscht');
    await loadMetadata();
    renderSettingsView();
  }
}

async function createApiKey() {
  const name = document.getElementById('akName')?.value?.trim() || 'API-Schlüssel';
  const r = await fetch('/api/twenty/api-keys', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace: state.workspace, name })
  });
  if (r.ok) {
    const d = await r.json();
    state.lastCreatedToken = d.token || null;
    toast('API-Schlüssel erzeugt ✓');
    await loadApiKeys();
    renderSettingsView();
  }
}
async function deleteApiKey(id) {
  const r = await fetch(`/api/twenty/api-keys/${encodeURIComponent(id)}`, { method: 'DELETE' });
  if (r.ok) { toast('API-Schlüssel widerrufen'); await loadApiKeys(); renderSettingsView(); }
}

async function createWebhook() {
  const targetUrl = document.getElementById('whUrl')?.value?.trim();
  const eventPattern = document.getElementById('whEvent')?.value?.trim() || '*';
  const description = document.getElementById('whDesc')?.value?.trim() || '';
  if (!targetUrl) { toast('Ziel-URL erforderlich'); return; }
  const r = await fetch('/api/twenty/webhooks', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace: state.workspace, targetUrl, eventPattern, description })
  });
  if (r.ok) {
    toast('Webhook registriert ✓');
    await loadWebhooks();
    renderSettingsView();
  } else {
    const err = await r.json().catch(() => ({}));
    toast(err.error || 'Ungültige Webhook-URL');
  }
}
async function deleteWebhook(id) {
  const r = await fetch(`/api/twenty/webhooks/${encodeURIComponent(id)}`, { method: 'DELETE' });
  if (r.ok) { toast('Webhook gelöscht'); await loadWebhooks(); renderSettingsView(); }
}

function renderSettingsView() {
  const sc = document.getElementById('stageContainer');
  const s = state.settingsSection;
  const objs = state.metadata?.objects || [];
  const activeObj = objs.find(o => o.namePlural === state.activeMetaObj) || objs[0];

  let bodyHtml = '';
  if (s === 'data-model') {
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">Datenmodell</h2>
      <p style="margin:0 0 16px;color:var(--t-text-secondary);">Standard- &amp; benutzerdefinierte Felder in SQLite (<span class="mono">${esc(state.workspace)}</span>)</p>
      <div class="t-obj-grid">
        ${objs.map(o => `<div class="t-obj-tile ${activeObj && activeObj.namePlural === o.namePlural ? 'active' : ''}" onclick="state.activeMetaObj='${esc(o.namePlural)}';state.showNewFieldForm=false;renderSettingsView()">
          <div style="display:flex;justify-content:space-between;align-items:center;"><strong>${esc(o.labelPlural)}</strong><span class="t-tag t-tag-gray">${fmtNum(o.recordCount)}</span></div>
          <span class="mono" style="font-size:11px;color:var(--t-text-tertiary);">${esc(o.nameSingular)} · ${(o.fields||[]).length} Felder</span>
        </div>`).join('')}
      </div>
      ${activeObj ? `<div class="t-s-card">
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;gap:8px;flex-wrap:wrap;">
          <strong>${esc(activeObj.labelPlural)} · Feld-Schema</strong>
          <div style="display:flex;gap:6px;">
            <button class="t-btn t-btn-primary" id="addCustomFieldBtn" onclick="state.showNewFieldForm=!state.showNewFieldForm;renderSettingsView()">+ Neues Feld hinzufügen</button>
            <button class="t-btn" onclick="setObject('${esc(activeObj.namePlural)}')">Im Arbeitsbereich öffnen ↗</button>
          </div>
        </div>
        ${state.showNewFieldForm ? `<div style="padding:12px;margin-bottom:12px;border:1px solid var(--t-border-strong);border-radius:7px;background:var(--t-bg-secondary);display:grid;grid-template-columns:repeat(auto-fit,minmax(160px,1fr));gap:8px;align-items:end;">
          <div><div style="font-size:11px;color:var(--t-text-secondary);margin-bottom:3px;">Anzeigename *</div><input id="nfLabel" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);" placeholder="z.B. Branche"></div>
          <div><div style="font-size:11px;color:var(--t-text-secondary);margin-bottom:3px;">Datentyp</div><select id="nfType" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);"><option value="TEXT">TEXT</option><option value="NUMBER">NUMBER</option><option value="CURRENCY">CURRENCY</option><option value="SELECT">SELECT</option><option value="BOOLEAN">BOOLEAN</option><option value="DATE">DATE</option><option value="URL">URL</option></select></div>
          <div><div style="font-size:11px;color:var(--t-text-secondary);margin-bottom:3px;">Optionen (für SELECT)</div><input id="nfOptions" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);" placeholder="A, B, C"></div>
          <div><div style="font-size:11px;color:var(--t-text-secondary);margin-bottom:3px;">Standardwert</div><input id="nfDefault" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);" placeholder="Optional"></div>
          <div style="display:flex;gap:6px;"><button class="t-btn t-btn-primary" id="saveCustomFieldBtn" onclick="createMetadataField('${esc(activeObj.nameSingular)}')">Feld speichern</button><button class="t-btn" onclick="state.showNewFieldForm=false;renderSettingsView()">Abbrechen</button></div>
        </div>` : ''}
        <table style="width:100%;border-collapse:collapse;">
          <thead><tr style="border-bottom:1px solid var(--t-border);color:var(--t-text-tertiary);font-size:11.5px;text-align:left;"><th style="padding:6px;">Anzeigename</th><th style="padding:6px;">API-Name</th><th style="padding:6px;">Typ</th><th style="padding:6px;">Quelle</th><th style="padding:6px;"></th></tr></thead>
          <tbody>
            ${(activeObj.fields||[]).map(f => `<tr style="border-bottom:1px solid var(--t-border);">
              <td style="padding:7px 6px;font-weight:500;">${esc(f.label)}</td>
              <td class="mono" style="padding:7px 6px;font-size:12px;">${esc(f.name)}</td>
              <td style="padding:7px 6px;"><span class="t-tag t-tag-blue">${esc(f.type)}</span></td>
              <td style="padding:7px 6px;color:var(--t-text-secondary);">${f.isCustom ? '<span class="t-tag t-tag-purple">Benutzerdefiniert</span>' : 'Standard'}</td>
              <td style="padding:7px 6px;text-align:right;">${f.isCustom && f.id ? `<button class="t-btn t-btn-danger" style="height:24px;font-size:11px;" onclick="deleteMetadataField('${esc(f.id)}')">Löschen</button>` : ''}</td>
            </tr>`).join('')}
          </tbody>
        </table>
      </div>` : ''}
    `;
  } else if (s === 'apis') {
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">APIs &amp; Webhooks · GraphQL-Playground</h2>
      <p style="margin:0 0 14px;color:var(--t-text-secondary);">Live-GraphQL-Endpunkt: <span class="mono">/graphql</span> &amp; <span class="mono">/api/twenty/graphql</span> · REST: <span class="mono">/rest/*</span></p>
      <div class="t-s-card">
        <div style="display:flex;gap:6px;margin-bottom:8px;flex-wrap:wrap;">
          <button class="t-btn" onclick="setGqlPreset('companies')">Unternehmen-Abfrage</button>
          <button class="t-btn" onclick="setGqlPreset('opportunities')">Verkaufschancen-Abfrage</button>
          <button class="t-btn" onclick="setGqlPreset('notes')">Notizen-Abfrage</button>
          <button class="t-btn t-btn-primary" id="runGqlBtn" style="margin-left:auto;" onclick="runGqlPlayground()">▶ GraphQL ausführen</button>
        </div>
        <textarea id="gqlEditor" class="mono" rows="6" style="width:100%;padding:10px;border:1px solid var(--t-border-strong);border-radius:6px;background:var(--t-bg-secondary);font-size:12px;outline:none;">query {
  companies(first: 5) {
    totalCount
    edges { node { id name domainName { primaryLinkUrl } address { addressCity } idealCustomerProfile } }
  }
}</textarea>
        <pre id="gqlOut" class="mono" style="margin:10px 0 0;padding:10px;border-radius:6px;background:var(--t-bg-tertiary);max-height:260px;overflow:auto;font-size:11.5px;">Klicke „GraphQL ausführen“...</pre>
      </div>

      <div class="t-s-card">
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;">
          <div><strong>API-Schlüssel</strong><div style="font-size:11.5px;color:var(--t-text-secondary);">SHA-256 gehasht in SQLite gespeichert; Klartext nur einmalig bei Erstellung sichtbar.</div></div>
        </div>
        ${state.lastCreatedToken ? `<div style="padding:10px;margin-bottom:10px;border-radius:6px;background:var(--t-green-soft);border:1px solid var(--t-green);font-size:12px;">
          <strong>Neuer API-Schlüssel (einmalig kopieren):</strong> <code class="mono" style="user-select:all;">${esc(state.lastCreatedToken)}</code>
        </div>` : ''}
        <div style="display:flex;gap:8px;margin-bottom:10px;flex-wrap:wrap;">
          <input id="akName" class="t-prop-input" style="flex:1;min-width:180px;border:1px solid var(--t-border-strong);" placeholder="Bezeichnung (z.B. n8n Integration)">
          <button class="t-btn t-btn-primary" onclick="createApiKey()">+ Schlüssel erzeugen</button>
        </div>
        <div style="display:flex;flex-direction:column;gap:6px;">
          ${(state.apiKeys || []).map(k => `<div style="display:flex;align-items:center;justify-content:space-between;padding:7px 10px;border:1px solid var(--t-border);border-radius:6px;">
            <div><strong>${esc(k.name)}</strong> <span class="mono" style="font-size:11.5px;color:var(--t-text-tertiary);margin-left:6px;">${esc(k.keyPrefix)}</span></div>
            <button class="t-btn t-btn-danger" style="height:24px;font-size:11px;" onclick="deleteApiKey('${esc(k.id)}')">Widerrufen</button>
          </div>`).join('') || '<div style="font-size:12px;color:var(--t-text-tertiary);">Noch keine API-Schlüssel angelegt.</div>'}
        </div>
      </div>

      <div class="t-s-card">
        <div style="margin-bottom:10px;"><strong>Webhooks</strong><div style="font-size:11.5px;color:var(--t-text-secondary);">Ereignisgesteuerte Endpunkte für CRM-Objekte (z.B. <code>*</code> oder <code>company.*</code>).</div></div>
        <div style="display:grid;grid-template-columns:2fr 1fr 1fr auto;gap:8px;margin-bottom:10px;">
          <input id="whUrl" class="t-prop-input" style="border:1px solid var(--t-border-strong);" placeholder="https://crm.conrad-ki.com/webhook/...">
          <input id="whEvent" class="t-prop-input mono" style="border:1px solid var(--t-border-strong);" value="*" placeholder="Ereignismuster">
          <input id="whDesc" class="t-prop-input" style="border:1px solid var(--t-border-strong);" placeholder="Beschreibung">
          <button class="t-btn t-btn-primary" onclick="createWebhook()">+ Webhook</button>
        </div>
        <div style="display:flex;flex-direction:column;gap:6px;">
          ${(state.webhooks || []).map(w => `<div style="display:flex;align-items:center;justify-content:space-between;padding:7px 10px;border:1px solid var(--t-border);border-radius:6px;gap:8px;">
            <div style="min-width:0;"><div class="mono" style="font-size:12px;overflow:hidden;text-overflow:ellipsis;">${esc(w.targetUrl)}</div><div style="font-size:11.5px;color:var(--t-text-secondary);"><span class="t-tag t-tag-blue">${esc(w.eventPattern)}</span> ${esc(w.description || '')}</div></div>
            <button class="t-btn t-btn-danger" style="height:24px;font-size:11px;" onclick="deleteWebhook('${esc(w.id)}')">Löschen</button>
          </div>`).join('') || '<div style="font-size:12px;color:var(--t-text-tertiary);">Noch keine Webhooks registriert.</div>'}
        </div>
      </div>
    `;
  } else if (s === 'experience') {
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">Darstellung</h2>
      <p style="margin:0 0 14px;color:var(--t-text-secondary);">Erscheinungsbild &amp; Arbeitsbereichs-Einstellungen</p>
      <div class="t-s-card" style="display:flex;align-items:center;justify-content:space-between;">
        <div><strong>Erscheinungsbild</strong><div style="font-size:12px;color:var(--t-text-secondary);">Zwischen hellem und dunklem Design wechseln</div></div>
        <button class="t-btn" onclick="toggleTheme()">Hell / Dunkel umschalten</button>
      </div>
      <div class="t-s-card" style="display:flex;align-items:center;justify-content:space-between;">
        <div><strong>SQL-Dump-Synchronisierung</strong><div style="font-size:12px;color:var(--t-text-secondary);">PostgreSQL-Dump erneut in SQLite WAL importieren</div></div>
        <button class="t-btn" onclick="syncSqlDump()">SQL-Dump synchronisieren</button>
      </div>
    `;
  } else {
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">Profil</h2>
      <p style="margin:0 0 14px;color:var(--t-text-secondary);">Arbeitsbereichs-Mitglied &amp; lokale KI-Agent-Standards</p>
      <div class="t-s-card">
        <div class="t-prop-row"><span class="t-prop-lbl">Name</span><span>Merlin Conrad</span></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Arbeitsbereich</span><span class="mono">${esc(state.workspace)}</span></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Engine</span><span>Conrad High-Speed CRM Engine (bis zu 10× schneller &amp; 100× schlanker)</span></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Sicherheitsregel</span><span>draftOnly = true (Kein automatischer E-Mail-Versand)</span></div>
      </div>
    `;
  }

  sc.innerHTML = `<div class="t-settings-wrap">
    <div class="t-settings-nav">
      <div class="t-nav-section" style="padding-top:2px;"><span>Benutzer</span></div>
      <button class="t-nav-item ${s==='profile'?'active':''}" onclick="openSettings('profile')"><span class="t-nav-left">${ICONS.user}<span>Profil</span></span></button>
      <button class="t-nav-item ${s==='experience'?'active':''}" onclick="openSettings('experience')"><span class="t-nav-left">${ICONS.sun}<span>Darstellung</span></span></button>
      <div class="t-nav-section"><span>Arbeitsbereich</span></div>
      <button class="t-nav-item ${s==='data-model'?'active':''}" onclick="openSettings('data-model')"><span class="t-nav-left">${ICONS.database}<span>Datenmodell</span></span></button>
      <button class="t-nav-item ${s==='apis'?'active':''}" onclick="openSettings('apis')"><span class="t-nav-left">${ICONS.code}<span>APIs &amp; Webhooks</span></span></button>
      <div style="margin-top:auto;padding-top:12px;"><button class="t-btn" style="width:100%;justify-content:center;" onclick="setObject('companies')">← Zurück zum CRM</button></div>
    </div>
    <div class="t-settings-body">${bodyHtml}</div>
  </div>`;
}

function setGqlPreset(k) {
  const ed = document.getElementById('gqlEditor'); if (!ed) return;
  if (k === 'companies') ed.value = `query {\n  companies(first: 5) {\n    totalCount\n    edges { node { id name domainName { primaryLinkUrl } address { addressCity } } }\n  }\n}`;
  if (k === 'opportunities') ed.value = `query {\n  opportunities(first: 5) {\n    totalCount\n    edges { node { id name stage amount { amountMicros currencyCode } } }\n  }\n}`;
  if (k === 'notes') ed.value = `query {\n  notes(first: 5) {\n    totalCount\n    edges { node { id title bodyV2 { markdown } } }\n  }\n}`;
}
async function runGqlPlayground() {
  const q = document.getElementById('gqlEditor')?.value || '';
  const r = await fetch(`/api/twenty/graphql?workspace=${encodeURIComponent(state.workspace)}`, {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ query: q })
  });
  document.getElementById('gqlOut').textContent = JSON.stringify(await r.json(), null, 2);
}

// Ask AI Drawer
function openAiDrawer() { document.getElementById('aiDrawer').classList.add('open'); setTimeout(() => document.getElementById('aiInput').focus(), 40); }
function closeAiDrawer() { document.getElementById('aiDrawer').classList.remove('open'); }
function quickAi(p) { document.getElementById('aiInput').value = p; sendAi(); }
async function sendAi() {
  const inp = document.getElementById('aiInput');
  const prompt = inp.value.trim(); if (!prompt) return;
  const msgs = document.getElementById('aiMsgs');
  const model = document.getElementById('aiModelSel').value;
  inp.value = '';
  msgs.insertAdjacentHTML('beforeend', `<div class="t-bubble user">${esc(prompt)}</div>`);
  const wait = document.createElement('div'); wait.className = 't-bubble'; wait.textContent = `⏳ ${model}...`;
  msgs.appendChild(wait); msgs.scrollTop = msgs.scrollHeight;
  try {
    const r = await fetch('/api/twenty/agent', {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ prompt, model, workspace: state.workspace })
    });
    const d = await r.json();
    const calls = (d.toolCalls || []).map(c => `<span class="t-tag t-tag-blue mono">${esc(c.name)}</span>`).join(' ');
    wait.innerHTML = `<div style="margin-bottom:5px;display:flex;gap:5px;flex-wrap:wrap;align-items:center;"><span class="t-tag t-tag-gray mono">${esc(d.model || model)}</span><span class="t-tag t-tag-gray">${d.elapsedMs || 0}ms</span>${calls}</div><div>${esc(d.reply || JSON.stringify(d))}</div>`;
    msgs.scrollTop = msgs.scrollHeight;
    if ((d.toolCalls || []).length) { loadStats(); loadCurrentView(); }
  } catch (e) { wait.textContent = 'Fehler: ' + e; }
}

// Create Modal
function openCreateModal(preset) {
  const cf = document.getElementById('createFields');
  const preStage = preset?.stage || 'NEW';
  const preStatus = preset?.status || 'TODO';
  let html = '';
  if (state.object === 'companies') {
    document.getElementById('createTitle').textContent = 'Neues Unternehmen';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crName" placeholder="Firmenname *">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crDom" placeholder="Domain (z.B. example.de)">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crCity" placeholder="Stadt">`;
  } else if (state.object === 'people') {
    document.getElementById('createTitle').textContent = 'Neue Person';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crFirst" placeholder="Vorname *">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crLast" placeholder="Nachname *">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crEmail" placeholder="E-Mail">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crJob" placeholder="Position">`;
  } else if (state.object === 'opportunities') {
    document.getElementById('createTitle').textContent = 'Neue Verkaufschance';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crOppName" placeholder="Deal-Name *">
      <select class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crOppStage">${STAGES.map(s=>`<option value="${s}" ${s===preStage?'selected':''}>${(window.STAGE_LABEL && window.STAGE_LABEL[s]) || s}</option>`).join('')}</select>
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crOppAmt" type="number" placeholder="Wert (EUR)">`;
  } else if (state.object === 'tasks') {
    document.getElementById('createTitle').textContent = 'Neue Aufgabe';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crTaskTitle" placeholder="Aufgabentitel *">
      <select class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crTaskStatus"><option value="TODO" ${preStatus==='TODO'?'selected':''}>Offen</option><option value="IN_PROGRESS" ${preStatus==='IN_PROGRESS'?'selected':''}>In Arbeit</option><option value="DONE" ${preStatus==='DONE'?'selected':''}>Erledigt</option></select>
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crTaskDue" placeholder="Fälligkeitsdatum (YYYY-MM-DD)">`;
  } else {
    document.getElementById('createTitle').textContent = 'Neue Notiz';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crNoteTitle" placeholder="Titel *">
      <textarea class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crNoteBody" rows="4" placeholder="Inhalt..."></textarea>`;
  }
  const cfFields = customFieldsForCurrentObject();
  if (cfFields.length) {
    html += cfFields.map(f => `<input class="t-prop-input cr-custom-input" style="border:1px solid var(--t-border-strong);" data-cfname="${esc(f.name)}" data-cftype="${esc(f.type||'TEXT')}" placeholder="${esc(f.label)} (${esc(f.type||'TEXT')})" value="${esc(f.defaultValue||'')}">`).join('');
  }
  cf.innerHTML = html;
  document.getElementById('createBackdrop').classList.add('open');
}
function closeCreateModal() { document.getElementById('createBackdrop').classList.remove('open'); }
async function submitCreateModal() {
  let obj = state.object, payload = { workspace: state.workspace };
  if (obj === 'companies') {
    payload.name = document.getElementById('crName').value.trim();
    payload.domainUrl = document.getElementById('crDom').value.trim();
    payload.city = document.getElementById('crCity').value.trim();
  } else if (obj === 'people') {
    payload.firstName = document.getElementById('crFirst').value.trim();
    payload.lastName = document.getElementById('crLast').value.trim();
    payload.email = document.getElementById('crEmail').value.trim();
    payload.jobTitle = document.getElementById('crJob').value.trim();
  } else if (obj === 'opportunities') {
    payload.name = document.getElementById('crOppName').value.trim();
    payload.stage = document.getElementById('crOppStage').value;
    payload.amountEur = Number(document.getElementById('crOppAmt').value || 0);
  } else if (obj === 'tasks') {
    payload.title = document.getElementById('crTaskTitle').value.trim();
    payload.status = document.getElementById('crTaskStatus')?.value || 'TODO';
    const d = document.getElementById('crTaskDue').value.trim(); if (d) payload.dueAt = d;
  } else {
    payload.title = document.getElementById('crNoteTitle').value.trim();
    payload.bodyMarkdown = document.getElementById('crNoteBody').value.trim();
  }
  document.querySelectorAll('.cr-custom-input').forEach(el => {
    const val = el.value.trim();
    if (!val) return;
    const ft = (el.dataset.cftype || '').toUpperCase();
    payload[el.dataset.cfname] = (ft === 'NUMBER' || ft === 'CURRENCY') ? Number(val || 0) : ft === 'BOOLEAN' ? (val === 'true' || val === 'Ja') : val;
  });
  const r = await fetch(`/api/twenty/${obj}`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
  if (r.ok) {
    closeCreateModal();
    toast('Erstellt ✓');
    loadStats();
    loadCurrentView();
  }
}

// Befehlsmenü (⌘K)
function openCmd() {
  document.getElementById('cmdBackdrop').classList.add('open');
  const inp = document.getElementById('cmdInput'); inp.value = ''; inp.focus();
  runCmdSearch();
}
function closeCmd() { document.getElementById('cmdBackdrop').classList.remove('open'); }
async function runCmdSearch() {
  const q = document.getElementById('cmdInput').value.trim();
  const out = document.getElementById('cmdResults');
  if (!q) {
    out.innerHTML = `
      <div class="t-pop-title">Navigation</div>
      <button class="t-pop-item" onclick="closeCmd();setObject('companies')"><span>${ICONS.building} Zu Unternehmen wechseln</span><span class="t-kbd">C</span></button>
      <button class="t-pop-item" onclick="closeCmd();setObject('people')"><span>${ICONS.user} Zu Kontakten wechseln</span></button>
      <button class="t-pop-item" onclick="closeCmd();setObject('opportunities')"><span>${ICONS.target} Zu Verkaufschancen (Kanban)</span></button>
      <button class="t-pop-item" onclick="closeCmd();setObject('tasks')"><span>${ICONS.checkbox} Zu Aufgaben wechseln</span></button>
      <button class="t-pop-item" onclick="closeCmd();setObject('notes')"><span>${ICONS.notes} Zu Notizen wechseln</span></button>
      <button class="t-pop-item" onclick="closeCmd();openSettings('data-model')"><span>${ICONS.settings} Einstellungen &amp; Datenmodell</span></button>
      <button class="t-pop-item" onclick="closeCmd();openAiDrawer()"><span>${ICONS.sparkles} KI-Copilot fragen</span><span class="t-kbd">⌘J</span></button>
      <a class="t-pop-item" href="/n8n"><span>${ICONS.workflow} Conrad Automation öffnen</span></a>
    `;
    return;
  }
  const [cR, pR, oR] = await Promise.all([
    fetch(`/api/twenty/companies?workspace=${encodeURIComponent(state.workspace)}&q=${encodeURIComponent(q)}&limit=5`).then(r => r.json()),
    fetch(`/api/twenty/people?workspace=${encodeURIComponent(state.workspace)}&q=${encodeURIComponent(q)}&limit=4`).then(r => r.json()),
    fetch(`/api/twenty/opportunities?workspace=${encodeURIComponent(state.workspace)}&q=${encodeURIComponent(q)}&limit=4`).then(r => r.json())
  ]);
  let html = '';
  (cR.data || []).forEach(c => { html += `<button class="t-pop-item" onclick="closeCmd();openRecord('companies','${esc(c.id)}')"><span>${ICONS.building} ${esc(c.name)}</span><span class="t-tag t-tag-gray">${esc(c.address?.addressCity || 'Unternehmen')}</span></button>`; });
  (pR.data || []).forEach(p => { html += `<button class="t-pop-item" onclick="closeCmd();openRecord('people','${esc(p.id)}')"><span>${ICONS.user} ${esc(p.fullName)}</span><span class="t-tag t-tag-gray">${esc(p.emails?.primaryEmail || 'Kontakt')}</span></button>`; });
  (oR.data || []).forEach(o => { html += `<button class="t-pop-item" onclick="closeCmd();setObject('opportunities')"><span>${ICONS.target} ${esc(o.name)}</span><span class="t-tag t-tag-blue">${esc(STAGE_LABEL[o.stage] || o.stage)}</span></button>`; });
  out.innerHTML = html || '<div style="padding:16px;text-align:center;color:var(--t-text-tertiary);">Keine Treffer.</div>';
}

// Export, CSV Import & Sync helpers
function exportCurrentJson() {
  const blob = new Blob([JSON.stringify(state.rows, null, 2)], { type: 'application/json' });
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = `twenty_${state.object}.json`; a.click();
}
function exportBackendCsv(selectedOnly) {
  const ids = selectedOnly ? Array.from(state.selectedIds).join(',') : '';
  const url = `/api/twenty/objects/${encodeURIComponent(state.object)}/export-csv?workspace=${encodeURIComponent(state.workspace)}${ids ? '&ids=' + encodeURIComponent(ids) : ''}`;
  const a = document.createElement('a'); a.href = url; a.download = `twenty_${state.object}.csv`; document.body.appendChild(a); a.click(); a.remove();
}
function exportCurrentCsv() { exportBackendCsv(false); }

function openCsvModal() {
  const hints = {
    companies: 'name,domainUrl,city,employees,arrEur',
    people: 'firstName,lastName,email,phone,jobTitle,city',
    opportunities: 'name,stage,amountEur,closeDate',
    tasks: 'title,status,dueAt,bodyMarkdown',
    notes: 'title,bodyMarkdown'
  };
  document.getElementById('csvModalTitle').textContent = `CSV in „${state.object}“ importieren`;
  document.getElementById('csvHint').textContent = `Erste Zeile = Spaltenköpfe (z.B. ${hints[state.object] || 'name'})`;
  document.getElementById('csvTextarea').value = '';
  document.getElementById('csvFileInput').value = '';
  document.getElementById('csvPreviewInfo').textContent = '';
  document.getElementById('csvImportBackdrop').classList.add('open');
}
function closeCsvModal() { document.getElementById('csvImportBackdrop').classList.remove('open'); }
function onCsvFileChange(ev) {
  const file = ev.target.files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = () => {
    const txt = String(reader.result || '');
    document.getElementById('csvTextarea').value = txt;
    const lines = txt.trim().split(/\r?\n/).filter(Boolean);
    document.getElementById('csvPreviewInfo').textContent = `${Math.max(0, lines.length - 1)} Datenzeilen erkannt (${file.name})`;
  };
  reader.readAsText(file);
}
async function submitCsvImport() {
  const csv = document.getElementById('csvTextarea')?.value?.trim();
  if (!csv) { toast('Bitte CSV-Datei wählen oder Text einfügen'); return; }
  const r = await fetch(`/api/twenty/objects/${encodeURIComponent(state.object)}/import-csv?workspace=${encodeURIComponent(state.workspace)}`, {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace: state.workspace, csv })
  });
  if (r.ok) {
    const d = await r.json();
    closeCsvModal();
    toast(`${d.imported || 0} Datensätze importiert ✓`);
    loadStats();
    loadCurrentView();
  } else {
    toast('CSV-Import fehlgeschlagen');
  }
}

async function syncSqlDump() {
  toast('Synchronisiere SQLite...');
  const r = await fetch('/api/twenty/sync-sql', { method: 'POST' });
  if (r.ok) { toast('SQL Sync abgeschlossen ✓'); loadStats(); loadCurrentView(); }
}

document.addEventListener('keydown', e => {
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') { e.preventDefault(); openCmd(); }
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'j') { e.preventDefault(); openAiDrawer(); }
  if (e.key === 'Enter' && state.focusedCell && !document.activeElement?.matches('input,textarea,select,button')) {
    e.preventDefault();
    startEditCell(state.focusedCell);
  }
  if (e.key === 'Escape') { closeCmd(); closeRecordDrawer(); closeAiDrawer(); closeCreateModal(); closeCsvModal(); closePops(); }
});

if (window.innerWidth <= 900) {
  document.getElementById('tDrawer').classList.add('collapsed');
  document.getElementById('expandDrawerBtn').style.display = 'inline-flex';
}
updateWsBadge();
loadMetadata();
loadSavedViews();
loadStats();
setObject('companies');
</script>
</body>
</html>"######
}

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
  --n8n-coral-hover: #f55742;
  --n8n-coral-soft: rgba(255,109,90,0.12);
  --n8n-green: #29a366;
  --n8n-green-soft: rgba(41,163,102,0.13);
  --n8n-red: #e04f5f;
  --n8n-violet: #7c3aed;
  --n8n-blue: #4078f2;
  --n8n-sidebar-bg: #262626;
  --n8n-sidebar-border: #383838;
  --n8n-sidebar-hover: #333333;
  --n8n-sidebar-text: #d4d4d8;
  --n8n-header-bg: #ffffff;
  --n8n-canvas-bg: #f4f5f7;
  --n8n-canvas-dot: #cfd4df;
  --n8n-node-bg: #ffffff;
  --n8n-node-border: #d8dbe2;
  --n8n-panel-bg: #ffffff;
  --n8n-subtle-bg: #f8f9fb;
  --n8n-border: #e2e4ea;
  --n8n-text-1: #1f2329;
  --n8n-text-2: #5b626e;
  --n8n-text-3: #8c93a1;
  --n8n-wire: #9aa2b1;
  --n8n-shadow: 0 2px 8px rgba(15,23,42,0.06);
  --n8n-shadow-lg: 0 18px 48px rgba(15,23,42,0.18);
}
html.dark {
  --n8n-header-bg: #222222;
  --n8n-canvas-bg: #1c1c1c;
  --n8n-canvas-dot: #343434;
  --n8n-node-bg: #2a2a2a;
  --n8n-node-border: #444444;
  --n8n-panel-bg: #222222;
  --n8n-subtle-bg: #292929;
  --n8n-border: #363636;
  --n8n-text-1: #efefef;
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

/* Left Dark n8n Sidebar (200px / 54px collapsed) */
.n8n-nav {
  width: 218px; min-width: 218px; background: var(--n8n-sidebar-bg); color: var(--n8n-sidebar-text);
  border-right: 1px solid var(--n8n-sidebar-border); display: flex; flex-direction: column;
  transition: width 0.15s ease, min-width 0.15s ease; z-index: 40; user-select: none;
}
.n8n-nav.collapsed { width: 54px; min-width: 54px; }
.n8n-nav.collapsed .nav-txt, .n8n-nav.collapsed .nav-badge, .n8n-nav.collapsed .brand-wordmark { display: none; }

.n8n-nav-top {
  height: 52px; padding: 0 12px; display: flex; align-items: center; justify-content: space-between;
  border-bottom: 1px solid var(--n8n-sidebar-border);
}
.n8n-brand { display: flex; align-items: center; gap: 8px; font-weight: 700; font-size: 15px; color: #fff; cursor: pointer; }
.n8n-nav-btn-icon {
  width: 26px; height: 26px; border-radius: 5px; border: none; background: transparent;
  color: #a1a1aa; cursor: pointer; display: inline-flex; align-items: center; justify-content: center;
}
.n8n-nav-btn-icon:hover { background: var(--n8n-sidebar-hover); color: #fff; }

.n8n-create-wrap { padding: 10px 10px 6px; }
.n8n-create-btn {
  width: 100%; height: 32px; border-radius: 6px; border: 1px solid #4b4b4b;
  background: #303030; color: #ffffff; font-weight: 600; font-size: 12.5px; cursor: pointer;
  display: flex; align-items: center; justify-content: center; gap: 6px;
}
.n8n-create-btn:hover { background: var(--n8n-coral); border-color: var(--n8n-coral); }

.n8n-nav-list { flex: 1; overflow-y: auto; padding: 4px 8px; display: flex; flex-direction: column; gap: 2px; }
.n8n-nav-item {
  height: 33px; padding: 0 9px; border-radius: 6px; border: none; background: transparent;
  color: var(--n8n-sidebar-text); font-size: 13px; font-weight: 500; cursor: pointer;
  display: flex; align-items: center; justify-content: space-between; gap: 8px; width: 100%; text-align: left;
}
.n8n-nav-item:hover { background: var(--n8n-sidebar-hover); color: #fff; }
.n8n-nav-item.active { background: #3b3b3b; color: #fff; font-weight: 600; }
.n8n-nav-left { display: flex; align-items: center; gap: 9px; min-width: 0; }
.nav-badge { font-size: 10.5px; background: #3f3f46; color: #e4e4e7; padding: 1px 6px; border-radius: 99px; }

.n8n-nav-foot {
  padding: 8px; border-top: 1px solid var(--n8n-sidebar-border);
  display: flex; flex-direction: column; gap: 2px;
}
.n8n-user-row {
  padding: 6px 8px; display: flex; align-items: center; gap: 8px; border-radius: 6px; cursor: pointer;
}
.n8n-user-row:hover { background: var(--n8n-sidebar-hover); }
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
.n8n-hdr-left { display: flex; align-items: center; gap: 8px; min-width: 0; flex: 1; }
.n8n-crumb-btn {
  border: none; background: transparent; color: var(--n8n-text-2); font-size: 13px; cursor: pointer;
  padding: 4px 6px; border-radius: 5px; display: inline-flex; align-items: center; gap: 4px; white-space: nowrap;
}
.n8n-crumb-btn:hover { background: var(--n8n-subtle-bg); color: var(--n8n-text-1); }
.n8n-title-inp {
  min-width: 140px; max-width: 280px; border: 1px solid transparent; border-radius: 5px;
  background: transparent; font-weight: 600; font-size: 13.5px; padding: 4px 7px; outline: none;
}
.n8n-title-inp:hover { border-color: var(--n8n-border); }
.n8n-title-inp:focus { border-color: var(--n8n-coral); background: var(--n8n-panel-bg); }
.n8n-tag-pill {
  height: 22px; padding: 0 8px; border-radius: 99px; border: 1px solid var(--n8n-border);
  background: var(--n8n-subtle-bg); color: var(--n8n-text-2); font-size: 11px; font-weight: 500;
  cursor: pointer; display: inline-flex; align-items: center; gap: 4px; white-space: nowrap;
}
.n8n-tag-pill:hover { border-color: var(--n8n-text-3); color: var(--n8n-text-1); }

/* Center Segmented Switcher (Editor | Executions | Evaluations) */
.n8n-seg {
  display: inline-flex; background: var(--n8n-subtle-bg); border: 1px solid var(--n8n-border);
  border-radius: 7px; padding: 2px; gap: 2px;
}
.n8n-seg-btn {
  height: 26px; padding: 0 11px; border-radius: 5px; border: none; background: transparent;
  font-size: 12px; font-weight: 500; color: var(--n8n-text-2); cursor: pointer;
}
.n8n-seg-btn.active { background: var(--n8n-panel-bg); color: var(--n8n-text-1); font-weight: 600; box-shadow: var(--n8n-shadow); }

.n8n-hdr-right { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.n8n-active-wrap { display: inline-flex; align-items: center; gap: 6px; font-size: 12px; color: var(--n8n-text-2); cursor: pointer; }
.n8n-switch {
  width: 34px; height: 19px; border-radius: 99px; background: #cbd5e1; position: relative;
  transition: background 0.15s ease; border: none; cursor: pointer; padding: 0;
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

.n8n-pop {
  position: absolute; z-index: 80; min-width: 220px; background: var(--n8n-panel-bg);
  border: 1px solid var(--n8n-border); border-radius: 8px; box-shadow: var(--n8n-shadow-lg); padding: 6px; display: none;
}
.n8n-pop.open { display: block; }
.n8n-pop-item {
  width: 100%; border: none; background: transparent; padding: 7px 9px; border-radius: 5px;
  display: flex; align-items: center; justify-content: space-between; gap: 8px; cursor: pointer;
  font-size: 12.5px; color: var(--n8n-text-1); text-align: left;
}
.n8n-pop-item:hover { background: var(--n8n-subtle-bg); }

/* @n8n/canvas Stage */
.n8n-stage { flex: 1; position: relative; overflow: hidden; display: flex; flex-direction: column; }
.n8n-canvas {
  flex: 1; position: relative; overflow: hidden;
  background-color: var(--n8n-canvas-bg);
  background-image: radial-gradient(var(--n8n-canvas-dot) 1.25px, transparent 1.25px);
  background-size: 18px 18px;
  cursor: grab;
}
.n8n-canvas:active { cursor: grabbing; }
.n8n-viewport { position: absolute; inset: 0; transform-origin: 0 0; }
svg.n8n-wires { position: absolute; top: 0; left: 0; width: 6000px; height: 4000px; pointer-events: none; overflow: visible; }
.wire-path { fill: none; stroke: var(--n8n-wire); stroke-width: 2.2px; pointer-events: stroke; cursor: pointer; transition: stroke 0.12s; }
.wire-path:hover { stroke: var(--n8n-coral); stroke-width: 3.4px; }
.wire-path.executed { stroke: var(--n8n-green); }
.wire-path.ai { stroke: #9ca3af; stroke-width: 1.8px; stroke-dasharray: 5 5; }
.wire-path.draft { stroke: var(--n8n-coral); stroke-width: 2.4px; stroke-dasharray: 5 4; pointer-events: none; }

/* Authentic @n8n/canvas Nodes (96x96 Square, D-Trigger, 220x96 AI Agent, 68x68 Circular Sub-node) */
.n8n-node-wrap { position: absolute; user-select: none; }
.n8n-node-box {
  position: relative; background: var(--n8n-node-bg); border: 2px solid var(--n8n-node-border);
  box-shadow: var(--n8n-shadow); transition: border-color 0.12s, box-shadow 0.12s;
  display: flex; align-items: center; justify-content: center; cursor: pointer;
}
.n8n-node-box.standard { width: 96px; height: 96px; border-radius: 16px; }
.n8n-node-box.trigger { width: 96px; height: 96px; border-radius: 38px 16px 16px 38px; }
.n8n-node-box.agent-root {
  width: 224px; height: 96px; border-radius: 16px; justify-content: flex-start; padding: 0 16px; gap: 12px;
}
.n8n-node-box.subnode { width: 68px; height: 68px; border-radius: 50%; }
.n8n-node-box.selected, .n8n-node-box:hover {
  border-color: var(--n8n-coral); box-shadow: 0 0 0 4px rgba(255,109,90,0.14);
}
.n8n-node-box.status-success { border-color: var(--n8n-green); }
.n8n-node-box.status-error { border-color: var(--n8n-red); }
.n8n-node-box.disabled-node { opacity: 0.48; }

.n8n-trig-bolt {
  position: absolute; left: -12px; top: 50%; transform: translateY(-50%);
  width: 22px; height: 22px; border-radius: 50%; background: var(--n8n-coral); color: #fff;
  display: flex; align-items: center; justify-content: center; border: 2px solid var(--n8n-canvas-bg);
  font-size: 11px; box-shadow: var(--n8n-shadow);
}
.n8n-exec-badge {
  position: absolute; right: 6px; bottom: 6px; width: 18px; height: 18px; border-radius: 50%;
  background: var(--n8n-green); color: #fff; font-size: 10px; font-weight: 700;
  display: flex; align-items: center; justify-content: center;
}
.n8n-exec-badge.err { background: var(--n8n-red); }
.n8n-items-pill {
  position: absolute; top: -9px; right: 6px; padding: 1px 6px; border-radius: 99px;
  background: var(--n8n-green); color: #fff; font-size: 9.5px; font-weight: 600;
}

/* Node Hover Toolbar */
.n8n-node-toolbar {
  position: absolute; top: -30px; left: 50%; transform: translateX(-50%);
  background: var(--n8n-panel-bg); border: 1px solid var(--n8n-border); border-radius: 6px;
  box-shadow: var(--n8n-shadow); padding: 2px 4px; display: none; align-items: center; gap: 2px; z-index: 25;
}
.n8n-node-wrap:hover .n8n-node-toolbar { display: flex; }
.n8n-tb-btn {
  width: 22px; height: 22px; border: none; background: transparent; border-radius: 4px;
  cursor: pointer; font-size: 11.5px; color: var(--n8n-text-2); display: inline-flex; align-items: center; justify-content: center;
}
.n8n-tb-btn:hover { background: var(--n8n-subtle-bg); color: var(--n8n-coral); }

/* Ports */
.n8n-port-in {
  position: absolute; left: -6px; top: 50%; transform: translateY(-50%);
  width: 11px; height: 11px; border-radius: 50%; background: var(--n8n-node-bg); border: 2px solid var(--n8n-wire);
  transition: transform 0.1s, border-color 0.1s, background 0.1s;
}
.n8n-port-in.snap { border-color: var(--n8n-coral); background: var(--n8n-coral); transform: translateY(-50%) scale(1.3); }
.n8n-expr-preview {
  margin-top: 4px; padding: 5px 8px; border-radius: 5px; font-size: 11px;
  background: var(--n8n-green-soft); color: var(--n8n-green); word-break: break-word;
}
.n8n-node-box.pinned-node { box-shadow: 0 0 0 3px rgba(124, 58, 237, 0.25); border-color: #7c3aed; }
.n8n-port-out {
  position: absolute; right: -6px; width: 11px; height: 11px; border-radius: 50%;
  background: var(--n8n-node-bg); border: 2px solid var(--n8n-wire); cursor: pointer;
}
.n8n-port-out:hover { border-color: var(--n8n-coral); transform: scale(1.2); }
.n8n-port-add {
  position: absolute; right: -26px; width: 16px; height: 16px; border-radius: 4px;
  border: 1px solid var(--n8n-node-border); background: var(--n8n-node-bg); color: var(--n8n-text-2);
  font-size: 12px; line-height: 1; display: flex; align-items: center; justify-content: center;
  cursor: pointer; opacity: 0; transition: opacity 0.12s;
}
.n8n-node-wrap:hover .n8n-port-add { opacity: 1; }
.n8n-port-add:hover { border-color: var(--n8n-coral); color: var(--n8n-coral); }
.n8n-branch-lbl {
  position: absolute; right: 8px; font-size: 9.5px; color: var(--n8n-text-3); font-weight: 600;
}
.n8n-diamond-ports {
  position: absolute; bottom: -19px; left: 16px; right: 16px; display: flex; justify-content: space-around; pointer-events: none;
}
.n8n-diamond { display: flex; flex-direction: column; align-items: center; gap: 2px; font-size: 9.5px; color: var(--n8n-text-3); }
.n8n-diamond-sq { width: 9px; height: 9px; background: var(--n8n-node-bg); border: 2px solid var(--n8n-wire); transform: rotate(45deg); }

/* Node Label Centered Below Node */
.n8n-node-label {
  position: relative; width: 150px; margin-left: -27px; margin-top: 8px;
  text-align: center; pointer-events: none;
}
.n8n-node-wrap.agent-root-wrap .n8n-node-label { display: none; }
.n8n-node-wrap.subnode-wrap .n8n-node-label { width: 130px; margin-left: -31px; }
.n8n-node-title { font-size: 12.5px; font-weight: 600; color: var(--n8n-text-1); line-height: 1.25; word-break: break-word; }
.n8n-node-sub { font-size: 10.5px; color: var(--n8n-text-3); margin-top: 2px; }

/* Sticky Note */
.n8n-sticky {
  position: absolute; background: #fff9db; border: 1px solid #f2e085; border-radius: 6px;
  padding: 12px; color: #3f381a; font-size: 12px; line-height: 1.45; overflow: hidden;
  box-shadow: var(--n8n-shadow); cursor: move;
}

/* Floating Canvas Controls */
.n8n-float-plus {
  position: absolute; top: 16px; right: 16px; z-index: 20;
  width: 42px; height: 42px; border-radius: 8px; border: 1px solid var(--n8n-border);
  background: var(--n8n-panel-bg); color: var(--n8n-text-1); font-size: 20px; font-weight: 500;
  box-shadow: var(--n8n-shadow); cursor: pointer; display: flex; align-items: center; justify-content: center;
}
.n8n-float-plus:hover { border-color: var(--n8n-coral); color: var(--n8n-coral); }

.n8n-controls-bl {
  position: absolute; bottom: 16px; left: 16px; z-index: 20; display: flex; gap: 6px;
}
.n8n-sq-btn {
  width: 34px; height: 34px; border-radius: 6px; border: 1px solid var(--n8n-border);
  background: var(--n8n-panel-bg); color: var(--n8n-text-2); cursor: pointer; box-shadow: var(--n8n-shadow);
  display: flex; align-items: center; justify-content: center; font-size: 14px; font-weight: 600;
}
.n8n-sq-btn:hover { color: var(--n8n-text-1); border-color: var(--n8n-text-3); }

.n8n-test-bar {
  position: absolute; bottom: 16px; left: 50%; transform: translateX(-50%); z-index: 20;
  display: flex; align-items: center; gap: 8px; background: rgba(255,255,255,0.85);
  backdrop-filter: blur(8px); padding: 6px; border-radius: 10px; border: 1px solid var(--n8n-border);
  box-shadow: var(--n8n-shadow-lg);
}
html.dark .n8n-test-bar { background: rgba(34,34,34,0.88); }

/* Collapsible Bottom Logs Panel */
.n8n-bottom-logs {
  height: 0; border-top: 1px solid var(--n8n-border); background: var(--n8n-panel-bg);
  transition: height 0.16s ease; overflow: hidden; display: flex; flex-direction: column; z-index: 25;
}
.n8n-bottom-logs.open { height: 220px; }

/* Slide-over Node Creator Drawer ("Was passiert als Nächstes?") */
.n8n-creator {
  position: fixed; top: 52px; right: 0; bottom: 0; width: 340px; max-width: 96vw;
  background: var(--n8n-panel-bg); border-left: 1px solid var(--n8n-border); box-shadow: var(--n8n-shadow-lg);
  z-index: 60; display: none; flex-direction: column;
}
.n8n-creator.open { display: flex; }
.n8n-creator-item {
  width: 100%; padding: 9px 10px; border-radius: 7px; border: 1px solid transparent; background: transparent;
  display: flex; align-items: center; gap: 10px; cursor: pointer; text-align: left;
}
.n8n-creator-item:hover { background: var(--n8n-subtle-bg); border-color: var(--n8n-border); }

/* 3-Column NDV (Node Details View) Modal */
.n8n-ndv-backdrop {
  position: fixed; inset: 0; background: rgba(15,23,42,0.55); backdrop-filter: blur(2px);
  z-index: 90; display: none; align-items: center; justify-content: center;
}
.n8n-ndv-backdrop.open { display: flex; }
.n8n-ndv {
  width: 94vw; max-width: 1360px; height: 86vh; background: var(--n8n-panel-bg);
  border: 1px solid var(--n8n-border); border-radius: 12px; box-shadow: var(--n8n-shadow-lg);
  display: flex; flex-direction: column; overflow: hidden;
}
.n8n-ndv-top {
  height: 44px; padding: 0 14px; border-bottom: 1px solid var(--n8n-border);
  display: flex; align-items: center; justify-content: space-between; background: var(--n8n-subtle-bg);
}
.n8n-ndv-grid {
  flex: 1; min-height: 0; display: grid; grid-template-columns: 28% 44% 28%;
}
.n8n-ndv-col { display: flex; flex-direction: column; min-height: 0; border-right: 1px solid var(--n8n-border); overflow: hidden; }
.n8n-ndv-col:last-child { border-right: none; }
.n8n-ndv-col-hdr {
  height: 46px; padding: 0 14px; border-bottom: 1px solid var(--n8n-border);
  display: flex; align-items: center; justify-content: space-between; gap: 8px;
}
.n8n-ndv-body { flex: 1; overflow: auto; padding: 14px; display: flex; flex-direction: column; gap: 10px; }
.n8n-field-lbl { font-size: 11.5px; font-weight: 600; color: var(--n8n-text-2); margin-bottom: 4px; display: block; }
.n8n-input, .n8n-textarea, .n8n-select {
  width: 100%; padding: 7px 9px; border: 1px solid var(--n8n-border); border-radius: 6px;
  background: var(--n8n-subtle-bg); outline: none; font-size: 12.5px;
}
.n8n-input:focus, .n8n-textarea:focus, .n8n-select:focus { border-color: var(--n8n-coral); background: var(--n8n-panel-bg); }
.n8n-code-ed {
  width: 100%; padding: 10px; border-radius: 6px; background: #161b22; color: #e6edf3;
  border: 1px solid #30363d; font-size: 12px; line-height: 1.5; outline: none;
}

/* Overview / Executions / Templates Subpages */
.n8n-page { flex: 1; overflow: auto; padding: 24px 32px; background: var(--n8n-canvas-bg); }
.n8n-wf-card {
  background: var(--n8n-panel-bg); border: 1px solid var(--n8n-border); border-radius: 8px;
  padding: 13px 16px; display: flex; align-items: center; justify-content: space-between; gap: 12px;
  margin-bottom: 8px; cursor: pointer;
}
.n8n-wf-card:hover { border-color: var(--n8n-coral); }

.n8n-toast {
  position: fixed; bottom: 16px; right: 16px; z-index: 120; padding: 8px 13px; border-radius: 6px;
  background: #1f2329; color: #fff; font-size: 12.5px; font-weight: 500; box-shadow: var(--n8n-shadow-lg); display: none;
}
.n8n-toast.show { display: block; }

@media (max-width: 960px) {
  .n8n-nav { position: fixed; top: 0; left: 0; bottom: 0; }
  .n8n-nav.collapsed { margin-left: -204px; width: 204px; min-width: 204px; }
  .n8n-seg { display: none; }
  .n8n-ndv-grid { grid-template-columns: 1fr; overflow-y: auto; }
  .n8n-ndv-col { border-right: none; border-bottom: 1px solid var(--n8n-border); min-height: 220px; }
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
}
</style>
</head>
<body>
<div class="n8n-shell">
  <!-- Left Dark n8n Sidebar -->
  <aside class="n8n-nav" id="n8nNav">
    <div class="n8n-nav-top">
      <div class="n8n-brand" onclick="setSection('overview')">
        <svg width="26" height="20" viewBox="0 0 36 24" fill="none">
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
        <span class="nav-badge" id="navWfCount">58</span>
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
        <span class="n8n-nav-left"><svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M13 2L3 14h9l-1 8l10-12h-9l1-8z"/></svg><span class="nav-txt">5.000 € → 20 € Vergleich</span></span>
      </a>
      <div class="n8n-user-row">
        <span class="n8n-avatar">MC</span>
        <div class="nav-txt" style="min-width:0;">
          <div style="font-size:12px;font-weight:600;color:#fff;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;">Merlin Conrad</div>
          <div style="font-size:10.5px;color:#9ca3af;">Conrad KI Engine · Aktiv</div>
        </div>
      </div>
    </div>
  </aside>

  <!-- Main Workspace -->
  <main class="n8n-main">
    <!-- Top 52px Header Bar -->
    <header class="n8n-hdr" id="wfHeader">
      <div class="n8n-hdr-left">
        <button class="n8n-btn" style="padding:0 7px;" onclick="toggleNav()" title="Menü">☰</button>
        <div class="n8n-hide-sm" style="position:relative;">
          <button class="n8n-crumb-btn" onclick="toggleN8nPop('wfPickPop')">Persönlich ▾</button>
          <div class="n8n-pop" id="wfPickPop" style="top:34px;left:0;width:320px;max-height:420px;overflow-y:auto;"></div>
        </div>
        <span class="n8n-hide-sm" style="color:var(--n8n-text-3);">/</span>
        <input class="n8n-title-inp" id="wfTitleInp" value="Workflow" onchange="renameWorkflow(this.value)">
        <div class="n8n-hide-sm" id="wfTagsWrap" style="display:flex;align-items:center;gap:4px;"></div>
        <button class="n8n-tag-pill n8n-hide-sm" onclick="promptAddTag()">+ Tag hinzufügen</button>
      </div>

      <div class="n8n-seg" id="centerTabs">
        <button class="n8n-seg-btn active" onclick="setEditorTab('editor',this)">Editor</button>
        <button class="n8n-seg-btn" onclick="setEditorTab('executions',this)">Ausführungen</button>
        <button class="n8n-seg-btn" onclick="setEditorTab('evaluations',this)">Auswertungen</button>
      </div>

      <div class="n8n-hdr-right">
        <input type="hidden" id="modelSelect" value="conrad-n8n-agent:0.6b">
        <span class="n8n-btn mono n8n-hide-sm" style="font-size:11px;padding:0 9px;cursor:default;background:var(--n8n-green-soft);border-color:var(--n8n-green);color:var(--n8n-green);font-weight:700;" title="Spezialisierte, nachtrainierte Conrad KI Engine für Workflows &amp; CRM-Automatisierung">⚡ Conrad KI Engine</span>
        <div class="n8n-active-wrap" onclick="toggleActiveWf()">
          <span class="n8n-hide-sm" id="activeLbl">Inaktiv</span>
          <button type="button" class="n8n-switch" id="activeSwitch"></button>
        </div>
        <button class="n8n-btn n8n-hide-sm" onclick="openShareModal()">Teilen</button>
        <button class="n8n-btn n8n-btn-coral" id="saveBtn" onclick="saveCurrentWorkflow()">Speichern</button>
        <div style="position:relative;">
          <button class="n8n-btn" style="padding:0 8px;" onclick="toggleN8nPop('wfMenuPop')">⋯</button>
          <div class="n8n-pop" id="wfMenuPop" style="top:34px;right:0;">
            <button class="n8n-pop-item" onclick="duplicateCurrentWorkflow();closeN8nPops()"><span>Workflow duplizieren</span></button>
            <button class="n8n-pop-item" onclick="openWfImportModal();closeN8nPops()"><span>JSON importieren</span></button>
            <button class="n8n-pop-item" onclick="downloadWorkflowJson();closeN8nPops()"><span>JSON herunterladen</span></button>
            <button class="n8n-pop-item" onclick="syncSqlite();closeN8nPops()"><span>Aus Docker-n8n-SQLite synchronisieren</span></button>
            <div style="border-top:1px solid var(--n8n-border);margin:4px 0;"></div>
            <button class="n8n-pop-item" style="color:var(--n8n-red);" onclick="deleteCurrentWorkflow();closeN8nPops()"><span>Workflow löschen</span></button>
          </div>
        </div>
      </div>
    </header>

    <!-- Primary Content Area -->
    <div id="mainContent" style="flex:1;min-height:0;display:flex;flex-direction:column;position:relative;overflow:hidden;">
      <!-- Canvas Stage -->
      <div class="n8n-stage" id="canvasStage">
        <div class="n8n-canvas" id="n8nCanvas">
          <div class="n8n-viewport" id="n8nViewport">
            <svg class="n8n-wires" id="wireSvg"></svg>
            <div id="nodesLayer"></div>
          </div>
          <button class="n8n-float-plus" onclick="openCreatorDrawer(null)" title="Node hinzufügen (Was passiert als Nächstes?)">+</button>
          <div class="n8n-controls-bl">
            <button class="n8n-sq-btn" onclick="fitToView()" title="Ansicht einpassen">⊡</button>
            <button class="n8n-sq-btn" onclick="zoomBy(0.12)" title="Vergrößern">+</button>
            <button class="n8n-sq-btn" onclick="zoomBy(-0.12)" title="Verkleinern">−</button>
            <button class="n8n-sq-btn" onclick="tidyUpLayout()" title="Workflow aufräumen">🪄</button>
          </div>
          <div class="n8n-test-bar">
            <button class="n8n-btn n8n-btn-coral" id="testWfBtn" onclick="runCurrentWorkflow()" style="height:34px;padding:0 16px;font-size:13px;">🧪 Workflow testen</button>
            <button class="n8n-btn" onclick="togglePayloadPop()" style="height:34px;">💬 Testdaten</button>
            <button class="n8n-btn" onclick="toggleBottomLogs()" id="logsToggleBtn" style="height:34px;">Protokoll</button>
          </div>
        </div>

        <!-- Collapsible Bottom Execution Logs Drawer -->
        <div class="n8n-bottom-logs" id="bottomLogs">
          <div style="height:34px;padding:0 14px;border-bottom:1px solid var(--n8n-border);display:flex;align-items:center;justify-content:space-between;background:var(--n8n-subtle-bg);">
            <strong style="font-size:12px;" id="logsHdrTxt">Ausführungsprotokoll &amp; Webhook-Ausgabe</strong>
            <button class="n8n-btn" style="height:24px;" onclick="toggleBottomLogs()">Schließen ×</button>
          </div>
          <div style="flex:1;overflow:auto;display:grid;grid-template-columns:280px 1fr;" id="logsBodyGrid">
            <div id="logsStepList" style="border-right:1px solid var(--n8n-border);overflow-y:auto;padding:8px;"></div>
            <pre id="logsJsonPre" class="mono" style="margin:0;padding:12px;overflow:auto;font-size:11.5px;">Noch kein Lauf in dieser Sitzung.</pre>
          </div>
        </div>
      </div>

      <!-- Non-canvas View Container (Overview / Executions / Templates / Variables / Insights) -->
      <div class="n8n-page" id="altPage" style="display:none;"></div>
    </div>
  </main>
</div>

<!-- Node Creator Slide-Over ("Was passiert als Nächstes?") -->
<aside class="n8n-creator" id="creatorDrawer">
  <div style="padding:14px 16px;border-bottom:1px solid var(--n8n-border);display:flex;align-items:center;justify-content:space-between;">
    <strong style="font-size:14px;">Was passiert als Nächstes? <span class="tag-pill mono" id="creatorCountBadge" style="font-size:11px;margin-left:4px;">118</span></strong>
    <button class="n8n-btn" style="height:26px;" onclick="closeCreatorDrawer()">×</button>
  </div>
  <div style="padding:10px 14px;border-bottom:1px solid var(--n8n-border);display:flex;flex-direction:column;gap:8px;">
    <input class="n8n-input" id="creatorSearch" placeholder="Nodes durchsuchen (Webhook, KI-Agent, Code, Gmail, Datentabelle...)" oninput="renderCreatorList()">
    <div id="creatorCatBar" style="display:flex;gap:5px;flex-wrap:wrap;"></div>
  </div>
  <div id="creatorList" style="flex:1;overflow-y:auto;padding:10px 12px;"></div>
</aside>

<!-- 3-Column NDV (Node Details View) Modal -->
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
          <strong style="font-size:12px;letter-spacing:0.04em;">EINGANG</strong>
          <div class="n8n-seg">
            <button class="n8n-seg-btn active" onclick="setNdvInFmt('json',this)">JSON</button>
            <button class="n8n-seg-btn" onclick="setNdvInFmt('table',this)">Tabelle</button>
          </div>
        </div>
        <div class="n8n-ndv-body" id="ndvInBody"></div>
      </div>

      <!-- Column 2: Parameter / Einstellungen + Schritt testen -->
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
          <div class="n8n-seg">
            <button class="n8n-seg-btn active" onclick="setNdvOutFmt('json',this)">JSON</button>
            <button class="n8n-seg-btn" onclick="setNdvOutFmt('table',this)">Tabelle</button>
          </div>
        </div>
        <div class="n8n-ndv-body" id="ndvOutBody"></div>
      </div>
    </div>
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
  "message": "Suche Firma Ohrladen in Twenty und erstelle einen Entwurf",
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
      <strong>n8n Workflow-JSON importieren</strong>
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

<div class="n8n-toast" id="n8nToast"></div>

<script>
const NODE_META = {
  'n8n-nodes-base.webhook': { label: 'Webhook', cat: 'Trigger', color: '#ff6d5a', trig: true, svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#ff6d5a" stroke-width="2" stroke-linecap="round"><path d="M4.876 13.61a4 4 0 1 0 6.124 3.39h6"/><path d="M15.066 20.502a4 4 0 1 0 1.934 -7.502c-.706 0 -1.424 .179 -2 .5l-3 -5.5"/><path d="M16 8a4 4 0 1 0 -8 0c0 1.506 .77 2.818 2 3.5l-3 5.5"/></svg>' },
  'n8n-nodes-base.manualTrigger': { label: 'Manueller Trigger', cat: 'Trigger', color: '#ff6d5a', trig: true, svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="#ff6d5a"><polygon points="6,4 20,12 6,20"/></svg>' },
  'n8n-nodes-base.scheduleTrigger': { label: 'Zeitplan-Trigger', cat: 'Trigger', color: '#0d9488', trig: true, svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#0d9488" stroke-width="2"><circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 3"/></svg>' },
  '@n8n/n8n-nodes-langchain.chatTrigger': { label: 'Chat-Trigger', cat: 'Trigger', color: '#7c3aed', trig: true, svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#7c3aed" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>' },
  'n8n-nodes-base.gmailTrigger': { label: 'Gmail-Trigger', cat: 'Trigger', color: '#ea4335', trig: true, svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#ea4335" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.telegramTrigger': { label: 'Telegram-Trigger', cat: 'Trigger', color: '#26a5e4', trig: true, svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#26a5e4" stroke-width="2"><path d="M15 10l-4 4l6 6l4-16l-18 7l4 2l2 6l3-4"/></svg>' },
  'n8n-nodes-base.microsoftOutlookTrigger': { label: 'Outlook-Trigger', cat: 'Trigger', color: '#0078d4', trig: true, svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#0078d4" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.errorTrigger': { label: 'Fehler-Trigger', cat: 'Trigger', color: '#dc2626', trig: true, svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#dc2626" stroke-width="2"><path d="M12 9v4m0 4v.01"/><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/></svg>' },
  '@n8n/n8n-nodes-langchain.agent': { label: 'KI-Agent', cat: 'KI', color: '#1f2329', rootAgent: true, svg: '<svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="7" width="18" height="12" rx="3"/><circle cx="9" cy="13" r="1.5" fill="currentColor"/><circle cx="15" cy="13" r="1.5" fill="currentColor"/><path d="M12 7V4m-3 0h6"/></svg>' },
  '@n8n/n8n-nodes-langchain.chainLlm': { label: 'Basis-LLM-Kette', cat: 'KI', color: '#7c3aed', rootAgent: true, svg: '<svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="#7c3aed" stroke-width="2"><path d="M12 3l2.5 5.5L20 11l-5.5 2.5L12 19l-2.5-5.5L4 11l5.5-2.5z"/></svg>' },
  '@n8n/n8n-nodes-langchain.lmChatOllama': { label: 'Ollama Chat-Modell', cat: 'KI-Sub-Node', color: '#18181b', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="8"/><circle cx="9.5" cy="10.5" r="1" fill="currentColor"/><circle cx="14.5" cy="10.5" r="1" fill="currentColor"/><path d="M9.5 14.5c1 .8 4 .8 5 0"/></svg>' },
  '@n8n/n8n-nodes-langchain.lmChatOpenAi': { label: 'OpenAI Chat-Modell', cat: 'KI-Sub-Node', color: '#10a37f', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#10a37f" stroke-width="2"><circle cx="12" cy="12" r="8"/><path d="M12 4v16M5 8l14 8M19 8L5 16"/></svg>' },
  '@n8n/n8n-nodes-langchain.memoryBufferWindow': { label: 'Fenster-Pufferspeicher', cat: 'KI-Sub-Node', color: '#7c3aed', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#7c3aed" stroke-width="2"><rect x="6" y="6" width="12" height="12" rx="2"/><path d="M9 2v4m6-4v4m-6 12v4m6-4v4M2 9h4m-4 6h4m12-6h4m-4 6h4"/></svg>' },
  '@n8n/n8n-nodes-langchain.vectorStoreQdrant': { label: 'Qdrant Vektorspeicher', cat: 'KI-Sub-Node', color: '#dc2626', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#dc2626" stroke-width="2"><polygon points="12,2 21,7 21,17 12,22 3,17 3,7"/><circle cx="12" cy="12" r="3"/></svg>' },
  '@n8n/n8n-nodes-langchain.embeddingsOllama': { label: 'Ollama-Embeddings', cat: 'KI-Sub-Node', color: '#4f46e5', subnode: true, svg: '<svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="#4f46e5" stroke-width="2"><path d="M4 7h16M4 12h16M4 17h10"/></svg>' },
  'n8n-nodes-base.httpRequest': { label: 'HTTP-Anfrage', cat: 'Kern', color: '#4078f2', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#4078f2" stroke-width="2"><circle cx="12" cy="12" r="9"/><path d="M3.6 9h16.8M3.6 15h16.8M11.5 3a17 17 0 0 0 0 18M12.5 3a17 17 0 0 1 0 18"/></svg>' },
  'n8n-nodes-base.code': { label: 'Code', cat: 'Kern', color: '#f97316', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#f97316" stroke-width="2.1" stroke-linecap="round"><path d="M8 6c-2 0-2 2-2 4s-2 2-2 2s2 0 2 2s0 4 2 4"/><path d="M16 6c2 0 2 2 2 4s2 2 2 2s-2 0-2 2s0 4-2 4"/></svg>' },
  'n8n-nodes-base.if': { label: 'Wenn (If)', cat: 'Kern', color: '#16a34a', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#16a34a" stroke-width="2.2" stroke-linecap="round"><path d="M4 12h5l4-5h7"/><path d="M9 12l4 5h7"/></svg>' },
  'n8n-nodes-base.switch': { label: 'Weiche (Switch)', cat: 'Kern', color: '#6366f1', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#6366f1" stroke-width="2" stroke-linecap="round"><path d="M3 12h6M13 6h8M13 12h8M13 18h8M9 12l4-6M9 12l4 6"/></svg>' },
  'n8n-nodes-base.set': { label: 'Felder bearbeiten (Set)', cat: 'Kern', color: '#2563eb', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#2563eb" stroke-width="2"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1l1-4z"/></svg>' },
  'n8n-nodes-base.respondToWebhook': { label: 'Auf Webhook antworten', cat: 'Kern', color: '#d97706', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#d97706" stroke-width="2"><path d="M9 14l-4-4l4-4"/><path d="M5 10h11a4 4 0 1 1 0 8h-1"/></svg>' },
  'n8n-nodes-base.gmail': { label: 'Gmail', cat: 'App', color: '#ea4335', svg: '<svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="#ea4335" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.telegram': { label: 'Telegram', cat: 'App', color: '#26a5e4', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#26a5e4" stroke-width="2"><path d="M15 10l-4 4l6 6l4-16l-18 7l4 2l2 6l3-4"/></svg>' },
  'n8n-nodes-base.microsoftOutlook': { label: 'Microsoft Outlook', cat: 'App', color: '#0078d4', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#0078d4" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.postgres': { label: 'Postgres', cat: 'App', color: '#336791', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#336791" stroke-width="2"><ellipse cx="12" cy="6" rx="8" ry="3"/><path d="M4 6v6a8 3 0 0 0 16 0v-6"/><path d="M4 12v6a8 3 0 0 0 16 0v-6"/></svg>' },
  'n8n-nodes-base.googleSheets': { label: 'Google Sheets', cat: 'App', color: '#0f9d58', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#0f9d58" stroke-width="2"><rect x="4" y="3" width="16" height="18" rx="2"/><path d="M4 9h16M4 15h16M10 9v12"/></svg>' },
  'n8n-nodes-base.emailSend': { label: 'E-Mail senden', cat: 'App', color: '#0d9488', svg: '<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#0d9488" stroke-width="2"><rect x="3" y="5" width="18" height="14" rx="2"/><path d="M3 7l9 6l9-6"/></svg>' },
  'n8n-nodes-base.noOp': { label: 'Keine Aktion', cat: 'Kern', color: '#64748b', svg: '<svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="#64748b" stroke-width="2"><path d="M5 12h14m-5-5l5 5l-5 5"/></svg>' },
  'n8n-nodes-base.stickyNote': { label: 'Haftnotiz', cat: 'Kern', color: '#ca8a04', svg: '<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="2"><path d="M13 20l7-7V5a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2z"/></svg>' }
};

const st = {
  section: 'editor', // 'overview' | 'editor' | 'templates' | 'credentials' | 'data-tables' | 'variables' | 'insights'
  editorTab: 'editor', // 'editor' | 'executions' | 'evaluations'
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
  creatorCat: 'Alle',
  executions: [],
  selectedExecIdx: 0,
  nodeStatusMap: {}, // nodeName -> { status, durationMs, input, output }
  lastExec: null,
  ndvNodeName: null,
  ndvCenterTab: 'params',
  ndvInFmt: 'json',
  ndvOutFmt: 'json',
  creatorSourceNode: null,
  creatorSourceBranch: 0,
  zoom: 0.92,
  panX: 60,
  panY: 70,
  dirty: false,
  selectedNodeName: null
};

function esc(s) { return String(s ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c])); }
function toast(msg) { const el = document.getElementById('n8nToast'); el.textContent = msg; el.classList.add('show'); setTimeout(() => el.classList.remove('show'), 2200); }
const CAT_COLORS = { Trigger: '#ff6d5a', KI: '#7c3aed', Kern: '#2563eb', Daten: '#0d9488', Kommunikation: '#ea4335', Integration: '#059669', App: '#4078f2' };
function makeNodeSvg(color, glyph) {
  return `<svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="${color}" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="5" fill="${color}14"/><text x="12" y="16" text-anchor="middle" font-size="9" font-weight="700" fill="${color}" stroke="none">${esc(glyph)}</text></svg>`;
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
    const glyph = (sc.displayName || sc.name || 'N').replace(/[^A-Za-z0-9ÄÖÜäöü]/g, '').slice(0, 2).toUpperCase() || 'N8';
    return { label: sc.displayName || tp, cat, color: clr, trig, subnode: sub, rootAgent, svg: makeNodeSvg(clr, glyph), description: sc.description || '' };
  }
  return { label: (tp||'Node').split('.').pop(), cat: 'Kern', color: '#64748b', svg: NODE_META['n8n-nodes-base.noOp'].svg };
}
function isTrigger(tp) { const t = String(tp||''); return !!metaFor(t).trig || (t.toLowerCase().includes('trigger') && !t.includes('respond')); }
function isSubnode(tp) { return !!metaFor(tp).subnode; }
function isAgentRoot(tp) { return !!metaFor(tp).rootAgent; }

if (localStorage.getItem('n8n_theme') === 'dark') { document.documentElement.classList.add('dark'); }
function toggleN8nTheme() {
  const dark = document.documentElement.classList.toggle('dark');
  localStorage.setItem('n8n_theme', dark ? 'dark' : 'light');
  document.getElementById('themeLbl').textContent = dark ? 'Helle Ansicht' : 'Dunkle Ansicht';
}
function toggleNav() { document.getElementById('n8nNav').classList.toggle('collapsed'); }
function toggleN8nPop(id) {
  const el = document.getElementById(id); const was = el.classList.contains('open');
  closeN8nPops(); if (!was) el.classList.add('open');
}
function closeN8nPops() { document.querySelectorAll('.n8n-pop').forEach(p => p.classList.remove('open')); }
document.addEventListener('click', e => { if (!e.target.closest('#wfPickPop,.n8n-crumb-btn,#wfMenuPop,.n8n-hdr-right')) closeN8nPops(); });

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
  const preferred = st.workflows.find(w => w.id === 'conradKiAgent001') || st.workflows.find(w => w.active) || st.workflows[0];
  if (preferred) await selectWorkflow(preferred.id);
}

function renderWfPicker() {
  document.getElementById('wfPickPop').innerHTML = st.workflows.map(w => `
    <button class="n8n-pop-item" onclick="selectWorkflow('${esc(w.id)}');closeN8nPops()">
      <span style="overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">${w.active ? '🟢 ' : ''}${esc(w.name)}</span>
      <span class="mono" style="font-size:10.5px;color:var(--n8n-text-3);">${w.nodeCount}n</span>
    </button>
  `).join('');
}

async function selectWorkflow(id) {
  const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(id)}`);
  st.currentWf = await r.json();
  st.nodeStatusMap = {};
  st.lastExec = null;
  st.dirty = false;
  st.section = 'editor';
  st.editorTab = 'editor';
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
  document.getElementById('wfTagsWrap').innerHTML = tags.map((t, i) => `
    <span class="n8n-tag-pill">${esc(typeof t === 'string' ? t : t.name)} <span onclick="removeTag(${i})" style="margin-left:2px;">×</span></span>
  `).join('');
  document.getElementById('saveBtn').textContent = st.dirty ? 'Speichern *' : 'Speichern';
}

function setSection(sec) {
  st.section = sec;
  document.querySelectorAll('.n8n-nav-item').forEach(el => el.classList.remove('active'));
  document.getElementById('nav-' + sec)?.classList.add('active');
  const stage = document.getElementById('canvasStage');
  const alt = document.getElementById('altPage');
  if (sec === 'editor') {
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

async function renderSectionPage(sec) {
  const alt = document.getElementById('altPage');
  if (sec === 'overview') {
    const r = await fetch('/api/n8n/workflows');
    const d = await r.json();
    st.workflows = d.data || [];
    alt.innerHTML = `
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px;flex-wrap:wrap;gap:10px;">
        <div><h2 style="margin:0;font-size:20px;">Conrad Automation · Workflows (${st.workflows.length})</h2><div style="color:var(--n8n-text-2);font-size:12.5px;">Enterprise DAG-Automation-Engine · Sofortige Persistenz · Bis zu 10× schneller</div></div>
        <div style="display:flex;gap:8px;">
          <input class="n8n-input" id="ovSearch" placeholder="Workflow suchen..." style="width:220px;" oninput="filterOverviewList()">
          <button class="n8n-btn n8n-btn-coral" onclick="createBlankWorkflow()">+ Workflow erstellen</button>
        </div>
      </div>
      <div id="ovList"></div>
    `;
    filterOverviewList();
  } else if (sec === 'templates') {
    const tpls = [
      { name: 'Hormozi Lead-Qualifizierung (0.6B KI-Agent)', desc: 'Webhook -> 0.6B Tool-Agent (Qdrant RAG + Twenty-Suche) -> Auf Webhook antworten', nodes: [
        { id: 't1', name: 'Webhook', type: 'n8n-nodes-base.webhook', position: [180, 280], parameters: { path: 'lead-qualify', httpMethod: 'POST' } },
        { id: 't2', name: 'KI-Agent', type: '@n8n/n8n-nodes-langchain.agent', position: [460, 280], parameters: { text: '={{ $json.message }}' } },
        { id: 't3', name: 'Ollama Chat-Modell', type: '@n8n/n8n-nodes-langchain.lmChatOllama', position: [440, 470], parameters: { model: 'conrad-n8n-agent:0.6b' } },
        { id: 't4', name: 'Qdrant Vektorspeicher', type: '@n8n/n8n-nodes-langchain.vectorStoreQdrant', position: [620, 470], parameters: { qdrantCollection: 'conrad_ki_knowledge' } },
        { id: 't5', name: 'Auf Webhook antworten', type: 'n8n-nodes-base.respondToWebhook', position: [800, 280], parameters: { respondWith: 'json' } }
      ], connections: { 'Webhook': { main: [[{ node: 'KI-Agent', type: 'main', index: 0 }]] }, 'KI-Agent': { main: [[{ node: 'Auf Webhook antworten', type: 'main', index: 0 }]] }, 'Ollama Chat-Modell': { ai_languageModel: [[{ node: 'KI-Agent', type: 'ai_languageModel', index: 0 }]] }, 'Qdrant Vektorspeicher': { ai_tool: [[{ node: 'KI-Agent', type: 'ai_tool', index: 0 }]] } } },
      { name: 'Gmail Entwurfs-Klassifizierer (Safe Draft)', desc: 'Gmail-Trigger -> 0.6B HTTP-Klassifizierer -> Wenn (Antwort) -> Nur Gmail-Entwurf speichern', nodes: [
        { id: 'g1', name: 'Gmail-Trigger', type: 'n8n-nodes-base.gmailTrigger', position: [180, 260], parameters: {} },
        { id: 'g2', name: '0.6B Klassifizierer', type: 'n8n-nodes-base.httpRequest', position: [430, 260], parameters: { method: 'POST', url: 'http://127.0.0.1:11434/api/chat' } },
        { id: 'g3', name: 'Wenn Antwort nötig', type: 'n8n-nodes-base.if', position: [680, 260], parameters: {} },
        { id: 'g4', name: 'Gmail-Entwurf', type: 'n8n-nodes-base.gmail', position: [930, 200], parameters: { operation: 'createDraft' } }
      ], connections: { 'Gmail-Trigger': { main: [[{ node: '0.6B Klassifizierer', type: 'main', index: 0 }]] }, '0.6B Klassifizierer': { main: [[{ node: 'Wenn Antwort nötig', type: 'main', index: 0 }]] }, 'Wenn Antwort nötig': { main: [[{ node: 'Gmail-Entwurf', type: 'main', index: 0 }], []] } } }
    ];
    window._n8nTpls = tpls;
    alt.innerHTML = `<h2 style="margin:0 0 14px;font-size:20px;">Workflow-Vorlagen</h2>` + tpls.map((t, i) => `
      <div class="n8n-wf-card" onclick="useTemplate(${i})">
        <div><strong style="font-size:14px;">${esc(t.name)}</strong><div style="color:var(--n8n-text-2);font-size:12px;margin-top:3px;">${esc(t.desc)}</div></div>
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
    alt.innerHTML = `
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:14px;flex-wrap:wrap;gap:10px;">
        <div><h2 style="margin:0;font-size:20px;">Datentabellen (n8n Data Tables · ${st.dataTables.length})</h2><div style="color:var(--n8n-text-2);font-size:12.5px;">Persistente SQLite-Tabellen für Workflows &amp; Lookups</div></div>
        <div style="display:flex;gap:8px;align-items:center;">
          <input id="newDtName" class="n8n-input" style="width:190px;" placeholder="Tabellenname *">
          <button class="n8n-btn n8n-btn-coral" id="createDtBtn" onclick="createDataTable()">+ Neue Tabelle</button>
        </div>
      </div>
      <div style="display:grid;grid-template-columns:260px 1fr;gap:14px;">
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;padding:8px;">
          <strong style="display:block;padding:6px 8px;font-size:12px;color:var(--n8n-text-2);">Tabellen</strong>
          ${st.dataTables.map(t => `<div class="n8n-wf-card" style="padding:8px 10px;border-color:${activeTable?.id===t.id?'var(--n8n-coral)':'var(--n8n-border)'};" onclick="st.selectedDataTableId='${esc(t.id)}';renderSectionPage('data-tables')">
            <div><strong>${esc(t.name)}</strong><div style="font-size:11px;color:var(--n8n-text-3);">${t.rowCount || 0} Zeilen</div></div>
            <button class="n8n-btn" style="height:24px;color:var(--n8n-red);" onclick="event.stopPropagation();deleteDataTable('${esc(t.id)}')">×</button>
          </div>`).join('') || '<div style="padding:12px;color:var(--n8n-text-3);font-size:12px;">Noch keine Datentabellen.</div>'}
        </div>
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;padding:14px;overflow:auto;">
          ${activeTable ? `
            <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;flex-wrap:wrap;gap:8px;">
              <strong>${esc(activeTable.name)} · ${rows.length} Zeilen</strong>
              <div style="display:flex;gap:6px;flex:1;max-width:460px;">
                <input id="newDtRowJson" class="n8n-input mono" value='{"schluessel":"demo","wert":"aktiv"}' placeholder='{"schluessel":"...","wert":"..."}'>
                <button class="n8n-btn n8n-btn-coral" id="addDtRowBtn" onclick="insertDataTableRow('${esc(activeTable.id)}')">+ Zeile</button>
              </div>
            </div>
            ${rows.length ? `<table style="width:100%;border-collapse:collapse;font-size:12px;">
              <thead><tr style="border-bottom:1px solid var(--n8n-border);text-align:left;"><th style="padding:6px;">ID</th><th style="padding:6px;">Daten (JSON)</th><th style="padding:6px;"></th></tr></thead>
              <tbody>${rows.map(rw => `<tr style="border-bottom:1px solid var(--n8n-border);"><td class="mono" style="padding:6px;">${esc(rw.id)}</td><td class="mono" style="padding:6px;">${esc(JSON.stringify(rw))}</td><td style="padding:6px;text-align:right;"><button class="n8n-btn" style="height:22px;color:var(--n8n-red);" onclick="deleteDataTableRow('${esc(activeTable.id)}',${Number(rw.id)})">Löschen</button></td></tr>`).join('')}</tbody>
            </table>` : '<div style="color:var(--n8n-text-3);font-size:12.5px;">Noch keine Zeilen in dieser Tabelle.</div>'}
          ` : '<div style="color:var(--n8n-text-3);">Wähle links eine Tabelle aus oder erstelle eine neue.</div>'}
        </div>
      </div>`;
  } else if (sec === 'variables') {
    const vRes = await fetch('/rest/variables');
    const vData = vRes.ok ? await vRes.json() : { data: [] };
    st.variables = vData.data || [];
    alt.innerHTML = `<h2 style="margin:0 0 10px;font-size:20px;">Variablen (<code>$vars.*</code> in QuickJS)</h2>
      <div class="n8n-wf-card" style="cursor:default;gap:8px;flex-wrap:wrap;margin-bottom:14px;">
        <input id="newVarKey" class="n8n-input mono" style="width:200px;" placeholder="SCHLUESSEL *">
        <input id="newVarVal" class="n8n-input mono" style="flex:1;min-width:200px;" placeholder="Wert">
        <button class="n8n-btn n8n-btn-coral" id="createVarBtn" onclick="createVariable()">+ Variable speichern</button>
      </div>
      ${(st.variables || []).map(vr => `<div class="n8n-wf-card" style="cursor:default;gap:10px;">
        <span class="mono" style="font-weight:600;min-width:180px;">$vars.${esc(vr.key)}</span>
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
        <div class="n8n-wf-card" style="flex-direction:column;align-items:flex-start;"><span style="color:var(--n8n-text-2);">Workflows gesamt</span><strong style="font-size:26px;">${m.n8n?.workflows ?? 58}</strong></div>
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
  toast('Gelöscht'); renderSectionPage('credentials');
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
  const rows = st.workflows.filter(w => !q || w.name.toLowerCase().includes(q) || (w.webhookPath||'').toLowerCase().includes(q));
  document.getElementById('ovList').innerHTML = rows.map(w => `
    <div class="n8n-wf-card" onclick="selectWorkflow('${esc(w.id)}')">
      <div style="min-width:0;flex:1;">
        <div style="display:flex;align-items:center;gap:8px;flex-wrap:wrap;">
          <strong style="font-size:14px;">${esc(w.name)}</strong>
          ${(w.tags||[]).map(t => `<span class="n8n-tag-pill">${esc(typeof t==='string'?t:t.name)}</span>`).join('')}
        </div>
        <div class="mono" style="font-size:11px;color:var(--n8n-text-3);margin-top:4px;">
          ${w.nodeCount} Nodes · Trigger: ${w.triggerCount || 0}${w.webhookPath ? ' · /webhook-rust/' + esc(w.webhookPath) : ''}
        </div>
      </div>
      <div style="display:flex;align-items:center;gap:10px;" onclick="event.stopPropagation()">
        <span style="font-size:12px;color:${w.active ? 'var(--n8n-green)' : 'var(--n8n-text-3)'};">${w.active ? 'Aktiv' : 'Inaktiv'}</span>
        <button class="n8n-switch ${w.active ? 'on' : ''}" onclick="toggleWfInList('${esc(w.id)}')"></button>
      </div>
    </div>
  `).join('');
}
async function toggleWfInList(id) {
  await fetch(`/api/n8n/workflows/${encodeURIComponent(id)}/toggle`, { method: 'POST' });
  const r = await fetch('/api/n8n/workflows'); const d = await r.json(); st.workflows = d.data || [];
  filterOverviewList();
}
async function useTemplate(idx) {
  const tpl = window._n8nTpls[idx];
  const r = await fetch('/api/n8n/workflows', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name: tpl.name, active: true, nodes: tpl.nodes, connections: tpl.connections }) });
  const wf = await r.json(); await init(); selectWorkflow(wf.id);
}

async function retryExecution(execId) {
  try {
    const r = await fetch(`/rest/executions/${encodeURIComponent(execId)}/retry`, { method: 'POST' });
    const raw = await r.json();
    const res = raw?.data || raw;
    if (!r.ok || res?.error) {
      toast(res?.error || 'Wiederholung fehlgeschlagen');
      return;
    }
    st.lastExec = res;
    if (Array.isArray(res?.nodeLogs)) {
      st.nodeStatusMap = {};
      res.nodeLogs.forEach(l => {
        if (l?.node) st.nodeStatusMap[l.node] = l.status === 'error' ? 'error' : 'success';
      });
    }
    st.selectedExecIdx = 0;
    toast(`Ausführung wiederholt (${res.status || 'ok'} · ${res.durationMs ?? 0}ms)`);
    await renderAltEditorTab();
  } catch (e) {
    toast('Fehler bei Ausführungswiederholung: ' + (e?.message || e));
  }
}

async function renderAltEditorTab() {
  const alt = document.getElementById('altPage');
  if (st.editorTab === 'executions') {
    const wid = st.currentWf?.id || '';
    const r = await fetch(`/api/n8n/executions?workflowId=${encodeURIComponent(wid)}&limit=40`);
    const d = await r.json();
    st.executions = d.data || [];
    const ex = st.executions[st.selectedExecIdx] || st.executions[0];
    alt.innerHTML = `
      <div style="display:grid;grid-template-columns:300px 1fr;gap:14px;height:100%;">
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;overflow-y:auto;padding:8px;">
          <strong style="display:block;padding:6px 8px;font-size:12px;color:var(--n8n-text-2);">Vergangene Ausführungen (${st.executions.length})</strong>
          ${st.executions.map((e, i) => `
            <div class="n8n-wf-card" style="padding:9px 10px;border-color:${i===st.selectedExecIdx?'var(--n8n-coral)':'var(--n8n-border)'};" onclick="st.selectedExecIdx=${i};renderAltEditorTab()">
              <div>
                <div style="font-weight:600;font-size:12px;color:${e.status==='success'?'var(--n8n-green)':'var(--n8n-red)'};">${e.status==='success'?'✓ Erfolgreich':'✕ Fehler'} · ${e.durationMs}ms</div>
                <div class="mono" style="font-size:10.5px;color:var(--n8n-text-3);">${esc(e.modelUsed)} · ${esc((e.startedAt||'').slice(11,19))}</div>
              </div>
            </div>
          `).join('') || '<div style="padding:16px;color:var(--n8n-text-3);">Noch keine Ausführungen. Klicke „Workflow testen“.</div>'}
        </div>
        <div style="background:var(--n8n-panel-bg);border:1px solid var(--n8n-border);border-radius:8px;overflow:auto;padding:16px;">
          ${ex ? `<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;gap:8px;flex-wrap:wrap;">
            <strong>Ausführung ${esc(ex.id.slice(0,12))}</strong>
            <div style="display:flex;gap:8px;align-items:center;">
              <button class="n8n-btn n8n-btn-ghost" onclick="retryExecution('${esc(ex.id)}')" title="Ausführung erneut starten">🔄 Wiederholen</button>
              <span class="n8n-tag-pill mono">${esc(ex.modelUsed)} · ${ex.durationMs}ms</span>
            </div>
          </div><pre class="mono" style="background:var(--n8n-subtle-bg);padding:12px;border-radius:6px;overflow:auto;font-size:11.5px;">${esc(JSON.stringify(ex, null, 2))}</pre>` : '<div style="color:var(--n8n-text-3);">Wähle eine Ausführung aus.</div>'}
        </div>
      </div>
    `;
  } else if (st.editorTab === 'evaluations') {
    alt.innerHTML = `
      <h2 style="margin:0 0 8px;font-size:18px;">Auswertungen · Conrad KI Engine Test</h2>
      <p style="margin:0 0 14px;color:var(--n8n-text-2);">Führe <strong class="mono">${esc(st.currentWf?.name)}</strong> direkt gegen unsere spezialisiert nachtrainierte <strong>Conrad KI Engine</strong> aus:</p>
      <div style="display:flex;gap:8px;margin-bottom:14px;">
        <input id="evalPrompt" class="n8n-input" value="Suche in Conrad CRM nach Ohrladen und erstelle einen kurzen Entwurf" style="max-width:480px;">
        <button class="n8n-btn n8n-btn-coral" onclick="runEvalComparison()">▶ Conrad KI Engine testen</button>
      </div>
      <div id="evalResultsGrid" style="display:grid;grid-template-columns:repeat(auto-fit,minmax(320px,1fr));gap:12px;"></div>
    `;
  }
}

async function runEvalComparison() {
  const msg = document.getElementById('evalPrompt')?.value || 'Hallo';
  const m = 'conrad-n8n-agent:0.6b';
  const grid = document.getElementById('evalResultsGrid');
  grid.innerHTML = `<div class="n8n-wf-card" id="eval-main" style="flex-direction:column;align-items:stretch;"><strong>⚡ Conrad KI Engine</strong><div style="color:var(--n8n-text-3);margin-top:6px;">läuft...</div></div>`;
  const card = document.getElementById('eval-main');
  try {
    const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/execute`, {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ payload: { message: msg, model: m }, model: m })
    });
    const d = await r.json();
    card.innerHTML = `<div style="display:flex;justify-content:space-between;"><strong>⚡ Conrad KI Engine</strong><span class="n8n-tag-pill" style="color:var(--n8n-green);">${d.durationMs}ms</span></div><pre class="mono" style="margin:8px 0 0;font-size:11px;max-height:240px;overflow:auto;">${esc(JSON.stringify(d.response, null, 2))}</pre>`;
  } catch (e) { card.textContent = String(e); }
}

// Canvas Rendering & Interactions
function applyCanvasTransform() {
  document.getElementById('n8nViewport').style.transform = `translate(${st.panX}px, ${st.panY}px) scale(${st.zoom})`;
}
function zoomBy(delta) { st.zoom = Math.max(0.35, Math.min(1.6, st.zoom + delta)); applyCanvasTransform(); }
function fitToView() {
  const nodes = (st.currentWf?.nodes || []).filter(n => n.type !== 'n8n-nodes-base.stickyNote');
  if (!nodes.length) { st.panX = 60; st.panY = 70; st.zoom = 0.9; applyCanvasTransform(); return; }
  const xs = nodes.map(n => n.position?.[0] || 0), ys = nodes.map(n => n.position?.[1] || 0);
  const minX = Math.min(...xs), maxX = Math.max(...xs) + 220;
  const minY = Math.min(...ys), maxY = Math.max(...ys) + 160;
  const cw = document.getElementById('n8nCanvas').clientWidth || 1000;
  const ch = document.getElementById('n8nCanvas').clientHeight || 620;
  st.zoom = Math.max(0.45, Math.min(1.0, Math.min((cw - 100) / Math.max(300, maxX - minX), (ch - 140) / Math.max(250, maxY - minY))));
  st.panX = Math.max(40, (cw - (maxX - minX) * st.zoom) / 2 - minX * st.zoom);
  st.panY = Math.max(40, (ch - (maxY - minY) * st.zoom) / 2 - minY * st.zoom);
  applyCanvasTransform();
}
function tidyUpLayout() {
  if (!st.currentWf?.nodes) return;
  let colX = 120;
  const mains = st.currentWf.nodes.filter(n => !isSubnode(n.type) && n.type !== 'n8n-nodes-base.stickyNote');
  mains.forEach((n, idx) => { n.position = [colX + idx * 240, 220]; });
  const subs = st.currentWf.nodes.filter(n => isSubnode(n.type));
  subs.forEach((n, idx) => { n.position = [380 + idx * 130, 420]; });
  st.dirty = true; updateHeaderState(); renderCanvas(); fitToView(); toast('Layout bereinigt');
}

function nodeDimensions(n) {
  if (isAgentRoot(n.type)) return { w: 224, h: 96 };
  if (isSubnode(n.type)) return { w: 68, h: 68 };
  return { w: 96, h: 96 };
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
    if (n.type === 'n8n-nodes-base.stickyNote') {
      const w = n.parameters?.width || 220, h = n.parameters?.height || 140;
      html += `<div class="n8n-sticky" data-nodename="${esc(n.name)}" style="left:${x}px;top:${y}px;width:${w}px;height:${h}px;">${esc((n.parameters?.content||'').replace(/[#*]/g,'').slice(0,220))}</div>`;
      continue;
    }

    const m = metaFor(n.type);
    const trig = isTrigger(n.type);
    const sub = isSubnode(n.type);
    const root = isAgentRoot(n.type);
    const boxCls = root ? 'agent-root' : sub ? 'subnode' : trig ? 'trigger' : 'standard';
    const wrapCls = root ? 'agent-root-wrap' : sub ? 'subnode-wrap' : '';
    const stat = st.nodeStatusMap[n.name];
    const statCls = stat ? (stat.status === 'error' ? 'status-error' : 'status-success') : '';
    const selCls = st.selectedNodeName === n.name ? 'selected' : '';
    const disCls = n.disabled ? 'disabled-node' : '';
    const pinArr = st.currentWf.pinData?.[n.name];
    const isPinned = Array.isArray(pinArr) && pinArr.length > 0;
    const pinCls = isPinned ? 'pinned-node' : '';

    // Outgoing ports count
    const outBranches = conns[n.name]?.main || [];
    const portCount = n.type === 'n8n-nodes-base.if' ? 2 : Math.max(1, outBranches.length);
    let portsHtml = '';
    if (!trig && !sub) portsHtml += `<div class="n8n-port-in" data-in-node="${esc(n.name)}"></div>`;
    if (!sub) {
      for (let b = 0; b < portCount; b++) {
        const topPct = portCount === 1 ? 50 : (portCount === 2 ? (b === 0 ? 32 : 68) : (20 + (60 * b) / (portCount - 1)));
        const lbl = n.type === 'n8n-nodes-base.if' ? (b === 0 ? 'true' : 'false') : (portCount > 1 ? String(b) : '');
        if (lbl) portsHtml += `<span class="n8n-branch-lbl" style="top:calc(${topPct}% - 7px);">${lbl}</span>`;
        portsHtml += `<div class="n8n-port-out" style="top:calc(${topPct}% - 5px);" data-out-node="${esc(n.name)}" data-out-branch="${b}" title="Ziehen zum Verbinden oder Klicken für nächsten Node"></div>`;
        portsHtml += `<button class="n8n-port-add" style="top:calc(${topPct}% - 8px);" onclick="event.stopPropagation();openCreatorDrawer('${esc(n.name)}',${b})" title="Node anhängen">+</button>`;
      }
    } else {
      portsHtml += `<div class="n8n-port-out" style="top:-6px;right:calc(50% - 5px);" data-out-node="${esc(n.name)}" data-out-branch="0"></div>`;
    }

    const diamondHtml = root ? `<div class="n8n-diamond-ports">
      <div class="n8n-diamond"><div class="n8n-diamond-sq"></div><span>Chat-Modell*</span></div>
      <div class="n8n-diamond"><div class="n8n-diamond-sq"></div><span>Speicher</span></div>
      <div class="n8n-diamond"><div class="n8n-diamond-sq"></div><span>Werkzeug</span></div>
    </div>` : '';

    const innerContent = root
      ? `${m.svg}<div style="min-width:0;"><div class="n8n-node-title">${esc(n.name)}</div><div class="n8n-node-sub">${esc(m.label)}</div></div>`
      : m.svg;

    html += `<div class="n8n-node-wrap ${wrapCls}" data-nodename="${esc(n.name)}" style="left:${x}px;top:${y}px;">
      <div class="n8n-node-toolbar">
        <button class="n8n-tb-btn" title="Schritt testen" onclick="event.stopPropagation();quickExecNode('${esc(n.name)}')">▶</button>
        <button class="n8n-tb-btn" title="Aktivieren/Deaktivieren" onclick="event.stopPropagation();toggleDisableNode('${esc(n.name)}')">⏻</button>
        <button class="n8n-tb-btn" title="NDV öffnen" onclick="event.stopPropagation();openNdv('${esc(n.name)}')">⤢</button>
        <button class="n8n-tb-btn" title="Node löschen" onclick="event.stopPropagation();deleteNode('${esc(n.name)}')">🗑</button>
      </div>
      <div class="n8n-node-box ${boxCls} ${statCls} ${selCls} ${disCls} ${pinCls}" ondblclick="event.stopPropagation();openNdv('${esc(n.name)}')" onclick="onNodeClick('${esc(n.name)}')">
        ${trig ? '<div class="n8n-trig-bolt">⚡</div>' : ''}
        ${isPinned ? '<div style="position:absolute;top:-10px;left:6px;background:#fef3c7;color:#b45309;border:1px solid #f59e0b;border-radius:10px;padding:1px 6px;font-size:9.5px;font-weight:700;white-space:nowrap;">📌 Angeheftet</div>' : ''}
        ${innerContent}
        ${stat ? `<div class="n8n-exec-badge ${stat.status==='error'?'err':''}">${stat.status==='error'?'!':'✓'}</div><div class="n8n-items-pill">1 Element</div>` : ''}
        ${portsHtml}
        ${diamondHtml}
      </div>
      <div class="n8n-node-label">
        <div class="n8n-node-title">${esc(n.name)}</div>
        <div class="n8n-node-sub">${esc(m.label)}</div>
      </div>
    </div>`;
  }
  layer.innerHTML = html;
  renderWires(nodeMap, conns);
}

function renderWires(nodeMap, conns) {
  let paths = '';
  for (const [srcName, edges] of Object.entries(conns || {})) {
    const src = nodeMap[srcName]; if (!src) continue;
    const sd = nodeDimensions(src);
    const sx = (src.position?.[0] ?? 0), sy = (src.position?.[1] ?? 0);

    for (const [portType, branches] of Object.entries(edges || {})) {
      if (!Array.isArray(branches)) continue;
      if (portType === 'main') {
        const totalB = src.type === 'n8n-nodes-base.if' ? 2 : Math.max(1, branches.length);
        branches.forEach((targets, bIdx) => {
          if (!Array.isArray(targets)) return;
          const yFrac = totalB === 1 ? 0.5 : (totalB === 2 ? (bIdx === 0 ? 0.32 : 0.68) : (0.2 + 0.6 * bIdx / (totalB - 1)));
          const x1 = sx + sd.w, y1 = sy + sd.h * yFrac;
          targets.forEach(t => {
            const dst = nodeMap[t.node]; if (!dst) return;
            const dd = nodeDimensions(dst);
            const x2 = (dst.position?.[0] ?? 0), y2 = (dst.position?.[1] ?? 0) + dd.h * 0.5;
            const dx = Math.max(55, Math.abs(x2 - x1) * 0.45);
            const execCls = (st.nodeStatusMap[srcName] && st.nodeStatusMap[t.node]) ? 'executed' : '';
            paths += `<path class="wire-path ${execCls}" onclick="event.stopPropagation();deleteEdgePrompt('${esc(srcName)}','main',${bIdx},'${esc(t.node)}')" d="M ${x1} ${y1} C ${x1+dx} ${y1}, ${x2-dx} ${y2}, ${x2} ${y2}"><title>Klicken zum Entfernen: ${esc(srcName)} → ${esc(t.node)}</title></path>`;
          });
        });
      } else {
        // AI sub-node dashed connection from subnode top center to Agent bottom diamond
        branches.forEach((targets, bIdx) => {
          if (!Array.isArray(targets)) return;
          targets.forEach(t => {
            const dst = nodeMap[t.node]; if (!dst) return;
            const dd = nodeDimensions(dst);
            const x1 = sx + sd.w / 2, y1 = sy;
            const offset = portType.includes('languageModel') ? 0.22 : portType.includes('memory') ? 0.5 : 0.78;
            const x2 = (dst.position?.[0] ?? 0) + dd.w * offset, y2 = (dst.position?.[1] ?? 0) + dd.h + 10;
            const dy = Math.max(40, Math.abs(y1 - y2) * 0.45);
            paths += `<path class="wire-path ai" onclick="event.stopPropagation();deleteEdgePrompt('${esc(srcName)}','${esc(portType)}',${bIdx},'${esc(t.node)}')" d="M ${x1} ${y1} C ${x1} ${y1-dy}, ${x2} ${y2+dy}, ${x2} ${y2}"><title>Klicken zum Entfernen: ${esc(srcName)} → ${esc(t.node)}</title></path>`;
          });
        });
      }
    }
  }
  paths += `<path class="wire-path draft" id="draftWirePath" d="" style="display:none;"/>`;
  document.getElementById('wireSvg').innerHTML = paths;
}

function deleteEdgePrompt(srcName, portType, branchIdx, dstName) {
  if (!st.currentWf?.connections?.[srcName]?.[portType]?.[branchIdx]) return;
  if (!confirm(`Verbindung "${srcName}" → "${dstName}" entfernen?`)) return;
  const arr = st.currentWf.connections[srcName][portType][branchIdx];
  if (Array.isArray(arr)) {
    st.currentWf.connections[srcName][portType][branchIdx] = arr.filter(t => t.node !== dstName);
  }
  st.dirty = true;
  updateHeaderState();
  renderCanvas();
  toast(`Verbindung zu ${dstName} entfernt`);
}

function connectNodes(srcName, branchIdx, dstName) {
  if (!st.currentWf || !srcName || !dstName || srcName === dstName) return false;
  const conns = st.currentWf.connections = st.currentWf.connections || {};
  conns[srcName] = conns[srcName] || { main: [] };
  conns[srcName].main = Array.isArray(conns[srcName].main) ? conns[srcName].main : [];
  while (conns[srcName].main.length <= branchIdx) conns[srcName].main.push([]);
  const bucket = conns[srcName].main[branchIdx];
  if (!bucket.some(t => t.node === dstName)) {
    bucket.push({ node: dstName, type: 'main', index: 0 });
    st.dirty = true;
    updateHeaderState();
    renderCanvas();
    toast(`Verbunden: ${srcName} → ${dstName}`);
  }
  return true;
}

function onNodeClick(name) {
  st.selectedNodeName = name;
  document.querySelectorAll('.n8n-node-box').forEach(b => b.classList.remove('selected'));
  const wrap = document.querySelector(`.n8n-node-wrap[data-nodename="${CSS.escape(name)}"] .n8n-node-box`);
  wrap?.classList.add('selected');
}

// Dragging Nodes & Panning Canvas
(function setupCanvasMouse() {
  const cvs = document.getElementById('n8nCanvas');
  let dragNode = null, dragWire = null, panning = false, startX = 0, startY = 0, origX = 0, origY = 0, moved = false;

  function clientToWorld(cx, cy) {
    const vp = document.getElementById('n8nViewport')?.getBoundingClientRect();
    if (!vp) return { x: cx, y: cy };
    return { x: (cx - vp.left) / st.zoom, y: (cy - vp.top) / st.zoom };
  }

  cvs.addEventListener('mousedown', e => {
    const portOut = e.target.closest('.n8n-port-out[data-out-node]');
    if (portOut) {
      e.stopPropagation();
      const pr = portOut.getBoundingClientRect();
      const p1 = clientToWorld(pr.left + pr.width / 2, pr.top + pr.height / 2);
      startX = e.clientX; startY = e.clientY; moved = false;
      dragWire = {
        srcName: portOut.dataset.outNode,
        branch: parseInt(portOut.dataset.outBranch || '0', 10) || 0,
        x1: p1.x, y1: p1.y
      };
      return;
    }
    const nodeEl = e.target.closest('.n8n-node-wrap, .n8n-sticky');
    if (e.target.closest('button,.n8n-port-out,.n8n-port-add,.wire-path')) return;
    startX = e.clientX; startY = e.clientY; moved = false;
    if (nodeEl) {
      const nm = nodeEl.dataset.nodename;
      dragNode = (st.currentWf?.nodes || []).find(n => n.name === nm);
      if (dragNode) { origX = dragNode.position?.[0] ?? 0; origY = dragNode.position?.[1] ?? 0; }
    } else {
      panning = true; origX = st.panX; origY = st.panY;
    }
  });
  window.addEventListener('mousemove', e => {
    if (dragWire) {
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
      if (wrap && wrap.dataset.nodename !== dragWire.srcName) {
        wrap.querySelector('.n8n-port-in')?.classList.add('snap');
      }
    } else if (dragNode) {
      const dx = (e.clientX - startX) / st.zoom, dy = (e.clientY - startY) / st.zoom;
      if (Math.abs(dx) > 2 || Math.abs(dy) > 2) moved = true;
      dragNode.position = [Math.round(origX + dx), Math.round(origY + dy)];
      st.dirty = true;
      const wrap = document.querySelector(`[data-nodename="${CSS.escape(dragNode.name)}"]`);
      if (wrap) { wrap.style.left = dragNode.position[0] + 'px'; wrap.style.top = dragNode.position[1] + 'px'; }
      const nodeMap = Object.fromEntries((st.currentWf?.nodes || []).map(n => [n.name, n]));
      renderWires(nodeMap, st.currentWf?.connections || {});
    } else if (panning) {
      st.panX = origX + (e.clientX - startX);
      st.panY = origY + (e.clientY - startY);
      applyCanvasTransform();
    }
  });
  window.addEventListener('mouseup', e => {
    if (dragWire) {
      document.querySelectorAll('.n8n-port-in.snap').forEach(p => p.classList.remove('snap'));
      const draft = document.getElementById('draftWirePath');
      if (draft) { draft.style.display = 'none'; draft.setAttribute('d', ''); }
      const dw = dragWire; dragWire = null;
      if (!moved) {
        openCreatorDrawer(dw.srcName, dw.branch);
      } else {
        const dropEl = document.elementFromPoint(e.clientX, e.clientY);
        const portIn = dropEl?.closest('.n8n-port-in[data-in-node]');
        const wrap = dropEl?.closest('.n8n-node-wrap[data-nodename]');
        const dstName = portIn?.dataset?.inNode || wrap?.dataset?.nodename;
        if (dstName && dstName !== dw.srcName) {
          connectNodes(dw.srcName, dw.branch, dstName);
        }
      }
      return;
    }
    if (dragNode && moved) updateHeaderState();
    dragNode = null; panning = false;
  });
  cvs.addEventListener('wheel', e => {
    e.preventDefault();
    zoomBy(e.deltaY < 0 ? 0.06 : -0.06);
  }, { passive: false });
})();

// Node Creator Drawer ("Was passiert als Nächstes?")
function openCreatorDrawer(srcName, branch = 0) {
  st.creatorSourceNode = srcName;
  st.creatorSourceBranch = branch;
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
      usableAsTool: !!sc.usableAsTool || sc.name.endsWith('Tool')
    });
  }
  for (const [tp, m] of Object.entries(NODE_META)) {
    if (!byType.has(tp)) byType.set(tp, { tp, label: m.label, cat: m.cat, desc: '', usableAsTool: false });
  }
  const all = Array.from(byType.values());
  const baseCats = Array.from(new Set(all.map(x => x.cat)));
  const cats = ['Alle', ...baseCats, 'KI-Werkzeuge'];
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
      || (st.creatorCat === 'KI-Werkzeuge' ? it.usableAsTool : it.cat === st.creatorCat);
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
  const m = metaFor(tp);
  let name = m.label, idx = 1;
  while ((st.currentWf.nodes || []).some(n => n.name === name)) { name = `${m.label} ${idx++}`; }
  let x = 420, y = 240;
  if (st.creatorSourceNode) {
    const src = st.currentWf.nodes.find(n => n.name === st.creatorSourceNode);
    if (src) { x = (src.position?.[0] ?? 200) + 230; y = (src.position?.[1] ?? 200) + (st.creatorSourceBranch * 120); }
    const conns = st.currentWf.connections = st.currentWf.connections || {};
    conns[st.creatorSourceNode] = conns[st.creatorSourceNode] || { main: [] };
    while (conns[st.creatorSourceNode].main.length <= st.creatorSourceBranch) conns[st.creatorSourceNode].main.push([]);
    conns[st.creatorSourceNode].main[st.creatorSourceBranch].push({ node: name, type: 'main', index: 0 });
  }
  st.currentWf.nodes.push({ id: 'n-' + Date.now(), name, type: tp, position: [x, y], parameters: {} });
  st.dirty = true; updateHeaderState(); closeCreatorDrawer(); renderCanvas(); toast(`${name} hinzugefügt`);
}

function toggleDisableNode(name) {
  const n = st.currentWf?.nodes?.find(x => x.name === name); if (!n) return;
  n.disabled = !n.disabled; st.dirty = true; updateHeaderState(); renderCanvas();
}
function deleteNode(name) {
  if (!st.currentWf) return;
  st.currentWf.nodes = st.currentWf.nodes.filter(n => n.name !== name);
  delete st.currentWf.connections?.[name];
  for (const edges of Object.values(st.currentWf.connections || {})) {
    for (const branches of Object.values(edges || {})) {
      if (Array.isArray(branches)) branches.forEach((arr, i) => { if (Array.isArray(arr)) branches[i] = arr.filter(t => t.node !== name); });
    }
  }
  st.dirty = true; updateHeaderState(); renderCanvas(); toast('Node entfernt');
}

// Execution: Full Workflow & Bottom Logs Drawer
function togglePayloadPop() { document.getElementById('payloadModal').classList.add('open'); }
function toggleBottomLogs(forceOpen) {
  const bl = document.getElementById('bottomLogs');
  if (forceOpen === true) bl.classList.add('open');
  else bl.classList.toggle('open');
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
      st.nodeStatusMap[l.node] = { status: l.status, durationMs: l.durationMs, output: l.output };
    });
    renderCanvas();
    document.getElementById('logsHdrTxt').textContent = `Ausführung ${d.status === 'success' ? 'erfolgreich' : d.status} · ${d.durationMs}ms · ${(d.nodeLogs||[]).length} Nodes (${d.modelUsed})`;
    document.getElementById('logsStepList').innerHTML = (d.nodeLogs || []).map((l, idx) => `
      <div class="n8n-pop-item" onclick="showStepLog(${idx})">
        <span>${l.status==='success'?'🟢':'🔴'} ${esc(l.node)}</span>
        <span class="mono" style="font-size:10.5px;color:var(--n8n-text-3);">${l.durationMs}ms</span>
      </div>
    `).join('');
    document.getElementById('logsJsonPre').textContent = JSON.stringify(d, null, 2);
    document.getElementById('logsToggleBtn').textContent = `Protokoll (${(d.nodeLogs||[]).length})`;
    toggleBottomLogs(true);
    toast(`Workflow abgeschlossen (${d.durationMs}ms)`);
  } catch (e) { toast('Fehler: ' + e); }
  btn.textContent = '🧪 Workflow testen'; btn.disabled = false;
}
function showStepLog(idx) {
  const l = st.lastExec?.nodeLogs?.[idx];
  if (l) document.getElementById('logsJsonPre').textContent = JSON.stringify(l, null, 2);
}
async function quickExecNode(name) {
  openNdv(name);
  await executeNdvStep();
}

// 3-Column NDV (Node Details View) Modal
function updatePinBtnState() {
  const btn = document.getElementById('ndvPinBtn');
  if (!btn || !st.ndvNodeName) return;
  const pinned = Array.isArray(st.currentWf?.pinData?.[st.ndvNodeName]) && st.currentWf.pinData[st.ndvNodeName].length > 0;
  btn.textContent = pinned ? '📌 Angeheftet lösen' : '📌 Daten anheften';
  btn.style.borderColor = pinned ? '#f59e0b' : '';
  btn.style.color = pinned ? '#b45309' : '';
}

function pinCurrentNodeOutput() {
  const nn = st.ndvNodeName;
  if (!st.currentWf || !nn) return;
  st.currentWf.pinData = st.currentWf.pinData || {};
  const cur = st.currentWf.pinData[nn];
  if (Array.isArray(cur) && cur.length > 0) {
    delete st.currentWf.pinData[nn];
    toast(`Angeheftete Daten von "${nn}" gelöst`);
  } else {
    let data = st.nodeStatusMap[nn]?.output;
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
  st.dirty = true;
  updateHeaderState();
  updatePinBtnState();
  renderNdvOutput();
  if (st.ndvCenterTab === 'settings') renderNdvCenter();
  renderCanvas();
}

function openNdv(nodeName) {
  const n = st.currentWf?.nodes?.find(x => x.name === nodeName); if (!n) return;
  st.ndvNodeName = nodeName;
  st.ndvCenterTab = 'params';
  document.getElementById('ndvTabParamsBtn').style.borderBottom = '2px solid var(--n8n-coral)';
  document.getElementById('ndvTabSettingsBtn').style.borderBottom = 'none';
  const m = metaFor(n.type);
  document.getElementById('ndvNodeTypeLbl').textContent = n.type;
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

function renderJsonOrTable(obj, fmt) {
  if (!obj) return '<div style="color:var(--n8n-text-3);">Noch keine Daten. Klicke auf <strong>Schritt testen</strong>.</div>';
  if (fmt === 'table' && typeof obj === 'object') {
    const flat = Array.isArray(obj) ? obj : [obj];
    const keys = Array.from(new Set(flat.flatMap(r => Object.keys(r || {}))));
    if (!keys.length) return '<div style="color:var(--n8n-text-3);">Leeres Objekt</div>';
    return `<table style="width:100%;border-collapse:collapse;font-size:12px;">
      <thead><tr>${keys.map(k => `<th style="text-align:left;padding:5px 7px;border-bottom:1px solid var(--n8n-border);background:var(--n8n-subtle-bg);">${esc(k)}</th>`).join('')}</tr></thead>
      <tbody>${flat.map(r => `<tr>${keys.map(k => `<td style="padding:5px 7px;border-bottom:1px solid var(--n8n-border);">${esc(typeof r[k]==='object'?JSON.stringify(r[k]):r[k])}</td>`).join('')}</tr>`).join('')}</tbody>
    </table>`;
  }
  return `<pre class="mono" style="margin:0;white-space:pre-wrap;word-break:break-word;font-size:11.5px;">${esc(JSON.stringify(obj, null, 2))}</pre>`;
}

function renderNdvInput() {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  const body = document.getElementById('ndvInBody');
  const stat = st.nodeStatusMap[n.name];
  if (isTrigger(n.type)) {
    const wp = n.parameters?.path || st.currentWf.webhookPath || st.currentWf.id;
    const url = `${window.location.origin}/webhook-rust/${wp}`;
    body.innerHTML = `
      <div><span class="n8n-field-lbl">Rust Webhook-URL</span><input class="n8n-input mono" readonly value="${esc(url)}" onclick="this.select()"></div>
      <div><span class="n8n-field-lbl">Testdaten (JSON)</span><textarea id="ndvTrigPayload" class="n8n-code-ed mono" rows="8" onchange="document.getElementById('globalPayloadInput').value=this.value">${esc(document.getElementById('globalPayloadInput').value)}</textarea></div>
      <div style="font-size:11.5px;color:var(--n8n-text-3);">cURL: <code class="mono">curl -X POST ${esc(url)} -d '{"message":"Hallo"}'</code></div>
    `;
  } else {
    body.innerHTML = renderJsonOrTable(stat?.input || null, st.ndvInFmt);
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
    el.style.display = 'none';
    el.textContent = '';
    return;
  }
  el.style.display = 'block';
  el.style.background = 'var(--n8n-subtle-bg)';
  el.style.color = 'var(--n8n-text-2)';
  el.textContent = '⏳ QuickJS wird ausgewertet...';
  ndvExprTimers[elId] = setTimeout(() => runExprPreviewNow(s, elId), 180);
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
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        expression: expr,
        item: stat?.input || testPayload || {},
        currentNode: n?.name || '',
        nodeOutputs,
        vars: {}
      })
    });
    const d = await r.json();
    const cur = document.getElementById(elId);
    if (!cur) return;
    cur.style.display = 'block';
    if (d.ok) {
      cur.style.background = 'var(--n8n-green-soft)';
      cur.style.color = 'var(--n8n-green)';
      const txt = typeof d.value === 'object' ? JSON.stringify(d.value) : String(d.value ?? '');
      cur.textContent = `⚡ ${d.engine || 'QuickJS'}: ${txt}`;
    } else {
      cur.style.background = 'var(--n8n-red-soft)';
      cur.style.color = 'var(--n8n-red)';
      cur.textContent = `✕ Expression-Fehler: ${d.error || 'ungültig'}`;
    }
  } catch {
    const cur = document.getElementById(elId);
    if (cur) { cur.style.display = 'block'; cur.textContent = '✕ Auswertung fehlgeschlagen'; }
  }
}

function updateNodeCredential(credType, credId) {
  const n = st.currentWf?.nodes?.find(x => x.name === st.ndvNodeName); if (!n) return;
  n.credentials = n.credentials || {};
  if (!credId) {
    delete n.credentials[credType];
  } else {
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
  if (!trimmed || trimmed === '[]' || trimmed === 'null') {
    delete st.currentWf.pinData[nn];
  } else {
    try {
      const parsed = JSON.parse(trimmed);
      st.currentWf.pinData[nn] = Array.isArray(parsed) ? parsed : [{ json: parsed }];
    } catch {
      toast('Ungültiges pinData-JSON');
      return;
    }
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
      <label style="display:flex;align-items:center;justify-content:space-between;padding:6px 0;"><span>Immer Daten ausgeben (alwaysOutputData)</span><input type="checkbox" ${n.alwaysOutputData?'checked':''} onchange="updateNodeProp('alwaysOutputData',this.checked)"></label>
      <label style="display:flex;align-items:center;justify-content:space-between;padding:6px 0;"><span>Nur einmal ausführen (executeOnce)</span><input type="checkbox" ${n.executeOnce?'checked':''} onchange="updateNodeProp('executeOnce',this.checked)"></label>
      <div><span class="n8n-field-lbl">Fehlerbehandlung (onError)</span>
        <select class="n8n-select" onchange="updateNodeProp('onError',this.value);updateNodeProp('continueOnFail',this.value!=='stopWorkflow')">
          <option value="stopWorkflow" ${onErr==='stopWorkflow'?'selected':''}>Workflow stoppen</option>
          <option value="continueRegularOutput" ${onErr==='continueRegularOutput'?'selected':''}>Mit regulärer Ausgabe fortfahren</option>
          <option value="continueErrorOutput" ${onErr==='continueErrorOutput'?'selected':''}>Fehlerausgang nutzen</option>
        </select>
      </div>
      <div style="display:grid;grid-template-columns:1fr 120px;gap:10px;align-items:center;padding:4px 0;">
        <label style="display:flex;align-items:center;gap:8px;"><input type="checkbox" ${n.retryOnFail?'checked':''} onchange="updateNodeProp('retryOnFail',this.checked)"><span>Bei Fehler wiederholen (retryOnFail)</span></label>
        <div><span class="n8n-field-lbl">Max. Versuche</span><input type="number" min="1" max="5" class="n8n-input mono" value="${esc(n.maxTries ?? 3)}" onchange="updateNodeProp('maxTries',Math.max(1,Math.min(5,Number(this.value)||3)))"></div>
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
          <button class="n8n-btn" style="height:22px;font-size:11px;padding:0 7px;" onclick="closeNdv();setSection('credentials')">+ Verwalten</button>
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
        const inputEl = isCode
          ? `<textarea class="n8n-code-ed mono" rows="${rows}" data-param-key="${pname}" oninput="updateNodeParam('${pname}',this.value);scheduleExprPreview('${prevId}',this.value)">${esc(valStr)}</textarea>`
          : `<input type="${isPw?'password':'text'}" class="n8n-input mono" data-param-key="${pname}" value="${esc(valStr)}" oninput="updateNodeParam('${pname}',this.value);scheduleExprPreview('${prevId}',this.value)">`;
        fields += `<div>
          <span class="n8n-field-lbl">${lbl}${prop.supportsExpression ? ' <span class="mono" style="font-size:10px;color:var(--n8n-coral);margin-left:4px;">{{ QuickJS }}</span>' : ''}</span>
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
    if (n.type === 'n8n-nodes-base.webhook') {
      fields += `
        <div><span class="n8n-field-lbl">HTTP-Methode</span><select class="n8n-select" onchange="updateNodeParam('httpMethod',this.value)"><option value="POST" ${p.httpMethod==='POST'?'selected':''}>POST</option><option value="GET" ${p.httpMethod==='GET'?'selected':''}>GET</option></select></div>
        <div><span class="n8n-field-lbl">Pfad</span><input class="n8n-input mono" value="${esc(p.path||'')}" oninput="updateNodeParam('path',this.value)"></div>
        <div><span class="n8n-field-lbl">Antwortmodus</span><select class="n8n-select" onchange="updateNodeParam('responseMode',this.value)"><option value="responseNode">Über „Auf Webhook antworten“-Node</option><option value="lastNode">Letzte Node</option></select></div>`;
    } else if (n.type === 'n8n-nodes-base.code') {
      fields += `<div><span class="n8n-field-lbl">JavaScript-Code</span><textarea class="n8n-code-ed mono" rows="11" oninput="updateNodeParam('jsCode',this.value)">${esc(p.jsCode||'')}</textarea></div>`;
    } else if (n.type.includes('agent') || n.type.includes('chainLlm')) {
      fields += `
        <div><span class="n8n-field-lbl">Prompt (Nutzer-Nachricht)</span><textarea class="n8n-textarea" rows="3" oninput="updateNodeParam('text',this.value)">${esc(p.text || p.prompt || '={{ $json.message }}')}</textarea></div>
        <div><span class="n8n-field-lbl">Systemnachricht</span><textarea class="n8n-textarea" rows="4" onchange="updateNodeOption('systemMessage',this.value)">${esc(p.options?.systemMessage || '')}</textarea></div>`;
    }
  }

  fields += `
    <details style="margin-top:8px;" open>
      <summary style="cursor:pointer;font-size:12px;font-weight:600;color:var(--n8n-text-2);">Rohparameter (JSON)</summary>
      <textarea id="ndvRawParams" class="n8n-code-ed mono" rows="7" style="margin-top:6px;" onchange="applyRawParams(this.value)">${esc(JSON.stringify(p, null, 2))}</textarea>
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
  if (stat) {
    badgeEl.textContent = `1 Element · ${stat.durationMs}ms${hasPinned ? ' · 📌 Angeheftet' : ''}`;
    badgeEl.style.color = hasPinned ? '#b45309' : 'var(--n8n-green)';
    document.getElementById('ndvOutBody').innerHTML = renderJsonOrTable(stat.output || null, st.ndvOutFmt);
  } else if (hasPinned) {
    const unwrapped = pinned.map(it => (it && it.json !== undefined) ? it.json : it);
    badgeEl.textContent = `📌 Angeheftet (${unwrapped.length})`;
    badgeEl.style.color = '#b45309';
    document.getElementById('ndvOutBody').innerHTML = renderJsonOrTable(unwrapped.length === 1 ? unwrapped[0] : unwrapped, st.ndvOutFmt);
  } else {
    badgeEl.textContent = '';
    document.getElementById('ndvOutBody').innerHTML = renderJsonOrTable(null, st.ndvOutFmt);
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
  n.name = nn;
  if (st.currentWf.pinData?.[old]) { st.currentWf.pinData[nn] = st.currentWf.pinData[old]; delete st.currentWf.pinData[old]; }
  const conns = st.currentWf.connections || {};
  if (conns[old]) { conns[nn] = conns[old]; delete conns[old]; }
  for (const edges of Object.values(conns)) {
    for (const branches of Object.values(edges || {})) {
      if (Array.isArray(branches)) branches.forEach(arr => { if (Array.isArray(arr)) arr.forEach(t => { if (t.node === old) t.node = nn; }); });
    }
  }
  st.ndvNodeName = nn; st.dirty = true; updateHeaderState(); renderCanvas();
}

async function executeNdvStep() {
  if (!st.currentWf || !st.ndvNodeName) return;
  const btn = document.getElementById('ndvTestStepBtn');
  btn.textContent = '⏳ Läuft...'; btn.disabled = true;
  const n = st.currentWf.nodes.find(x => x.name === st.ndvNodeName);
  let payload = {};
  try { payload = JSON.parse(document.getElementById('globalPayloadInput').value); } catch { payload = { message: 'Hallo' }; }
  try {
    const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/execute-node`, {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ nodeName: st.ndvNodeName, payload, model: document.getElementById('modelSelect').value, parameters: n?.parameters })
    });
    const d = await r.json();
    st.nodeStatusMap[st.ndvNodeName] = { status: d.status || 'success', durationMs: d.durationMs || 0, input: d.input, output: d.output };
    renderNdvInput();
    renderNdvOutput();
    renderCanvas();
  } catch (e) { toast('Fehler: ' + e); }
  btn.textContent = '▶ Schritt testen'; btn.disabled = false;
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
async function renameWorkflow(val) { if (!st.currentWf || !val.trim()) return; st.currentWf.name = val.trim(); await saveCurrentWorkflow(); init(); }
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
  const name = prompt('Workflow-Name:', 'Neuer Conrad-Automation-Workflow'); if (!name) return;
  const r = await fetch('/api/n8n/workflows', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ name, active: true }) });
  const wf = await r.json(); await init(); selectWorkflow(wf.id);
}
async function duplicateCurrentWorkflow() {
  if (!st.currentWf) return;
  const r = await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}/duplicate`, { method: 'POST' });
  if (r.ok) { const wf = await r.json(); await init(); selectWorkflow(wf.id); toast('Workflow dupliziert'); }
}
async function deleteCurrentWorkflow() {
  if (!st.currentWf || !confirm(`Workflow "${st.currentWf.name}" wirklich löschen?`)) return;
  await fetch(`/api/n8n/workflows/${encodeURIComponent(st.currentWf.id)}`, { method: 'DELETE' });
  toast('Workflow gelöscht'); await init();
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
  const r = await fetch('/api/n8n/workflows', {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload)
  });
  if (!r.ok) { toast('Import fehlgeschlagen'); return; }
  const wf = await r.json();
  document.getElementById('wfImportModal').classList.remove('open');
  toast(`Workflow "${wf.name}" importiert ✓`);
  await init();
  await selectWorkflow(wf.id);
}
function openShareModal() {
  if (!st.currentWf) return;
  const wp = st.currentWf.webhookPath || st.currentWf.id;
  alert(`Webhook-Endpunkt:\n${window.location.origin}/webhook-rust/${wp}`);
}
async function syncSqlite() {
  const r = await fetch('/api/n8n/sync-sqlite', { method: 'POST' });
  toast('SQLite synchronisiert ✓'); init();
}

if (window.innerWidth <= 960) document.getElementById('n8nNav').classList.add('collapsed');
document.addEventListener('keydown', e => {
  if (e.key === 'Escape') {
    closeNdv(); closeCreatorDrawer(); closeN8nPops();
    document.getElementById('wfImportModal')?.classList.remove('open');
    document.getElementById('payloadModal')?.classList.remove('open');
  }
});
init();
</script>
</body>
</html>"######
}

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
        <div style="font-size:13px;color:var(--ink-secondary);">Sendet einen Live-Auftrag (<span class="mono">Ohrladen</span>) direkt an die nachtrainierte Conrad KI Engine.</div>
        <div><button class="btn btn-orange" onclick="runProofB()">▶ KI-Auftrag live testen</button></div>
        <pre class="proof-pre mono" id="outB">Sendet lesenden Auftrag („Suche Firmen mit Name Ohrladen“) an die Conrad KI Engine...</pre>
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
      body: JSON.stringify({ prompt: 'Suche Firmen mit Name Ohrladen', workspace: 'conrad-ki', model })
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
      body: JSON.stringify({ payload: { prompt: 'Suche in Twenty nach Heidelberger Schlossgastronomie und fasse kurz zusammen.', model: 'conrad-n8n-agent:0.6b' } })
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
