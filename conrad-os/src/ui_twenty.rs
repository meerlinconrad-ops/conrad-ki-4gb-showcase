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
  --t-bg-hover: #f1f1f1;
  --t-bg-selected: #edf3ff;
  --t-bg-elevated: #ffffff;
  --t-border: #ebebeb;
  --t-border-strong: #d6d6d6;
  --t-text-primary: #171717;
  --t-text-secondary: #666666;
  --t-text-tertiary: #999999;
  --t-accent: #171717;
  --t-focus-ring: #1961ed;
  --t-blue: #1961ed;
  --t-blue-soft: #edf3ff;
  --t-blue-border: #bfd6fe;
  --t-green: #15803d;
  --t-green-soft: #ecfdf3;
  --t-amber: #b45309;
  --t-amber-soft: #fffbeb;
  --t-purple: #6d28d9;
  --t-purple-soft: #f5f3ff;
  --t-cyan: #0e7490;
  --t-cyan-soft: #ecfeff;
  --t-red: #dc2626;
  --t-red-soft: #fef2f2;
  --t-shadow-sm: 0 1px 2px rgba(0,0,0,0.04);
  --t-shadow-md: 0 4px 16px rgba(0,0,0,0.08), 0 0 0 1px rgba(0,0,0,0.04);
  --t-shadow-lg: 0 16px 40px rgba(0,0,0,0.14);
}
html.dark {
  --t-bg-primary: #141414;
  --t-bg-secondary: #171717;
  --t-bg-tertiary: #202020;
  --t-bg-hover: #262626;
  --t-bg-selected: #1b2740;
  --t-bg-elevated: #1c1c1c;
  --t-border: #282828;
  --t-border-strong: #3b3b3b;
  --t-text-primary: #ebebeb;
  --t-text-secondary: #a0a0a0;
  --t-text-tertiary: #6b6b6b;
  --t-accent: #ebebeb;
  --t-focus-ring: #3b82f6;
  --t-blue: #60a5fa;
  --t-blue-soft: rgba(59,130,246,0.15);
  --t-blue-border: rgba(59,130,246,0.35);
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
  --t-shadow-md: 0 4px 16px rgba(0,0,0,0.35), 0 0 0 1px rgba(255,255,255,0.06);
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
  user-select: none; transition: margin-left 0.16s ease; z-index: 30; position: relative;
}
.t-drawer.collapsed { margin-left: -220px; }
.t-ws-top {
  height: 44px; padding: 0 10px; display: flex; align-items: center; justify-content: space-between;
  gap: 6px; position: relative;
}
.t-ws-btn {
  flex: 1; min-width: 0; display: flex; align-items: center; gap: 7px; padding: 4px 6px;
  border-radius: 5px; border: 1px solid transparent; background: transparent; cursor: pointer;
  text-align: left;
}
.t-ws-btn:hover { background: var(--t-bg-hover); }
.t-logo-sq {
  width: 20px; height: 20px; border-radius: 5px; background: var(--t-text-primary);
  color: var(--t-bg-primary); font-weight: 700; font-size: 10px; flex-shrink: 0;
  display: inline-flex; align-items: center; justify-content: center; letter-spacing: -0.04em;
}
.t-ws-name { font-weight: 600; font-size: 13px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.t-icon-btn {
  width: 26px; height: 26px; border-radius: 5px; border: 1px solid transparent;
  background: transparent; color: var(--t-text-secondary); cursor: pointer;
  display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;
}
.t-icon-btn:hover { background: var(--t-bg-hover); color: var(--t-text-primary); }

.t-nav-scroll { flex: 1; overflow-y: auto; padding: 4px 8px 12px; display: flex; flex-direction: column; gap: 1px; }
.t-nav-section {
  font-size: 11px; font-weight: 500; color: var(--t-text-tertiary);
  padding: 12px 8px 4px; display: flex; align-items: center; justify-content: space-between;
}
.t-nav-item {
  height: 28px; padding: 0 8px; border-radius: 5px; display: flex; align-items: center;
  justify-content: space-between; gap: 8px; cursor: pointer; color: var(--t-text-secondary);
  font-weight: 500; font-size: 13px; border: none; background: transparent; width: 100%; text-align: left;
}
.t-nav-item:hover { background: var(--t-bg-hover); color: var(--t-text-primary); }
.t-nav-item.active { background: var(--t-bg-tertiary); color: var(--t-text-primary); font-weight: 600; }
.t-nav-left { display: flex; align-items: center; gap: 8px; min-width: 0; overflow: hidden; }
.t-nav-left span { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.t-nav-count { font-size: 11px; color: var(--t-text-tertiary); font-variant-numeric: tabular-nums; }
.t-fav-del { opacity: 0; font-size: 12px; color: var(--t-text-tertiary); padding: 0 3px; border-radius: 3px; }
.t-nav-item:hover .t-fav-del { opacity: 1; }
.t-fav-del:hover { color: var(--t-red); background: var(--t-bg-primary); }
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
.t-breadcrumb { display: flex; align-items: center; gap: 7px; font-size: 13px; font-weight: 600; color: var(--t-text-primary); min-width: 0; }
.t-hdr-right { display: flex; align-items: center; gap: 6px; flex-wrap: nowrap; }

.t-btn {
  height: 28px; padding: 0 10px; border-radius: 5px; border: 1px solid var(--t-border-strong);
  background: var(--t-bg-primary); color: var(--t-text-primary); font-size: 12.5px; font-weight: 500;
  cursor: pointer; display: inline-flex; align-items: center; gap: 5px; white-space: nowrap;
  box-shadow: var(--t-shadow-sm);
}
.t-btn:hover { background: var(--t-bg-hover); }
.t-btn:disabled { opacity: 0.45; cursor: not-allowed; }
.t-btn-primary {
  background: var(--t-text-primary); color: var(--t-bg-primary); border-color: var(--t-text-primary);
}
.t-btn-primary:hover { opacity: 0.9; background: var(--t-text-primary); }
.t-btn-blue {
  background: var(--t-blue); color: #ffffff; border-color: var(--t-blue);
}
.t-btn-blue:hover { opacity: 0.92; background: var(--t-blue); }
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
.t-vb-left { display: flex; align-items: center; gap: 4px; position: relative; }
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
.t-search-inline:focus { border-color: var(--t-focus-ring); background: var(--t-bg-primary); }
.t-search-wrap { position: relative; display: inline-flex; align-items: center; }
.t-search-wrap > span { position: absolute; left: 8px; top: 50%; transform: translateY(-50%); color: var(--t-text-tertiary); pointer-events: none; display: inline-flex; align-items: center; }
.t-search-wrap svg { display: block; }

/* Filter & Sort Chips Row (SortOrFilterChip blue style) */
.t-chips-bar {
  min-height: 38px; padding: 5px 12px; border-bottom: 1px solid var(--t-border);
  background: var(--t-bg-secondary); display: none; align-items: center; justify-content: space-between; gap: 8px; flex-wrap: wrap;
}
.t-chips-bar.visible { display: flex; }
.t-chips-list { display: flex; align-items: center; gap: 6px; flex-wrap: wrap; }
.t-chip-filter {
  height: 24px; padding: 0 4px 0 7px; border-radius: 5px; border: 1px solid var(--t-blue-border);
  background: var(--t-blue-soft); font-size: 12px; color: var(--t-blue);
  display: inline-flex; align-items: center; gap: 5px; font-weight: 500;
}
.t-chip-filter strong { font-weight: 600; }
.t-chip-filter button {
  border: none; background: transparent; cursor: pointer; padding: 0 2px; color: var(--t-blue);
  display: inline-flex; align-items: center; border-radius: 3px; height: 18px;
}
.t-chip-filter button:hover { background: rgba(25,97,237,0.14); }

/* Popover menus */
.t-popover {
  position: absolute; top: 34px; right: 0; z-index: 65; min-width: 235px;
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
.t-pop-sep { border-top: 1px solid var(--t-border); margin: 4px 0; }

/* Floating Context Dropdown (for Column Header / Inline Select / + Column) */
.t-float-menu {
  position: fixed; z-index: 85; min-width: 210px; max-width: 280px;
  background: var(--t-bg-elevated); border: 1px solid var(--t-border-strong); border-radius: 8px;
  box-shadow: var(--t-shadow-md); padding: 5px; display: none;
}
.t-float-menu.open { display: block; }

/* RecordTable (Spreadsheet Grid) */
.t-table-wrap { flex: 1; overflow: auto; position: relative; background: var(--t-bg-primary); }
table.t-grid {
  width: 100%; border-collapse: separate; border-spacing: 0; table-layout: fixed;
  min-width: 820px;
}
table.t-grid thead th {
  position: sticky; top: 0; z-index: 12; height: 33px; padding: 0 8px;
  background: var(--t-bg-primary); border-bottom: 1px solid var(--t-border); border-right: 1px solid var(--t-border);
  font-size: 12px; font-weight: 500; color: var(--t-text-secondary); text-align: left; white-space: nowrap;
  user-select: none; cursor: pointer; position: sticky;
}
table.t-grid thead th:hover { background: var(--t-bg-secondary); color: var(--t-text-primary); }
table.t-grid thead th.sticky-col, table.t-grid tbody td.sticky-col, table.t-grid tfoot td.sticky-col {
  position: sticky; left: 32px; z-index: 13; background: var(--t-bg-primary);
  box-shadow: 2px 0 4px -2px rgba(0,0,0,0.06);
}
table.t-grid thead th.sticky-chk, table.t-grid tbody td.sticky-chk, table.t-grid tfoot td.sticky-chk {
  position: sticky; left: 0; z-index: 13; background: var(--t-bg-primary); text-align: center; padding: 0;
}
table.t-grid thead th.sticky-col, table.t-grid thead th.sticky-chk { z-index: 15; }
.t-th-inner { display: flex; align-items: center; gap: 6px; overflow: hidden; text-overflow: ellipsis; padding-right: 6px; }
.t-th-inner svg { color: var(--t-text-tertiary); flex-shrink: 0; }
.t-th-sort-ind { margin-left: auto; font-size: 11px; color: var(--t-blue); }
.t-col-resizer {
  position: absolute; top: 0; right: -3px; width: 6px; height: 100%;
  cursor: col-resize; z-index: 20;
}
.t-col-resizer:hover, .t-col-resizer.resizing { background: var(--t-blue); opacity: 0.6; }
th.t-th-add-col {
  width: 36px !important; min-width: 36px; text-align: center !important; padding: 0 !important;
  color: var(--t-text-tertiary); border-right: none !important;
}
th.t-th-add-col:hover { color: var(--t-text-primary); background: var(--t-bg-secondary); }

table.t-grid tbody td {
  height: 33px; padding: 0 8px; border-bottom: 1px solid var(--t-border); border-right: 1px solid var(--t-border);
  font-size: 13px; color: var(--t-text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  vertical-align: middle; position: relative; cursor: pointer;
}
table.t-grid tbody tr:hover td { background: var(--t-bg-secondary); }
table.t-grid tbody tr:hover td.sticky-col, table.t-grid tbody tr:hover td.sticky-chk { background: var(--t-bg-secondary); }
table.t-grid tbody tr.selected td { background: var(--t-bg-selected); }
table.t-grid tbody tr.selected td.sticky-col, table.t-grid tbody tr.selected td.sticky-chk { background: var(--t-bg-selected); }
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
  width: 16px; height: 16px; border-radius: 4px; flex-shrink: 0;
  display: inline-flex; align-items: center; justify-content: center; overflow: hidden;
  font-size: 9px; font-weight: 600;
}
.t-avatar-sm {
  width: 18px; height: 18px; border-radius: 99px; font-size: 9.5px; font-weight: 600;
  display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;
}

/* Field Pills */
.t-link-wrap { display: inline-flex; align-items: center; gap: 4px; max-width: 100%; }
.t-link-pill {
  display: inline-flex; align-items: center; gap: 4px; max-width: 100%; padding: 1px 6px;
  border-radius: 4px; background: var(--t-bg-tertiary); border: 1px solid var(--t-border);
  color: var(--t-text-secondary); font-size: 12px; overflow: hidden; text-overflow: ellipsis;
}
.t-link-ext {
  opacity: 0; color: var(--t-text-tertiary); font-size: 11px; padding: 1px 3px; border-radius: 3px;
}
td:hover .t-link-ext { opacity: 1; }
.t-link-ext:hover { color: var(--t-text-primary); background: var(--t-bg-hover); }
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
table.t-grid tfoot td.t-agg-click { cursor: pointer; }
table.t-grid tfoot td.t-agg-click:hover { background: var(--t-bg-hover); color: var(--t-text-primary); }
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
  min-width: 262px; width: 262px; background: var(--t-bg-tertiary); border: 1px solid var(--t-border);
  border-radius: 8px; display: flex; flex-direction: column; max-height: 100%;
  transition: border-color 0.12s ease, background 0.12s ease;
}
.t-kcol.drag-over { border-color: var(--t-blue); background: var(--t-blue-soft); }
.t-kcol-hdr {
  padding: 9px 10px; display: flex; align-items: center; justify-content: space-between;
  border-bottom: 1px solid var(--t-border); gap: 6px;
}
.t-kcol-list { flex: 1; overflow-y: auto; padding: 8px; display: flex; flex-direction: column; gap: 7px; }
.t-kcard {
  background: var(--t-bg-primary); border: 1px solid var(--t-border); border-radius: 6px;
  padding: 10px; cursor: grab; box-shadow: var(--t-shadow-sm); display: flex; flex-direction: column; gap: 6px;
  transition: border-color 0.1s ease, opacity 0.1s ease;
}
.t-kcard:hover { border-color: var(--t-border-strong); }
.t-kcard.dragging { opacity: 0.45; }
.t-kcard-title { font-weight: 600; font-size: 13px; color: var(--t-text-primary); }
.t-kcard-row { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--t-text-secondary); }

/* Settings View */
.t-settings-wrap { flex: 1; display: flex; overflow: hidden; background: var(--t-bg-primary); }
.t-settings-nav {
  width: 220px; min-width: 220px; border-right: 1px solid var(--t-border);
  background: var(--t-bg-secondary); padding: 14px 10px; display: flex; flex-direction: column; gap: 2px;
  overflow-y: auto;
}
.t-settings-body { flex: 1; overflow-y: auto; padding: 24px 32px; max-width: 1020px; }
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
.t-theme-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; margin-top: 10px; }
.t-theme-card {
  border: 2px solid var(--t-border); border-radius: 8px; padding: 12px; cursor: pointer; text-align: left;
  background: var(--t-bg-secondary); display: flex; flex-direction: column; gap: 8px;
}
.t-theme-card.active { border-color: var(--t-blue); background: var(--t-blue-soft); }

/* RightDrawer + Fullscreen ShowPage */
.t-rdrawer {
  position: fixed; top: 0; right: 0; width: 520px; max-width: 100vw; height: 100vh;
  background: var(--t-bg-primary); border-left: 1px solid var(--t-border);
  box-shadow: var(--t-shadow-lg); z-index: 70; display: none; flex-direction: column;
  transition: width 0.16s ease;
}
.t-rdrawer.open { display: flex; }
.t-rdrawer.fullscreen {
  left: 220px; width: calc(100vw - 220px); max-width: 100vw; border-left: none;
}
body.drawer-collapsed .t-rdrawer.fullscreen { left: 0; width: 100vw; }
.t-rd-hdr {
  padding: 10px 14px; border-bottom: 1px solid var(--t-border);
  display: flex; align-items: center; justify-content: space-between; gap: 8px;
  min-height: 52px;
}
.t-rd-tabs {
  display: flex; gap: 2px; padding: 0 14px; border-bottom: 1px solid var(--t-border);
  background: var(--t-bg-secondary); overflow-x: auto; flex-shrink: 0;
}
.t-rd-tab {
  padding: 8px 10px; border: none; border-bottom: 2px solid transparent; background: transparent;
  cursor: pointer; font-size: 12.5px; font-weight: 500; color: var(--t-text-secondary); white-space: nowrap;
}
.t-rd-tab.active { color: var(--t-text-primary); border-bottom-color: var(--t-text-primary); font-weight: 600; }
.t-rd-body { flex: 1; overflow-y: auto; padding: 14px; display: flex; flex-direction: column; gap: 12px; }
.t-rdrawer.fullscreen .t-rd-body {
  padding: 0; display: grid; grid-template-columns: 340px 1fr; gap: 0; overflow: hidden;
}
.t-rd-left-col { display: flex; flex-direction: column; gap: 8px; }
.t-rdrawer.fullscreen .t-rd-left-col {
  border-right: 1px solid var(--t-border); padding: 16px; overflow-y: auto; background: var(--t-bg-secondary);
}
.t-rd-right-col { display: flex; flex-direction: column; gap: 10px; }
.t-rdrawer.fullscreen .t-rd-right-col {
  padding: 16px 24px; overflow-y: auto;
}
.t-prop-row {
  display: grid; grid-template-columns: 128px 1fr; align-items: center; gap: 8px;
  min-height: 30px; padding: 2px 0; border-bottom: 1px solid var(--t-border);
}
.t-prop-lbl { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--t-text-secondary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.t-prop-input {
  width: 100%; border: 1px solid transparent; border-radius: 4px; padding: 4px 6px;
  background: transparent; font-size: 12.5px; outline: none;
}
.t-prop-input:hover { background: var(--t-bg-hover); }
.t-prop-input:focus { background: var(--t-bg-primary); border-color: var(--t-focus-ring); }

/* Ask AI Chat */
.t-chat-msgs { flex: 1; overflow-y: auto; padding: 14px; display: flex; flex-direction: column; gap: 10px; }
.t-bubble {
  padding: 10px 12px; border-radius: 7px; border: 1px solid var(--t-border);
  background: var(--t-bg-secondary); font-size: 12.5px; line-height: 1.5; white-space: pre-wrap;
}
.t-bubble.user { background: var(--t-blue-soft); border-color: transparent; align-self: flex-end; max-width: 88%; }
.t-chat-foot { padding: 10px 12px; border-top: 1px solid var(--t-border); display: flex; flex-direction: column; gap: 8px; }

/* Command Palette & Modals */
.t-backdrop {
  position: fixed; inset: 0; background: rgba(15,23,42,0.42); backdrop-filter: blur(2px);
  z-index: 90; display: none; align-items: flex-start; justify-content: center; padding-top: 72px;
}
.t-backdrop.open { display: flex; }
.t-modal {
  width: 540px; max-width: 94vw; background: var(--t-bg-elevated);
  border: 1px solid var(--t-border-strong); border-radius: 10px; box-shadow: var(--t-shadow-lg); overflow: hidden;
}
.t-cmd-item.cmd-active { background: var(--t-bg-hover); }
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
  .t-rdrawer, .t-rdrawer.fullscreen { width: 100vw; left: 0; }
  .t-rdrawer.fullscreen .t-rd-body { grid-template-columns: 1fr; overflow-y: auto; }
  .t-rdrawer.fullscreen .t-rd-left-col { border-right: none; border-bottom: 1px solid var(--t-border); overflow-y: visible; }
  .t-rdrawer.fullscreen .t-rd-right-col { overflow-y: visible; padding: 14px; }
  .t-settings-wrap { flex-direction: column; }
  .t-settings-nav { width: 100%; min-width: 0; flex-direction: row; overflow-x: auto; border-right: none; border-bottom: 1px solid var(--t-border); padding: 6px 8px; gap: 4px; }
  .t-settings-nav .t-nav-section, .t-settings-nav .t-settings-back { display: none; }
  .t-settings-nav .t-nav-item { width: auto; flex-shrink: 0; }
  .t-settings-body { padding: 16px; }
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
  .t-wh-grid { grid-template-columns: 1fr !important; }
}
</style>
</head>
<body>
<div class="t-app">
  <!-- WorkspaceNavigationDrawer -->
  <aside class="t-drawer" id="tDrawer">
    <div class="t-ws-top">
      <button class="t-ws-btn" onclick="toggleWsMenu(event)" title="Arbeitsbereich wechseln">
        <span class="t-logo-sq" id="wsBadge">CK</span>
        <span class="t-ws-name" id="wsLabel">Conrad KI Demo</span>
        <span id="wsChev"></span>
      </button>
      <button class="t-icon-btn" onclick="toggleDrawer()" title="Sidebar ein-/ausklappen" id="btnCollapse"></button>
      <div class="t-popover left" id="wsPop" style="top:40px;left:8px;min-width:225px;"></div>
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
      <div id="favoritesNavList">
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
      </div>

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
        <span style="display:inline-flex;align-items:center;gap:5px;"><span class="t-status-dot"></span><span>Conrad CRM · Live</span></span>
        <button class="t-icon-btn" onclick="toggleTheme()" id="themeBtn" title="Hell / Dunkel wechseln"></button>
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
        <button class="t-icon-btn" id="favViewBtn" onclick="togglePinCurrentView()" title="Ansicht zu Favoriten hinzufügen"></button>
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

        <div class="t-popover" id="filterPop" style="min-width:275px;">
          <div class="t-pop-title">Schnellfilter</div>
          <div id="filterPopItems"></div>
        </div>
        <div class="t-popover" id="sortPop" style="min-width:240px;">
          <div class="t-pop-title">Sortieren nach</div>
          <div id="sortPopItems"></div>
        </div>
        <div class="t-popover" id="optPop" style="min-width:255px;">
          <div class="t-pop-title">Ansichtsoptionen</div>
          <button class="t-pop-item" id="toggleLayoutBtn" onclick="toggleLayoutMode();closePops()"><span>Ansicht wechseln (Tabelle / Kanban)</span></button>
          <button class="t-pop-item" onclick="openSaveViewModal();closePops()"><span>Aktuelle Ansicht speichern</span></button>
          <button class="t-pop-item" onclick="openFieldsManagerMenu(event)"><span>Sichtbare Spalten verwalten</span><span>›</span></button>
          <div class="t-pop-sep"></div>
          <button class="t-pop-item" onclick="openCsvModal();closePops()"><span>CSV importieren</span></button>
          <button class="t-pop-item" onclick="exportBackendCsv();closePops()"><span>CSV exportieren (vollständig)</span></button>
          <button class="t-pop-item" onclick="exportCurrentJson();closePops()"><span>JSON exportieren</span></button>
          <button class="t-pop-item" onclick="syncSqlDump();closePops()"><span>Demo-Datenbestand zurücksetzen</span></button>
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

<!-- Floating Context Menu (Column Head / + Column / Inline Select) -->
<div class="t-float-menu" id="floatMenu"></div>

<!-- RightDrawer / Fullscreen ShowPage -->
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
      <button class="t-icon-btn" onclick="togglePinOpenRecord()" title="Zu Favoriten hinzufügen" id="rdFavBtn"></button>
      <button class="t-icon-btn" onclick="toggleRecordFullscreen()" title="Im Vollbild öffnen" id="rdFullBtn"></button>
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
    <span style="font-size:12px;font-weight:600;color:var(--t-text-primary);">⚡ Conrad KI Engine · Spezialisiert für CRM &amp; Prozesse</span>
  </div>
  <div class="t-chat-msgs" id="aiMsgs">
    <div class="t-bubble"><strong>Conrad KI-Copilot</strong><br>Spezialisierte Conrad KI Engine für Ihr Tagesgeschäft — sucht Datensätze, legt Aufgaben &amp; Notizen an, steuert Vertriebsphasen und erstellt sichere E-Mail-Entwürfe (<code>draftOnly: true</code>).</div>
  </div>
  <div class="t-chat-foot">
    <div style="display:flex;gap:5px;flex-wrap:wrap;">
      <button class="t-btn" style="height:24px;font-size:11.5px;" onclick="quickAi('Zeige Top ICP Firmen')">Top-ICP-Firmen</button>
      <button class="t-btn" style="height:24px;font-size:11.5px;" onclick="quickAi('Erstelle Task: Elektro Weber anrufen morgen um 10:00')">+ Aufgabe morgen</button>
      <button class="t-btn" style="height:24px;font-size:11.5px;" onclick="quickAi('Suche Firma Elektro Weber und fasse zusammen')">Suche Elektro Weber</button>
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
      <input id="cmdInput" placeholder="Einträge, Ansichten, Einstellungen oder Befehl suchen..." style="flex:1;border:none;outline:none;background:transparent;font-size:13.5px;" oninput="runCmdSearch()" onkeydown="onCmdKey(event)">
      <span class="t-kbd">ESC</span>
    </div>
    <div id="cmdResults" style="max-height:380px;overflow-y:auto;padding:6px;"></div>
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

<!-- Save View Modal -->
<div class="t-backdrop" id="saveViewBackdrop" onclick="if(event.target===this)closeSaveViewModal()">
  <div class="t-modal" style="width:420px;">
    <div style="padding:12px 16px;border-bottom:1px solid var(--t-border);display:flex;justify-content:space-between;align-items:center;">
      <strong>Ansicht speichern</strong>
      <button class="t-icon-btn" onclick="closeSaveViewModal()">×</button>
    </div>
    <div style="padding:16px;display:flex;flex-direction:column;gap:10px;">
      <label style="font-size:12px;color:var(--t-text-secondary);">Name der Ansicht</label>
      <input id="saveViewNameInput" class="t-prop-input" style="border:1px solid var(--t-border-strong);padding:7px 9px;" placeholder="z.B. Rhein-Neckar ICP">
    </div>
    <div style="padding:10px 16px;border-top:1px solid var(--t-border);display:flex;justify-content:flex-end;gap:8px;background:var(--t-bg-secondary);">
      <button class="t-btn" onclick="closeSaveViewModal()">Abbrechen</button>
      <button class="t-btn t-btn-blue" onclick="submitSaveViewModal()">Speichern</button>
    </div>
  </div>
</div>

<!-- Quick Create Field Modal (from + Column) -->
<div class="t-backdrop" id="quickFieldBackdrop" onclick="if(event.target===this)closeQuickFieldModal()">
  <div class="t-modal" style="width:440px;">
    <div style="padding:12px 16px;border-bottom:1px solid var(--t-border);display:flex;justify-content:space-between;align-items:center;">
      <strong>Neues Feld anlegen</strong>
      <button class="t-icon-btn" onclick="closeQuickFieldModal()">×</button>
    </div>
    <div style="padding:16px;display:flex;flex-direction:column;gap:10px;">
      <div>
        <div style="font-size:11.5px;color:var(--t-text-secondary);margin-bottom:4px;">Anzeigename *</div>
        <input id="qfLabel" class="t-prop-input" style="border:1px solid var(--t-border-strong);padding:6px 8px;" placeholder="z.B. Branche oder Priorität">
      </div>
      <div>
        <div style="font-size:11.5px;color:var(--t-text-secondary);margin-bottom:4px;">Datentyp</div>
        <select id="qfType" class="t-prop-input" style="border:1px solid var(--t-border-strong);padding:6px 8px;">
          <option value="TEXT">Text (TEXT)</option>
          <option value="NUMBER">Zahl (NUMBER)</option>
          <option value="CURRENCY">Währung EUR (CURRENCY)</option>
          <option value="SELECT">Auswahl (SELECT)</option>
          <option value="BOOLEAN">Ja / Nein (BOOLEAN)</option>
          <option value="DATE">Datum (DATE)</option>
          <option value="URL">Link (URL)</option>
        </select>
      </div>
      <div>
        <div style="font-size:11.5px;color:var(--t-text-secondary);margin-bottom:4px;">Optionen (nur für Auswahl, kommagetrennt)</div>
        <input id="qfOptions" class="t-prop-input" style="border:1px solid var(--t-border-strong);padding:6px 8px;" placeholder="Handwerk, IT, Gesundheit">
      </div>
      <div>
        <div style="font-size:11.5px;color:var(--t-text-secondary);margin-bottom:4px;">Standardwert (optional)</div>
        <input id="qfDefault" class="t-prop-input" style="border:1px solid var(--t-border-strong);padding:6px 8px;" placeholder="Optional">
      </div>
    </div>
    <div style="padding:10px 16px;border-top:1px solid var(--t-border);display:flex;justify-content:flex-end;gap:8px;background:var(--t-bg-secondary);">
      <button class="t-btn" onclick="closeQuickFieldModal()">Abbrechen</button>
      <button class="t-btn t-btn-primary" onclick="submitQuickFieldModal()">Feld anlegen</button>
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
      <textarea id="csvTextarea" class="mono" rows="7" placeholder="name,domainUrl,city,employees,arrEur&#10;Elektro Weber Demo GmbH,https://elektro-weber-demo.de,Mannheim,25,12000" style="width:100%;padding:8px;border:1px solid var(--t-border-strong);border-radius:6px;background:var(--t-bg-secondary);font-size:12px;outline:none;"></textarea>
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
  starFilled: '<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z"/></svg>',
  filter: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M4 4h16v2.172a2 2 0 0 1 -.586 1.414l-4.414 4.414v7l-6 2v-8.5l-4.48 -4.928a2 2 0 0 1 -.52 -1.345v-2.227z"/></svg>',
  sort: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 9l4 -4l4 4m-4 -4v14"/><path d="M21 15l-4 4l-4 -4m4 4v-14"/></svg>',
  options: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="6" cy="10" r="2"/><path d="M6 4v4"/><path d="M6 12v8"/><circle cx="12" cy="16" r="2"/><path d="M12 4v10"/><path d="M12 18v2"/><circle cx="18" cy="7" r="2"/><path d="M18 4v1"/><path d="M18 9v11"/></svg>',
  plus: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="M5 12h14"/></svg>',
  maximize: '<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M16 4l4 0l0 4"/><path d="M14 10l6 -6"/><path d="M8 20l-4 0l0 -4"/><path d="M4 20l6 -6"/></svg>',
  minimize: '<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M15 19v-2a2 2 0 0 1 2 -2h2"/><path d="M15 5v2a2 2 0 0 0 2 2h2"/><path d="M5 15h2a2 2 0 0 1 2 2v2"/><path d="M5 9h2a2 2 0 0 0 2 -2v-2"/></svg>',
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
  tag: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M7.5 7.5m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0"/><path d="M3 6v5.172a2 2 0 0 0 .586 1.414l7.71 7.71a2.41 2.41 0 0 0 3.408 0l5.592 -5.592a2.41 2.41 0 0 0 0 -3.408l-7.71 -7.71a2 2 0 0 0 -1.414 -.586h-5.172a3 3 0 0 0 -3 3z"/></svg>',
  arrowLeft: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M5 12l6 6"/><path d="M5 12l6 -6"/></svg>',
  arrowRight: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M13 18l6 -6"/><path d="M13 6l6 6"/></svg>',
  arrowUp: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="M18 11l-6 -6"/><path d="M6 11l6 -6"/></svg>',
  arrowDown: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="M18 13l-6 6"/><path d="M6 13l6 6"/></svg>',
  eyeOff: '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M10.585 10.587a2 2 0 0 0 2.829 2.828"/><path d="M16.681 16.673a8.717 8.717 0 0 1 -4.681 1.327c-3.6 0 -6.6 -2 -9 -6c1.272 -2.12 2.712 -3.678 4.32 -4.674m2.86 -1.146a9.055 9.055 0 0 1 1.82 -.18c3.6 0 6.6 2 9 6c-.666 1.11 -1.379 2.067 -2.138 2.87"/><path d="M3 3l18 18"/></svg>',
  edit: '<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M7 7h-1a2 2 0 0 0 -2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2 -2v-1"/><path d="M20.385 6.585a2.1 2.1 0 0 0 -2.97 -2.97l-8.415 8.385v3h3l8.385 -8.415z"/><path d="M16 5l3 3"/></svg>'
};

const AVATAR_COLORS = [
  ['#dbeafe','#1d4ed8'], ['#dcfce7','#15803d'], ['#fef3c7','#b45309'],
  ['#ede9fe','#6d28d9'], ['#fce7f3','#be185d'], ['#cffafe','#0e7490']
];

const WORKSPACES = [
  { id: 'conrad-ki', name: 'Conrad KI Demo', badge: 'CK', domain: 'conrad-ki.com' },
  { id: 'handwerk', name: 'Handwerk & Mittelstand Demo', badge: 'HM', domain: 'handwerk-demo.de' },
  { id: 'all', name: 'Alle Arbeitsbereiche', badge: 'CR', domain: 'crm.conrad-ki.com' }
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

const TASK_STATUSES = ['TODO', 'IN_PROGRESS', 'DONE'];
const TASK_STATUS_LABEL = { TODO: 'Offen', IN_PROGRESS: 'In Arbeit', DONE: 'Erledigt' };
const TASK_STATUS_CLASS = { TODO: 't-tag-gray', IN_PROGRESS: 't-tag-blue', DONE: 't-tag-green' };

const FILTER_OPERATORS = [
  { id: 'contains', label: 'enthält', needsVal: true },
  { id: 'equals', label: 'ist gleich', needsVal: true },
  { id: 'not_equals', label: 'ist nicht', needsVal: true },
  { id: 'gt', label: 'größer als (>)', needsVal: true },
  { id: 'lt', label: 'kleiner als (<)', needsVal: true },
  { id: 'not_empty', label: 'ist nicht leer', needsVal: false },
  { id: 'empty', label: 'ist leer', needsVal: false }
];

const BASE_COLUMNS = {
  companies: [
    { key: 'name', label: 'Name', icon: 'building', width: 250, sticky: true, primary: true, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'domainUrl', label: 'Domain', icon: 'link', width: 195, type: 'url', editable: true, filterable: true, sortable: true },
    { key: 'createdBy', label: 'Erstellt von', icon: 'user', width: 130, type: 'actor', editable: false, filterable: true, sortable: true },
    { key: 'city', label: 'Stadt', icon: 'pin', width: 140, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'employees', label: 'Mitarbeiter', icon: 'users', width: 110, type: 'number', editable: true, filterable: true, sortable: true, agg: 'sum' },
    { key: 'arrEur', label: 'ARR', icon: 'euro', width: 115, type: 'currency', editable: true, filterable: true, sortable: true, agg: 'sum' },
    { key: 'idealCustomerProfile', label: 'ICP', icon: 'sparkles', width: 90, type: 'boolean', editable: true, filterable: true, sortable: true },
    { key: 'createdAt', label: 'Erstellt am', icon: 'calendar', width: 115, type: 'date', editable: false, filterable: true, sortable: true }
  ],
  people: [
    { key: 'fullName', label: 'Name', icon: 'user', width: 230, sticky: true, primary: true, type: 'fullname', editable: true, filterable: true, sortable: true },
    { key: 'email', label: 'E-Mail', icon: 'mail', width: 220, type: 'email', editable: true, filterable: true, sortable: true },
    { key: 'companyName', label: 'Unternehmen', icon: 'building', width: 190, type: 'relation_company', editable: false, filterable: true, sortable: true },
    { key: 'jobTitle', label: 'Position', icon: 'options', width: 175, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'phone', label: 'Telefon', icon: 'phone', width: 150, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'city', label: 'Stadt', icon: 'pin', width: 130, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'createdAt', label: 'Erstellt am', icon: 'calendar', width: 115, type: 'date', editable: false, filterable: true, sortable: true }
  ],
  opportunities: [
    { key: 'name', label: 'Name', icon: 'target', width: 245, sticky: true, primary: true, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'stage', label: 'Phase', icon: 'tag', width: 145, type: 'stage', editable: true, filterable: true, sortable: true },
    { key: 'amountEur', label: 'Betrag', icon: 'euro', width: 120, type: 'currency', editable: true, filterable: true, sortable: true, agg: 'sum' },
    { key: 'companyName', label: 'Unternehmen', icon: 'building', width: 195, type: 'relation_company', editable: false, filterable: true, sortable: true },
    { key: 'contactName', label: 'Ansprechpartner', icon: 'user', width: 170, type: 'text', editable: false, filterable: true, sortable: true },
    { key: 'closeDate', label: 'Abschlussdatum', icon: 'calendar', width: 125, type: 'date', editable: true, filterable: true, sortable: true }
  ],
  tasks: [
    { key: 'title', label: 'Titel', icon: 'checkbox', width: 280, sticky: true, primary: true, type: 'task_title', editable: true, filterable: true, sortable: true },
    { key: 'status', label: 'Status', icon: 'tag', width: 140, type: 'task_status', editable: true, filterable: true, sortable: true },
    { key: 'dueAt', label: 'Fällig am', icon: 'calendar', width: 125, type: 'date', editable: true, filterable: true, sortable: true },
    { key: 'companyName', label: 'Verknüpfungen', icon: 'building', width: 190, type: 'relation_company', editable: false, filterable: true, sortable: true },
    { key: 'bodyMarkdown', label: 'Inhalt', icon: 'notes', width: 260, type: 'text', editable: true, filterable: true, sortable: true }
  ],
  notes: [
    { key: 'title', label: 'Titel', icon: 'notes', width: 240, sticky: true, primary: true, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'bodyMarkdown', label: 'Textvorschau', icon: 'notes', width: 380, type: 'text', editable: true, filterable: true, sortable: true },
    { key: 'companyName', label: 'Verknüpfungen', icon: 'building', width: 185, type: 'relation_company', editable: false, filterable: true, sortable: true },
    { key: 'createdBy', label: 'Erstellt von', icon: 'user', width: 130, type: 'actor', editable: false, filterable: true, sortable: true },
    { key: 'createdAt', label: 'Erstellt am', icon: 'calendar', width: 115, type: 'date', editable: false, filterable: true, sortable: true }
  ]
};

const state = {
  workspace: window.location.hostname.startsWith('handwerk.') ? 'handwerk' : 'conrad-ki',
  mode: 'object',
  object: 'companies',
  viewPreset: 'all-companies',
  viewLayout: 'table',
  settingsSection: 'data-model',
  rows: [],
  rawRows: [],
  totalCount: 0,
  aggregates: null,
  limit: 50,
  offset: 0,
  q: '',
  filters: { icp: null, hasDomain: null, stage: null, status: null },
  fieldFilters: [], // [{ id, field, label, op, value }]
  sort: '',
  clientSort: null, // { field, label, dir: 'asc'|'desc' }
  colPrefs: {}, // { [obj]: { order: [], hidden: [], widths: {}, aggs: {} } }
  inlineAddOpen: false,
  selectedIds: new Set(),
  focusedCell: null,
  openRecord: null,
  recordFullscreen: false,
  rdTab: 'timeline',
  timeline: null,
  editingNoteId: null,
  stats: null,
  metadata: null,
  savedViews: [],
  favorites: [],
  webhooks: [],
  apiKeys: [],
  demoMembers: [
    { id: 'm1', name: 'Merlin Conrad', email: 'merlin@conrad-ki.com', role: 'Administrator', status: 'Aktiv' },
    { id: 'm2', name: 'Vertrieb Demo', email: 'vertrieb@conrad-ki.com', role: 'Mitglied', status: 'Aktiv' },
    { id: 'm3', name: 'Conrad KI Engine', email: 'engine@conrad-ki.com', role: 'KI-Agent (draftOnly)', status: 'Aktiv' }
  ],
  lastCreatedToken: null,
  showNewFieldForm: false,
  activeMetaObj: 'companies',
  fieldSearchQ: '',
  searchTimer: null,
  cmdIdx: 0,
  viewDirty: false
};

try {
  const savedPrefs = localStorage.getItem('conrad_crm_col_prefs_v1');
  if (savedPrefs) state.colPrefs = JSON.parse(savedPrefs);
} catch {}

function saveColPrefs() {
  try { localStorage.setItem('conrad_crm_col_prefs_v1', JSON.stringify(state.colPrefs)); } catch {}
}

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

// Mount static SVGs
(function mountIcons() {
  const m = {
    wsChev: 'chevDown', btnCollapse: 'panelLeft', expandDrawerBtn: 'panelLeft',
    icSearch: 'search', icSparkles: 'sparkles', icSettings: 'settings',
    icFav1: 'table', icFav2: 'sparkles', icFav3: 'kanban', icFav4: 'checkbox',
    icObjComp: 'building', icObjPeo: 'user', icObjOpp: 'target', icObjTask: 'checkbox',
    icObjNote: 'notes', icObjWf: 'workflow', icPlusHdr: 'plus', icAiHdr: 'sparkles',
    viewChev: 'chevDown', icSearchInline: 'search', icFilter: 'filter', icSort: 'sort',
    icOptions: 'options', rdFavBtn: 'star', rdFullBtn: 'maximize', rdDeleteBtn: 'trash', rdCloseBtn: 'x',
    icAiDrawer: 'sparkles', aiCloseBtn: 'x', icCmdSearch: 'search', favViewBtn: 'star'
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
  if (state.mode === 'settings' && state.settingsSection === 'experience') renderSettingsView();
}
function setThemeMode(mode) {
  const wantDark = mode === 'dark';
  document.documentElement.classList.toggle('dark', wantDark);
  localStorage.setItem('twenty_theme', wantDark ? 'dark' : 'light');
  updateThemeIcon();
  if (state.mode === 'settings') renderSettingsView();
}
function toggleDrawer() {
  const d = document.getElementById('tDrawer');
  const col = d.classList.toggle('collapsed');
  document.body.classList.toggle('drawer-collapsed', col);
  document.getElementById('expandDrawerBtn').style.display = col ? 'inline-flex' : 'none';
}

function toggleWsMenu(ev) {
  ev.stopPropagation();
  const pop = document.getElementById('wsPop');
  const was = pop.classList.contains('open');
  closePops();
  if (was) return;
  pop.innerHTML = `<div class="t-pop-title">Arbeitsbereich wählen</div>`
    + WORKSPACES.map(w => `
      <button class="t-pop-item" onclick="switchWorkspace('${w.id}')">
        <span style="display:inline-flex;align-items:center;gap:8px;">
          <span class="t-logo-sq" style="width:18px;height:18px;font-size:9px;">${w.badge}</span>
          <span>${esc(w.name)}</span>
        </span>
        <span>${state.workspace === w.id ? '✓' : ''}</span>
      </button>
    `).join('')
    + `<div class="t-pop-sep"></div>
       <button class="t-pop-item" onclick="closePops();openSettings('workspace')"><span>${ICONS.settings} Arbeitsbereichs-Einstellungen</span></button>`;
  pop.classList.add('open');
}
function switchWorkspace(wsId) {
  closePops();
  state.workspace = wsId;
  updateWsBadge();
  state.offset = 0;
  state.selectedIds.clear();
  loadMetadata();
  loadSavedViews();
  loadFavorites();
  loadStats();
  loadCurrentView();
}
function updateWsBadge() {
  const ws = WORKSPACES.find(w => w.id === state.workspace) || WORKSPACES[0];
  document.getElementById('wsLabel').textContent = ws.name;
  document.getElementById('wsBadge').textContent = ws.badge;
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

async function loadFavorites() {
  try {
    const r = await fetch(`/api/twenty/favorites?workspace=${encodeURIComponent(state.workspace)}`);
    if (r.ok) {
      const d = await r.json();
      state.favorites = d.favorites || [];
      renderFavoritesSidebar();
      updateFavViewStar();
    }
  } catch {}
}

function renderFavoritesSidebar() {
  const box = document.getElementById('favoritesNavList');
  if (!box) return;
  const defaults = [
    { id: 'fav-all', favKey: 'all-companies', icon: 'table', label: 'Alle Unternehmen' },
    { id: 'fav-icp', favKey: 'icp-companies', icon: 'sparkles', label: 'ICP-Unternehmen' },
    { id: 'fav-stage', favKey: 'by-stage', icon: 'kanban', label: 'Nach Phase' },
    { id: 'fav-tasks', favKey: 'my-tasks', icon: 'checkbox', label: 'Meine Aufgaben' }
  ];
  let html = defaults.map(d => `
    <button class="t-nav-item" id="${d.id}" onclick="applyFavorite('${d.favKey}')">
      <span class="t-nav-left">${ICONS[d.icon]}<span>${d.label}</span></span>
    </button>
  `).join('');
  (state.favorites || []).forEach(f => {
    const ic = f.targetType === 'view' ? 'table' : (f.targetType === 'people' ? 'user' : f.targetType === 'opportunities' ? 'target' : 'building');
    html += `<button class="t-nav-item" onclick="openFavoriteItem('${esc(f.targetType)}','${esc(f.targetId)}')">
      <span class="t-nav-left">${ICONS[ic] || ICONS.star}<span>${esc(f.label)}</span></span>
      <span class="t-fav-del" title="Favorit entfernen" onclick="event.stopPropagation();deleteFavorite('${esc(f.id)}')">×</span>
    </button>`;
  });
  box.innerHTML = html;
}

function openFavoriteItem(tType, tId) {
  if (tType === 'view') {
    const v = state.savedViews.find(x => x.id === tId);
    if (v) {
      const pluralMap = { company: 'companies', person: 'people', opportunity: 'opportunities', task: 'tasks', note: 'notes' };
      const obj = pluralMap[v.objectName] || v.objectName || 'companies';
      state.mode = 'object';
      state.object = obj;
      updateNavHighlights();
      applySavedView(v.id);
    } else {
      applyFavorite(tId);
    }
  } else if (['companies','people','opportunities','tasks','notes'].includes(tType)) {
    openRecord(tType, tId);
  }
}

async function deleteFavorite(id) {
  const r = await fetch(`/api/twenty/favorites/${encodeURIComponent(id)}`, { method: 'DELETE' });
  if (r.ok) {
    state.favorites = state.favorites.filter(x => x.id !== id);
    renderFavoritesSidebar();
    updateFavViewStar();
    toast('Favorit entfernt');
  }
}

function updateFavViewStar() {
  const btn = document.getElementById('favViewBtn');
  if (!btn) return;
  const targetId = state.viewPreset.startsWith('saved:') ? state.viewPreset.slice(6) : state.viewPreset;
  const found = (state.favorites || []).find(f => f.targetType === 'view' && f.targetId === targetId);
  btn.innerHTML = found ? ICONS.starFilled : ICONS.star;
  btn.style.color = found ? 'var(--t-amber)' : 'var(--t-text-secondary)';
}

async function togglePinCurrentView() {
  const targetId = state.viewPreset.startsWith('saved:') ? state.viewPreset.slice(6) : state.viewPreset;
  const found = (state.favorites || []).find(f => f.targetType === 'view' && f.targetId === targetId);
  if (found) {
    await deleteFavorite(found.id);
    return;
  }
  const label = document.getElementById('viewName')?.textContent || 'Ansicht';
  const r = await fetch('/api/twenty/favorites', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace: state.workspace, label, targetType: 'view', targetId })
  });
  if (r.ok) {
    const f = await r.json();
    state.favorites.push(f);
    renderFavoritesSidebar();
    updateFavViewStar();
    toast('Zu Favoriten hinzugefügt ★');
  }
}

function customFieldsForCurrentObject() {
  const singularMap = { companies: 'company', people: 'person', opportunities: 'opportunity', notes: 'note', tasks: 'task' };
  const target = singularMap[state.object] || state.object;
  const objMeta = (state.metadata?.objects || []).find(o => o.namePlural === state.object || o.nameSingular === target);
  return (objMeta?.fields || []).filter(f => f.isCustom);
}

function getAllColumnsForObject(obj) {
  const base = (BASE_COLUMNS[obj] || []).map(c => ({ ...c, isCustom: false }));
  const singularMap = { companies: 'company', people: 'person', opportunities: 'opportunity', notes: 'note', tasks: 'task' };
  const target = singularMap[obj] || obj;
  const objMeta = (state.metadata?.objects || []).find(o => o.namePlural === obj || o.nameSingular === target);
  const cfs = (objMeta?.fields || []).filter(f => f.isCustom).map(f => ({
    key: f.name,
    label: f.label,
    icon: 'tag',
    width: 145,
    type: String(f.type || 'TEXT').toLowerCase(),
    options: f.options || [],
    editable: true,
    filterable: true,
    sortable: true,
    isCustom: true,
    customMeta: f
  }));
  const all = [...base, ...cfs];
  const prefs = state.colPrefs[obj] || {};
  const widths = prefs.widths || {};
  const hidden = new Set(prefs.hidden || []);
  const order = prefs.order || [];
  all.forEach(c => {
    if (widths[c.key]) c.width = widths[c.key];
    c.hidden = hidden.has(c.key) && !c.primary;
  });
  if (order.length) {
    all.sort((a, b) => {
      if (a.primary) return -1;
      if (b.primary) return 1;
      const ia = order.indexOf(a.key);
      const ib = order.indexOf(b.key);
      if (ia === -1 && ib === -1) return 0;
      if (ia === -1) return 1;
      if (ib === -1) return -1;
      return ia - ib;
    });
  }
  return all;
}

function getVisibleColumnsForObject(obj) {
  return getAllColumnsForObject(obj).filter(c => !c.hidden);
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
  state.inlineAddOpen = false;
  state.filters = { icp: null, hasDomain: null, stage: null, status: null };
  state.fieldFilters = [];
  state.sort = '';
  state.clientSort = null;
  state.viewDirty = false;
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
    document.body.classList.add('drawer-collapsed');
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
    const sLbl = { workspace: 'Allgemein', members: 'Mitglieder', profile: 'Profil', experience: 'Erscheinungsbild', 'data-model': 'Datenmodell', apis: 'APIs & Webhooks' };
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
  updateFavViewStar();

  // View Switcher Popover
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
    + (customViews.length ? `<div class="t-pop-sep"></div><div class="t-pop-title">Gespeicherte Ansichten</div>` + customViews.map(v => `
      <div class="t-pop-item" style="justify-content:space-between;" onclick="applySavedView('${esc(v.id)}')">
        <span style="display:inline-flex;align-items:center;gap:6px;">${v.layout === 'kanban' ? ICONS.kanban : ICONS.table} ${esc(v.name)}</span>
        <span style="display:inline-flex;align-items:center;gap:6px;">
          ${state.viewPreset === 'saved:' + v.id ? '<span>✓</span>' : ''}
          <span onclick="event.stopPropagation();deleteSavedView('${esc(v.id)}')" title="Ansicht löschen" style="color:var(--t-text-tertiary);padding:0 3px;">×</span>
        </span>
      </div>
    `).join('') : '')
    + `<div class="t-pop-sep"></div>
      <button class="t-pop-item" onclick="openSaveViewModal();closePops()"><span>+ Aktuelle Ansicht speichern</span></button>`;

  // Filter Popover (Quick presets + Field/Operator Filter Builder)
  const cols = getAllColumnsForObject(state.object).filter(c => c.filterable);
  let fHtml = '';
  if (state.object === 'companies') {
    fHtml += `<button class="t-pop-item" onclick="toggleFilter('icp', true)"><span>ICP ist Ja</span><span>${state.filters.icp ? '✓' : ''}</span></button>`;
    fHtml += `<button class="t-pop-item" onclick="toggleFilter('hasDomain', true)"><span>Domain nicht leer</span><span>${state.filters.hasDomain ? '✓' : ''}</span></button>`;
  } else if (state.object === 'opportunities') {
    STAGES.forEach(s => {
      fHtml += `<button class="t-pop-item" onclick="toggleFilter('stage', '${s}')"><span>Phase: ${STAGE_LABEL[s] || s}</span><span>${state.filters.stage === s ? '✓' : ''}</span></button>`;
    });
  } else if (state.object === 'tasks') {
    TASK_STATUSES.forEach(s => {
      fHtml += `<button class="t-pop-item" onclick="toggleFilter('status', '${s}')"><span>Status: ${TASK_STATUS_LABEL[s] || s}</span><span>${state.filters.status === s ? '✓' : ''}</span></button>`;
    });
  }
  fHtml += `<div class="t-pop-sep"></div>
    <div class="t-pop-title">Filter hinzufügen (Feld &amp; Operator)</div>
    <div style="padding:4px 6px;display:flex;flex-direction:column;gap:6px;" onclick="event.stopPropagation()">
      <select id="fbField" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);">
        ${cols.map(c => `<option value="${esc(c.key)}">${esc(c.label)}</option>`).join('')}
      </select>
      <select id="fbOp" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);" onchange="document.getElementById('fbVal').style.display=(this.value==='empty'||this.value==='not_empty')?'none':'block'">
        ${FILTER_OPERATORS.map(op => `<option value="${op.id}">${esc(op.label)}</option>`).join('')}
      </select>
      <input id="fbVal" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);" placeholder="Filterwert..." onkeydown="if(event.key==='Enter')applyFieldFilterFromPop()">
      <div style="display:flex;gap:6px;justify-content:space-between;">
        <button class="t-btn" style="flex:1;justify-content:center;" onclick="clearAllFilters()">Zurücksetzen</button>
        <button class="t-btn t-btn-blue" style="flex:1;justify-content:center;" onclick="applyFieldFilterFromPop()">Anwenden</button>
      </div>
    </div>`;
  document.getElementById('filterPopItems').innerHTML = fHtml;

  // Sort Popover
  const serverSorts = state.object === 'companies'
    ? [['', 'Standard (Aktualisiert ↓)'], ['name_asc', 'Name (A → Z)'], ['employees_desc', 'Mitarbeiter (Absteigend)'], ['arr_desc', 'ARR (Absteigend)']]
    : [['', 'Standard (Aktualisiert ↓)']];
  const colSorts = cols.map(c => ({ key: c.key, label: c.label }));
  document.getElementById('sortPopItems').innerHTML =
    serverSorts.map(([val, lbl]) => `
      <button class="t-pop-item" onclick="setSort('${val}')"><span>${esc(lbl)}</span><span>${state.sort === val && !state.clientSort ? '✓' : ''}</span></button>
    `).join('')
    + `<div class="t-pop-sep"></div><div class="t-pop-title">Nach Spalte sortieren</div>`
    + colSorts.slice(0, 6).map(cs => `
      <div style="display:flex;align-items:center;justify-content:space-between;padding:2px 6px;gap:4px;">
        <span style="font-size:12px;color:var(--t-text-secondary);">${esc(cs.label)}</span>
        <span style="display:inline-flex;gap:3px;">
          <button class="t-btn" style="height:22px;padding:0 6px;font-size:11px;" onclick="setClientSort('${esc(cs.key)}','${esc(cs.label)}','asc')">↑</button>
          <button class="t-btn" style="height:22px;padding:0 6px;font-size:11px;" onclick="setClientSort('${esc(cs.key)}','${esc(cs.label)}','desc')">↓</button>
        </span>
      </div>
    `).join('');

  // Active chips bar (blue filter/sort chips + Save View button)
  const chips = [];
  if (state.filters.icp) chips.push(`<span class="t-chip-filter">${ICONS.filter} <span>ICP</span> <strong>ist Ja</strong> <button onclick="toggleFilter('icp', true)" title="Entfernen">×</button></span>`);
  if (state.filters.hasDomain) chips.push(`<span class="t-chip-filter">${ICONS.link} <span>Domain</span> <strong>nicht leer</strong> <button onclick="toggleFilter('hasDomain', true)" title="Entfernen">×</button></span>`);
  if (state.filters.stage) chips.push(`<span class="t-chip-filter">${ICONS.tag} <span>Phase</span> <strong>${esc(STAGE_LABEL[state.filters.stage] || state.filters.stage)}</strong> <button onclick="toggleFilter('stage', '${esc(state.filters.stage)}')" title="Entfernen">×</button></span>`);
  if (state.filters.status) chips.push(`<span class="t-chip-filter">${ICONS.checkbox} <span>Status</span> <strong>${esc(TASK_STATUS_LABEL[state.filters.status] || state.filters.status)}</strong> <button onclick="toggleFilter('status', '${esc(state.filters.status)}')" title="Entfernen">×</button></span>`);
  (state.fieldFilters || []).forEach(ff => {
    const opLbl = (FILTER_OPERATORS.find(o => o.id === ff.op) || {}).label || ff.op;
    chips.push(`<span class="t-chip-filter">${ICONS.filter} <span>${esc(ff.label)} ${esc(opLbl)}</span> ${ff.value ? `<strong>${esc(ff.value)}</strong>` : ''} <button onclick="removeFieldFilter('${esc(ff.id)}')" title="Entfernen">×</button></span>`);
  });
  if (state.sort) chips.push(`<span class="t-chip-filter">${ICONS.sort} <span>Sortierung:</span> <strong>${esc(state.sort)}</strong> <button onclick="setSort('')" title="Entfernen">×</button></span>`);
  if (state.clientSort) chips.push(`<span class="t-chip-filter">${ICONS.sort} <span>Sortierung:</span> <strong>${esc(state.clientSort.label)} (${state.clientSort.dir === 'asc' ? '↑' : '↓'})</strong> <button onclick="clearClientSort()" title="Entfernen">×</button></span>`);
  if (state.q) chips.push(`<span class="t-chip-filter">${ICONS.search} <span>Enthält</span> <strong>${esc(state.q)}</strong> <button onclick="clearSearch()" title="Entfernen">×</button></span>`);

  const cb = document.getElementById('chipsBar');
  const hasAny = chips.length > 0 || state.viewDirty;
  if (hasAny) {
    const saveBtn = customViewObj
      ? `<button class="t-btn t-btn-blue" style="height:24px;font-size:11.5px;" onclick="updateActiveSavedView()">Ansicht aktualisieren</button>`
      : `<button class="t-btn t-btn-blue" style="height:24px;font-size:11.5px;" onclick="openSaveViewModal()">Ansicht speichern</button>`;
    cb.innerHTML = `<div class="t-chips-list">${chips.join('')}</div><div style="display:flex;gap:6px;align-items:center;">${saveBtn}</div>`;
    cb.classList.add('visible');
  } else {
    cb.innerHTML = '';
    cb.classList.remove('visible');
  }
}

function togglePop(id) {
  const target = document.getElementById(id);
  const wasOpen = target.classList.contains('open');
  closePops();
  if (!wasOpen) target.classList.add('open');
}
function closePops() {
  document.querySelectorAll('.t-popover').forEach(p => p.classList.remove('open'));
  const fm = document.getElementById('floatMenu');
  if (fm) fm.classList.remove('open');
}
document.addEventListener('click', e => {
  if (!e.target.closest('.t-vb-left,.t-vb-right,.t-ws-top,.t-float-menu,.t-th-menu-btn,th[data-col-key]')) closePops();
});

function selectViewPreset(presetId, layout) {
  closePops();
  state.viewPreset = presetId;
  state.viewLayout = layout;
  state.filters = { icp: null, hasDomain: null, stage: null, status: null };
  state.fieldFilters = [];
  state.sort = '';
  state.clientSort = null;
  state.viewDirty = false;
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

function openSaveViewModal() {
  const inp = document.getElementById('saveViewNameInput');
  inp.value = state.q ? `Suche: ${state.q}` : 'Meine Ansicht';
  document.getElementById('saveViewBackdrop').classList.add('open');
  setTimeout(() => { inp.focus(); inp.select(); }, 30);
}
function closeSaveViewModal() { document.getElementById('saveViewBackdrop').classList.remove('open'); }
async function submitSaveViewModal() {
  const name = document.getElementById('saveViewNameInput')?.value?.trim();
  if (!name) { toast('Bitte Namen angeben'); return; }
  const payload = {
    workspace: state.workspace,
    objectName: state.object,
    name,
    layout: state.viewLayout,
    filters: { ...state.filters, q: state.q || null, fieldFilters: state.fieldFilters },
    sort: { key: state.sort || '', clientSort: state.clientSort },
    columns: getVisibleColumnsForObject(state.object).map(c => c.key)
  };
  const r = await fetch('/api/twenty/views', {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload)
  });
  if (r.ok) {
    const saved = await r.json();
    state.savedViews.push(saved);
    state.viewPreset = 'saved:' + saved.id;
    state.viewDirty = false;
    closeSaveViewModal();
    renderHeaderAndViewBar();
    toast('Ansicht gespeichert ✓');
  } else {
    toast('Ansicht konnte nicht gespeichert werden');
  }
}

async function updateActiveSavedView() {
  if (!state.viewPreset.startsWith('saved:')) return openSaveViewModal();
  const id = state.viewPreset.slice(6);
  const r = await fetch(`/api/twenty/views/${encodeURIComponent(id)}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      layout: state.viewLayout,
      filters: { ...state.filters, q: state.q || null, fieldFilters: state.fieldFilters },
      sort: { key: state.sort || '', clientSort: state.clientSort },
      columns: getVisibleColumnsForObject(state.object).map(c => c.key)
    })
  });
  if (r.ok) {
    const updated = await r.json();
    const idx = state.savedViews.findIndex(v => v.id === id);
    if (idx >= 0) state.savedViews[idx] = updated;
    state.viewDirty = false;
    renderHeaderAndViewBar();
    toast('Ansicht aktualisiert ✓');
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
  state.fieldFilters = Array.isArray(f.fieldFilters) ? f.fieldFilters : [];
  state.q = f.q || '';
  state.sort = (v.sort && v.sort.key) || '';
  state.clientSort = (v.sort && v.sort.clientSort) || null;
  state.viewDirty = false;
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
  state.viewDirty = true;
  state.offset = 0;
  renderHeaderAndViewBar();
  loadCurrentView();
}
function applyFieldFilterFromPop() {
  const fieldKey = document.getElementById('fbField')?.value;
  const op = document.getElementById('fbOp')?.value || 'contains';
  const val = document.getElementById('fbVal')?.value?.trim() || '';
  if (!fieldKey) return;
  if (op !== 'empty' && op !== 'not_empty' && !val) { toast('Bitte Filterwert eingeben'); return; }
  const col = getAllColumnsForObject(state.object).find(c => c.key === fieldKey);
  state.fieldFilters.push({ id: 'ff_' + Date.now(), field: fieldKey, label: col ? col.label : fieldKey, op, value: val });
  state.viewDirty = true;
  state.offset = 0;
  closePops();
  renderHeaderAndViewBar();
  loadCurrentView();
}
function removeFieldFilter(id) {
  state.fieldFilters = state.fieldFilters.filter(x => x.id !== id);
  state.viewDirty = true;
  renderHeaderAndViewBar();
  loadCurrentView();
}
function clearAllFilters() {
  closePops();
  state.filters = { icp: null, hasDomain: null, stage: null, status: null };
  state.fieldFilters = [];
  state.sort = '';
  state.clientSort = null;
  state.q = '';
  state.viewDirty = false;
  document.getElementById('inlineSearch').value = '';
  state.offset = 0;
  renderHeaderAndViewBar();
  loadCurrentView();
}
function clearSearch() { state.q = ''; document.getElementById('inlineSearch').value = ''; state.offset = 0; renderHeaderAndViewBar(); loadCurrentView(); }
function setSort(val) { closePops(); state.sort = val; state.clientSort = null; state.viewDirty = !!val; state.offset = 0; renderHeaderAndViewBar(); loadCurrentView(); }
function setClientSort(field, label, dir) {
  closePops();
  state.sort = '';
  state.clientSort = { field, label, dir };
  state.viewDirty = true;
  renderHeaderAndViewBar();
  applyClientFiltersAndSort();
  if (state.viewLayout === 'kanban') renderKanbanView(); else renderTableView();
}
function clearClientSort() {
  state.clientSort = null;
  state.viewDirty = state.fieldFilters.length > 0;
  renderHeaderAndViewBar();
  applyClientFiltersAndSort();
  if (state.viewLayout === 'kanban') renderKanbanView(); else renderTableView();
}
function onSearchInput() {
  clearTimeout(state.searchTimer);
  state.searchTimer = setTimeout(() => {
    state.q = document.getElementById('inlineSearch').value.trim();
    state.offset = 0;
    renderHeaderAndViewBar();
    loadCurrentView();
  }, 180);
}

function getRowFieldRaw(row, obj, key) {
  if (key === 'name') return row.name || '';
  if (key === 'domainUrl') return cleanDomain(row.domainName?.primaryLinkUrl || '');
  if (key === 'createdBy') return row.createdBy?.name || '';
  if (key === 'city') return obj === 'companies' ? (row.address?.addressCity || '') : (row.city || '');
  if (key === 'employees') return row.employees ?? '';
  if (key === 'arrEur') return row.annualRecurringRevenue?.amountEur || ((row.annualRecurringRevenue?.amountMicros || 0) / 1000000) || 0;
  if (key === 'idealCustomerProfile') return row.idealCustomerProfile ? 'Ja' : 'Nein';
  if (key === 'createdAt') return fmtDate(row.createdAt);
  if (key === 'fullName') return row.fullName || '';
  if (key === 'email') return row.emails?.primaryEmail || '';
  if (key === 'phone') return row.phones?.primaryPhoneNumber || '';
  if (key === 'companyName') return row.company?.name || row.companyName || '';
  if (key === 'jobTitle') return row.jobTitle || '';
  if (key === 'stage') return STAGE_LABEL[row.stage] || row.stage || '';
  if (key === 'amountEur') return row.amount?.amountEur || ((row.amount?.amountMicros || 0) / 1000000) || 0;
  if (key === 'contactName') return row.pointOfContact?.fullName || '';
  if (key === 'closeDate') return row.closeDate ? fmtDate(row.closeDate) : '';
  if (key === 'title') return row.title || '';
  if (key === 'status') return TASK_STATUS_LABEL[row.status] || row.status || '';
  if (key === 'dueAt') return row.dueAt ? fmtDate(row.dueAt) : '';
  if (key === 'bodyMarkdown') return row.bodyMarkdown || '';
  if (row.customFields && row.customFields[key] !== undefined) return row.customFields[key];
  return row[key] ?? '';
}

function applyClientFiltersAndSort() {
  let list = [...(state.rawRows || [])];
  if (state.fieldFilters.length) {
    list = list.filter(row => {
      return state.fieldFilters.every(ff => {
        const raw = getRowFieldRaw(row, state.object, ff.field);
        const str = String(raw ?? '').trim().toLowerCase();
        const tgt = String(ff.value ?? '').trim().toLowerCase();
        if (ff.op === 'not_empty') return str !== '' && str !== '—';
        if (ff.op === 'empty') return str === '' || str === '—';
        if (ff.op === 'contains') return str.includes(tgt);
        if (ff.op === 'equals') return str === tgt;
        if (ff.op === 'not_equals') return str !== tgt;
        if (ff.op === 'gt') return Number(raw || 0) > Number(ff.value || 0);
        if (ff.op === 'lt') return Number(raw || 0) < Number(ff.value || 0);
        return true;
      });
    });
  }
  if (state.clientSort) {
    const { field, dir } = state.clientSort;
    const mul = dir === 'asc' ? 1 : -1;
    list.sort((a, b) => {
      const va = getRowFieldRaw(a, state.object, field);
      const vb = getRowFieldRaw(b, state.object, field);
      if (typeof va === 'number' && typeof vb === 'number') return (va - vb) * mul;
      return String(va ?? '').localeCompare(String(vb ?? ''), 'de', { numeric: true }) * mul;
    });
  }
  state.rows = list;
  if (state.fieldFilters.length) state.totalCount = list.length;
}

async function loadCurrentView() {
  if (state.mode === 'settings') { renderSettingsView(); return; }
  const useWide = state.viewLayout === 'kanban' || state.fieldFilters.length > 0 || !!state.clientSort;
  const p = new URLSearchParams({ workspace: state.workspace, limit: String(useWide ? 200 : state.limit), offset: String(useWide ? 0 : state.offset) });
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
    state.rawRows = d.data || [];
    state.totalCount = d.totalCount ?? state.rawRows.length;
    state.aggregates = d.aggregates || null;
    applyClientFiltersAndSort();
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

// Column Header Menu, Move, Hide, Resize, + Column
function openColumnMenu(ev, colKey) {
  ev.stopPropagation();
  const cols = getAllColumnsForObject(state.object);
  const vis = cols.filter(c => !c.hidden);
  const col = cols.find(c => c.key === colKey);
  if (!col) return;
  const visIdx = vis.findIndex(c => c.key === colKey);
  const canMove = !col.primary;
  const canLeft = canMove && visIdx > 1;
  const canRight = canMove && visIdx < vis.length - 1;
  const fm = document.getElementById('floatMenu');
  const rect = ev.currentTarget.getBoundingClientRect();
  fm.style.left = Math.min(rect.left, window.innerWidth - 230) + 'px';
  fm.style.top = (rect.bottom + 4) + 'px';
  fm.innerHTML = `
    <div class="t-pop-title">${esc(col.label)}</div>
    <button class="t-pop-item" onclick="setClientSort('${esc(col.key)}','${esc(col.label)}','asc')"><span>${ICONS.arrowUp} Aufsteigend sortieren</span><span>${state.clientSort?.field===col.key && state.clientSort.dir==='asc'?'✓':''}</span></button>
    <button class="t-pop-item" onclick="setClientSort('${esc(col.key)}','${esc(col.label)}','desc')"><span>${ICONS.arrowDown} Absteigend sortieren</span><span>${state.clientSort?.field===col.key && state.clientSort.dir==='desc'?'✓':''}</span></button>
    <button class="t-pop-item" onclick="openFilterForCol('${esc(col.key)}')"><span>${ICONS.filter} Nach Feld filtern</span></button>
    ${canMove ? `<div class="t-pop-sep"></div>` : ''}
    ${canLeft ? `<button class="t-pop-item" onclick="moveColumn('${esc(col.key)}',-1)"><span>${ICONS.arrowLeft} Nach links verschieben</span></button>` : ''}
    ${canRight ? `<button class="t-pop-item" onclick="moveColumn('${esc(col.key)}',1)"><span>${ICONS.arrowRight} Nach rechts verschieben</span></button>` : ''}
    ${!col.primary ? `<button class="t-pop-item" onclick="toggleColumnVisibility('${esc(col.key)}',true)"><span>${ICONS.eyeOff} Spalte ausblenden</span></button>` : ''}
  `;
  fm.classList.add('open');
}

function openFilterForCol(colKey) {
  closePops();
  togglePop('filterPop');
  const sel = document.getElementById('fbField');
  if (sel) sel.value = colKey;
  setTimeout(() => document.getElementById('fbVal')?.focus(), 20);
}

function moveColumn(colKey, dir) {
  const vis = getVisibleColumnsForObject(state.object).map(c => c.key);
  const idx = vis.indexOf(colKey);
  const target = idx + dir;
  if (idx <= 0 || target <= 0 || target >= vis.length) return;
  [vis[idx], vis[target]] = [vis[target], vis[idx]];
  const allKeys = getAllColumnsForObject(state.object).map(c => c.key).filter(k => !vis.includes(k));
  state.colPrefs[state.object] = state.colPrefs[state.object] || {};
  state.colPrefs[state.object].order = [...vis, ...allKeys];
  state.viewDirty = true;
  saveColPrefs();
  closePops();
  renderHeaderAndViewBar();
  renderTableView();
}

function toggleColumnVisibility(colKey, hide) {
  state.colPrefs[state.object] = state.colPrefs[state.object] || {};
  const set = new Set(state.colPrefs[state.object].hidden || []);
  if (hide) set.add(colKey); else set.delete(colKey);
  state.colPrefs[state.object].hidden = Array.from(set);
  state.viewDirty = true;
  saveColPrefs();
  closePops();
  renderHeaderAndViewBar();
  renderTableView();
}

function openAddColumnMenu(ev) {
  ev.stopPropagation();
  const cols = getAllColumnsForObject(state.object);
  const hiddenCols = cols.filter(c => c.hidden);
  const fm = document.getElementById('floatMenu');
  const rect = ev.currentTarget.getBoundingClientRect();
  fm.style.left = Math.max(8, Math.min(rect.right - 230, window.innerWidth - 240)) + 'px';
  fm.style.top = (rect.bottom + 4) + 'px';
  fm.innerHTML = `
    ${hiddenCols.length ? `<div class="t-pop-title">Ausgeblendete Spalten</div>` + hiddenCols.map(c => `
      <button class="t-pop-item" onclick="toggleColumnVisibility('${esc(c.key)}',false)"><span>${ICONS[c.icon] || ICONS.tag} ${esc(c.label)}</span><span>+ Einblenden</span></button>
    `).join('') + `<div class="t-pop-sep"></div>` : ''}
    <button class="t-pop-item" onclick="closePops();openQuickFieldModal()"><span>${ICONS.plus} Neues Feld erstellen</span></button>
    <button class="t-pop-item" onclick="closePops();openSettings('data-model')"><span>${ICONS.settings} Datenmodell verwalten</span></button>
  `;
  fm.classList.add('open');
}

function openFieldsManagerMenu(ev) {
  ev.stopPropagation();
  const cols = getAllColumnsForObject(state.object).filter(c => !c.primary);
  const fm = document.getElementById('floatMenu');
  const rect = ev.currentTarget.getBoundingClientRect();
  fm.style.left = Math.max(8, Math.min(rect.left - 220, window.innerWidth - 240)) + 'px';
  fm.style.top = rect.top + 'px';
  fm.innerHTML = `<div class="t-pop-title">Sichtbare Felder</div>` + cols.map(c => `
    <button class="t-pop-item" onclick="event.stopPropagation();toggleColumnVisibility('${esc(c.key)}',${!c.hidden});openFieldsManagerMenu(event)">
      <span>${ICONS[c.icon] || ICONS.tag} ${esc(c.label)}</span>
      <span>${!c.hidden ? '✓' : ''}</span>
    </button>
  `).join('');
  fm.classList.add('open');
}

function openQuickFieldModal() {
  document.getElementById('qfLabel').value = '';
  document.getElementById('qfType').value = 'TEXT';
  document.getElementById('qfOptions').value = '';
  document.getElementById('qfDefault').value = '';
  document.getElementById('quickFieldBackdrop').classList.add('open');
  setTimeout(() => document.getElementById('qfLabel').focus(), 30);
}
function closeQuickFieldModal() { document.getElementById('quickFieldBackdrop').classList.remove('open'); }
async function submitQuickFieldModal() {
  const label = document.getElementById('qfLabel')?.value?.trim();
  const fieldType = document.getElementById('qfType')?.value || 'TEXT';
  const optsRaw = document.getElementById('qfOptions')?.value?.trim() || '';
  const defaultValue = document.getElementById('qfDefault')?.value?.trim() || '';
  if (!label) { toast('Bitte Feldbezeichnung angeben'); return; }
  const singularMap = { companies: 'company', people: 'person', opportunities: 'opportunity', notes: 'note', tasks: 'task' };
  const objectName = singularMap[state.object] || 'company';
  const options = optsRaw ? optsRaw.split(',').map(s => s.trim()).filter(Boolean) : [];
  const r = await fetch('/api/twenty/metadata/fields', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace: 'all', objectName, label, type: fieldType, options, defaultValue })
  });
  if (r.ok) {
    closeQuickFieldModal();
    toast('Neues Feld angelegt ✓');
    await loadMetadata();
    renderTableView();
  } else {
    const err = await r.json().catch(() => ({}));
    toast(err.error || 'Feld konnte nicht angelegt werden');
  }
}

function startColResize(ev, colKey) {
  ev.stopPropagation();
  ev.preventDefault();
  const handle = ev.currentTarget;
  const th = handle.closest('th');
  if (!th) return;
  const startX = ev.clientX;
  const startW = th.getBoundingClientRect().width;
  handle.classList.add('resizing');
  const onMove = (me) => {
    const nextW = Math.max(80, Math.min(600, Math.round(startW + (me.clientX - startX))));
    th.style.width = nextW + 'px';
  };
  const onUp = (ue) => {
    handle.classList.remove('resizing');
    document.removeEventListener('mousemove', onMove);
    document.removeEventListener('mouseup', onUp);
    const finalW = Math.max(80, Math.min(600, Math.round(startW + (ue.clientX - startX))));
    state.colPrefs[state.object] = state.colPrefs[state.object] || {};
    state.colPrefs[state.object].widths = state.colPrefs[state.object].widths || {};
    state.colPrefs[state.object].widths[colKey] = finalW;
    saveColPrefs();
  };
  document.addEventListener('mousemove', onMove);
  document.addEventListener('mouseup', onUp);
}

// Cell rendering & Footer Aggregates
function renderCellContent(row, col) {
  const obj = state.object;
  if (col.isCustom) {
    const raw = (row.customFields && row.customFields[col.key] !== undefined) ? row.customFields[col.key] : (row[col.key] ?? '');
    const ft = String(col.type || 'text').toUpperCase();
    if (ft === 'BOOLEAN') {
      return raw === true || raw === 'true' ? '<span class="t-tag t-tag-green">Ja</span>' : (raw === false || raw === 'false' ? '<span class="t-tag t-tag-gray">Nein</span>' : '<span style="color:var(--t-text-tertiary);">—</span>');
    }
    if (raw === '' || raw === null || raw === undefined) return '<span style="color:var(--t-text-tertiary);">—</span>';
    if (ft === 'CURRENCY') return `<span class="t-num">${fmtEur(raw)}</span>`;
    if (ft === 'NUMBER') return `<span class="t-num">${fmtNum(raw)}</span>`;
    if (ft === 'SELECT') return `<span class="t-tag t-tag-purple">${esc(raw)}</span>`;
    if (ft === 'URL') return `<span class="t-link-pill">${esc(raw)}</span>`;
    return esc(String(raw));
  }

  if (obj === 'companies') {
    if (col.key === 'name') {
      const dom = row.domainName?.primaryLinkUrl || '';
      return `<div class="t-name-cell">
        <div class="t-name-main">${renderFavicon(dom, row.name)}<span class="t-name-text" onclick="event.stopPropagation();openRecord('companies','${esc(row.id)}')">${esc(row.name)}</span></div>
        <button class="t-open-hover" onclick="event.stopPropagation();openRecord('companies','${esc(row.id)}')" title="Datensatz öffnen">${ICONS.maximize}</button>
      </div>`;
    }
    if (col.key === 'domainUrl') {
      const dom = row.domainName?.primaryLinkUrl || '';
      const domLabel = cleanDomain(dom);
      if (!dom) return '<span style="color:var(--t-text-tertiary);">—</span>';
      return `<span class="t-link-wrap"><span class="t-link-pill">${esc(domLabel)}</span><a class="t-link-ext" href="${esc(dom.startsWith('http') ? dom : 'https://' + dom)}" target="_blank" rel="noopener" onclick="event.stopPropagation()" title="Website öffnen">↗</a></span>`;
    }
    if (col.key === 'createdBy') {
      const nm = row.createdBy?.name || 'Demo-Team';
      const [cBg, cFg] = colorFor(nm);
      return `<span class="t-tag t-tag-gray"><span class="t-avatar-sm" style="width:14px;height:14px;font-size:8px;background:${cBg};color:${cFg};">${esc(initials(nm))}</span>${esc(nm)}</span>`;
    }
    if (col.key === 'city') return esc(row.address?.addressCity || '') || '<span style="color:var(--t-text-tertiary);">—</span>';
    if (col.key === 'employees') return `<span class="t-num">${fmtNum(row.employees)}</span>`;
    if (col.key === 'arrEur') {
      const arr = row.annualRecurringRevenue?.amountEur || ((row.annualRecurringRevenue?.amountMicros || 0) / 1000000);
      return `<span class="t-num">${fmtEur(arr)}</span>`;
    }
    if (col.key === 'idealCustomerProfile') {
      return row.idealCustomerProfile ? '<span class="t-tag t-tag-green">Ja</span>' : '<span class="t-tag t-tag-gray">Nein</span>';
    }
    if (col.key === 'createdAt') return `<span style="color:var(--t-text-secondary);">${fmtDate(row.createdAt)}</span>`;
  }

  if (obj === 'people') {
    if (col.key === 'fullName') {
      const [bg, fg] = colorFor(row.fullName);
      return `<div class="t-name-cell">
        <div class="t-name-main"><span class="t-avatar-sm" style="background:${bg};color:${fg};">${esc(initials(row.fullName))}</span><span class="t-name-text" onclick="event.stopPropagation();openRecord('people','${esc(row.id)}')">${esc(row.fullName || 'Ohne Name')}</span></div>
        <button class="t-open-hover" onclick="event.stopPropagation();openRecord('people','${esc(row.id)}')">${ICONS.maximize}</button>
      </div>`;
    }
    if (col.key === 'email') {
      const em = row.emails?.primaryEmail || '';
      return em ? `<span class="t-link-pill">${esc(em)}</span>` : '<span style="color:var(--t-text-tertiary);">—</span>';
    }
    if (col.key === 'companyName') {
      return row.company?.name ? `<span class="t-tag t-tag-gray" style="cursor:pointer;" onclick="event.stopPropagation();openRecord('companies','${esc(row.company.id)}')">${esc(row.company.name)}</span>` : '<span style="color:var(--t-text-tertiary);">—</span>';
    }
    if (col.key === 'jobTitle') return esc(row.jobTitle) || '<span style="color:var(--t-text-tertiary);">—</span>';
    if (col.key === 'phone') return esc(row.phones?.primaryPhoneNumber || '') || '<span style="color:var(--t-text-tertiary);">—</span>';
    if (col.key === 'city') return esc(row.city) || '<span style="color:var(--t-text-tertiary);">—</span>';
    if (col.key === 'createdAt') return `<span style="color:var(--t-text-secondary);">${fmtDate(row.createdAt)}</span>`;
  }

  if (obj === 'opportunities') {
    if (col.key === 'name') {
      return `<div class="t-name-cell">
        <span class="t-name-text" onclick="event.stopPropagation();openRecord('opportunities','${esc(row.id)}')">${esc(row.name)}</span>
        <button class="t-open-hover" onclick="event.stopPropagation();openRecord('opportunities','${esc(row.id)}')">${ICONS.maximize}</button>
      </div>`;
    }
    if (col.key === 'stage') return `<span class="t-tag ${STAGE_CLASS[row.stage] || 't-tag-gray'}">${esc(STAGE_LABEL[row.stage] || row.stage)}</span>`;
    if (col.key === 'amountEur') {
      const amt = row.amount?.amountEur || ((row.amount?.amountMicros || 0) / 1000000);
      return `<span class="t-num">${fmtEur(amt)}</span>`;
    }
    if (col.key === 'companyName') return row.company?.name ? `<span class="t-tag t-tag-gray" style="cursor:pointer;" onclick="event.stopPropagation();openRecord('companies','${esc(row.company.id)}')">${esc(row.company.name)}</span>` : '—';
    if (col.key === 'contactName') return esc(row.pointOfContact?.fullName || '—');
    if (col.key === 'closeDate') return fmtDate(row.closeDate);
  }

  if (obj === 'tasks') {
    if (col.key === 'title') {
      const done = row.status === 'DONE';
      return `<div class="t-name-cell">
        <div class="t-name-main">
          <button class="t-icon-btn" style="width:18px;height:18px;border:1px solid var(--t-border-strong);border-radius:99px;font-size:10px;flex-shrink:0;${done?'background:var(--t-green);color:#fff;border-color:var(--t-green);':''}" onclick="event.stopPropagation();patchRecord('tasks','${esc(row.id)}',{status:'${done?'TODO':'DONE'}'})" title="Als erledigt markieren">${done ? '✓' : ''}</button>
          <span class="t-name-text" style="${done ? 'text-decoration:line-through;color:var(--t-text-tertiary);' : ''}" onclick="event.stopPropagation();openRecord('tasks','${esc(row.id)}')">${esc(row.title)}</span>
        </div>
        <button class="t-open-hover" onclick="event.stopPropagation();openRecord('tasks','${esc(row.id)}')">${ICONS.maximize}</button>
      </div>`;
    }
    if (col.key === 'status') return `<span class="t-tag ${TASK_STATUS_CLASS[row.status] || 't-tag-gray'}">${esc(TASK_STATUS_LABEL[row.status] || row.status)}</span>`;
    if (col.key === 'dueAt') return fmtDate(row.dueAt);
    if (col.key === 'companyName') return row.companyName ? `<span class="t-tag t-tag-gray">${esc(row.companyName)}</span>` : '—';
    if (col.key === 'bodyMarkdown') return `<span style="color:var(--t-text-secondary);">${esc((row.bodyMarkdown || '').slice(0, 70)) || '—'}</span>`;
  }

  if (obj === 'notes') {
    if (col.key === 'title') {
      return `<div class="t-name-cell">
        <span class="t-name-text" onclick="event.stopPropagation();openRecord('notes','${esc(row.id)}')">${esc(row.title || 'Notiz')}</span>
        <button class="t-open-hover" onclick="event.stopPropagation();openRecord('notes','${esc(row.id)}')">${ICONS.maximize}</button>
      </div>`;
    }
    if (col.key === 'bodyMarkdown') return `<span style="color:var(--t-text-secondary);">${esc((row.bodyMarkdown || '').replace(/\s+/g,' ').slice(0, 95)) || '—'}</span>`;
    if (col.key === 'companyName') return row.companyName ? `<span class="t-tag t-tag-gray">${esc(row.companyName)}</span>` : '—';
    if (col.key === 'createdBy') return `<span class="t-tag t-tag-gray">${esc(row.createdBy?.name || 'Demo-Team')}</span>`;
    if (col.key === 'createdAt') return `<span style="color:var(--t-text-secondary);">${fmtDate(row.createdAt)}</span>`;
  }
  return '—';
}

function getRawEditValue(row, col) {
  const obj = state.object;
  if (col.isCustom) {
    const raw = (row.customFields && row.customFields[col.key] !== undefined) ? row.customFields[col.key] : (row[col.key] ?? '');
    return raw === null || raw === undefined ? '' : String(raw);
  }
  if (obj === 'companies') {
    if (col.key === 'name') return row.name || '';
    if (col.key === 'domainUrl') return row.domainName?.primaryLinkUrl || '';
    if (col.key === 'city') return row.address?.addressCity || '';
    if (col.key === 'employees') return row.employees ?? '';
    if (col.key === 'arrEur') {
      const a = row.annualRecurringRevenue?.amountEur || ((row.annualRecurringRevenue?.amountMicros || 0) / 1000000);
      return a ? String(Math.round(a)) : '';
    }
    if (col.key === 'idealCustomerProfile') return row.idealCustomerProfile ? 'true' : 'false';
  }
  if (obj === 'people') {
    if (col.key === 'fullName') return row.fullName || '';
    if (col.key === 'email') return row.emails?.primaryEmail || '';
    if (col.key === 'jobTitle') return row.jobTitle || '';
    if (col.key === 'phone') return row.phones?.primaryPhoneNumber || '';
    if (col.key === 'city') return row.city || '';
  }
  if (obj === 'opportunities') {
    if (col.key === 'name') return row.name || '';
    if (col.key === 'stage') return row.stage || 'NEW';
    if (col.key === 'amountEur') {
      const a = row.amount?.amountEur || ((row.amount?.amountMicros || 0) / 1000000);
      return a ? String(Math.round(a)) : '';
    }
    if (col.key === 'closeDate') return fmtDate(row.closeDate) === '—' ? '' : fmtDate(row.closeDate);
  }
  if (obj === 'tasks') {
    if (col.key === 'title') return row.title || '';
    if (col.key === 'status') return row.status || 'TODO';
    if (col.key === 'dueAt') return fmtDate(row.dueAt) === '—' ? '' : fmtDate(row.dueAt);
    if (col.key === 'bodyMarkdown') return row.bodyMarkdown || '';
  }
  if (obj === 'notes') {
    if (col.key === 'title') return row.title || '';
    if (col.key === 'bodyMarkdown') return row.bodyMarkdown || '';
  }
  return '';
}

function computeFooterCellHtml(col) {
  const obj = state.object;
  if (col.primary) return `Alle zählen <span class="t-agg-val">${fmtNum(state.totalCount)}</span>`;
  if (obj === 'companies') {
    if (col.key === 'domainUrl') {
      const cnt = state.rows.filter(c => cleanDomain(c.domainName?.primaryLinkUrl)).length;
      const pct = state.fieldFilters.length
        ? (state.rows.length ? Math.round((cnt / state.rows.length) * 100) + '%' : '0%')
        : (state.totalCount && state.aggregates ? Math.round((state.aggregates.withDomainCount / state.totalCount) * 100) + '%' : '—');
      return `% nicht leer <span class="t-agg-val">${pct}</span>`;
    }
    if (col.key === 'idealCustomerProfile') {
      const cnt = state.fieldFilters.length
        ? state.rows.filter(c => c.idealCustomerProfile).length
        : (state.aggregates ? state.aggregates.icpCount : state.rows.filter(c => c.idealCustomerProfile).length);
      return `Anzahl Ja <span class="t-agg-val">${fmtNum(cnt)}</span>`;
    }
    if (col.key === 'arrEur') {
      const sum = state.fieldFilters.length
        ? state.rows.reduce((s, c) => s + (c.annualRecurringRevenue?.amountEur || ((c.annualRecurringRevenue?.amountMicros || 0) / 1000000)), 0)
        : (state.aggregates?.totalArrEur ?? state.rows.reduce((s, c) => s + (c.annualRecurringRevenue?.amountEur || 0), 0));
      return `Summe <span class="t-agg-val">${sum ? fmtEur(sum) : '€0'}</span>`;
    }
    if (col.key === 'employees') {
      const mode = state.colPrefs[obj]?.aggs?.[col.key] || 'sum';
      const vals = state.rows.map(r => Number(r.employees || 0)).filter(n => !isNaN(n));
      const sum = vals.reduce((a, b) => a + b, 0);
      const val = mode === 'avg' ? (vals.length ? Math.round(sum / vals.length) : 0) : sum;
      return `${mode === 'avg' ? 'Ø' : 'Summe'} <span class="t-agg-val">${fmtNum(val)}</span>`;
    }
  }
  if (obj === 'opportunities' && col.key === 'amountEur') {
    const sum = state.rows.reduce((s, o) => s + (o.amount?.amountEur || ((o.amount?.amountMicros || 0) / 1000000)), 0);
    return `Summe <span class="t-agg-val">${sum ? fmtEur(sum) : '€0'}</span>`;
  }
  if (col.isCustom && (col.type === 'number' || col.type === 'currency')) {
    const vals = state.rows.map(r => Number(getRowFieldRaw(r, obj, col.key) || 0)).filter(n => !isNaN(n) && n !== 0);
    const sum = vals.reduce((a, b) => a + b, 0);
    return sum ? `Summe <span class="t-agg-val">${col.type === 'currency' ? fmtEur(sum) : fmtNum(sum)}</span>` : '';
  }
  return '';
}

function toggleAggMode(colKey) {
  state.colPrefs[state.object] = state.colPrefs[state.object] || {};
  state.colPrefs[state.object].aggs = state.colPrefs[state.object].aggs || {};
  const cur = state.colPrefs[state.object].aggs[colKey] || 'sum';
  state.colPrefs[state.object].aggs[colKey] = cur === 'sum' ? 'avg' : 'sum';
  saveColPrefs();
  updateTableFooterAggregates();
}

function updateTableFooterAggregates() {
  const cols = getVisibleColumnsForObject(state.object);
  const tds = document.querySelectorAll('table.t-grid tfoot td[data-foot-col]');
  tds.forEach(td => {
    const col = cols.find(c => c.key === td.dataset.footCol);
    if (col) td.innerHTML = computeFooterCellHtml(col);
  });
}

function renderTableView() {
  const sc = document.getElementById('stageContainer');
  const cols = getVisibleColumnsForObject(state.object);
  const allChecked = state.rows.length > 0 && state.rows.every(r => state.selectedIds.has(r.id));

  let headCols = `<th class="sticky-chk" style="width:32px;"><input type="checkbox" ${allChecked ? 'checked' : ''} onchange="toggleSelectAll(this.checked)"></th>`;
  headCols += cols.map(c => {
    const sortInd = state.clientSort?.field === c.key ? (state.clientSort.dir === 'asc' ? '↑' : '↓') : '';
    return `<th class="${c.sticky ? 'sticky-col' : ''}" data-col-key="${esc(c.key)}" style="width:${c.width}px;" onclick="openColumnMenu(event,'${esc(c.key)}')">
      <div class="t-th-inner">${ICONS[c.icon] || ICONS.tag}<span>${esc(c.label)}</span>${sortInd ? `<span class="t-th-sort-ind">${sortInd}</span>` : ''}</div>
      <div class="t-col-resizer" onclick="event.stopPropagation()" onmousedown="startColResize(event,'${esc(c.key)}')"></div>
    </th>`;
  }).join('');
  headCols += `<th class="t-th-add-col" title="Spalten verwalten oder neues Feld anlegen" onclick="openAddColumnMenu(event)">+</th>`;

  const bodyRows = state.rows.map(row => {
    const chk = state.selectedIds.has(row.id);
    const tds = cols.map(c => {
      const rawVal = getRawEditValue(row, c);
      const optsAttr = c.isCustom ? `data-options="${esc(JSON.stringify(c.options || []))}"` : '';
      const editableAttrs = c.editable
        ? `data-editable="1" data-custom="${c.isCustom ? '1' : '0'}" data-obj="${esc(state.object)}" data-id="${esc(row.id)}" data-field="${esc(c.key)}" data-type="${esc(c.type)}" ${optsAttr} data-val="${esc(rawVal)}" onclick="onCellClick(event,this)" ondblclick="onCellDblClick(event,this)"`
        : `onclick="focusCell(this)"`;
      return `<td class="${c.sticky ? 'sticky-col' : ''}" ${editableAttrs}>${renderCellContent(row, c)}</td>`;
    }).join('');
    return `<tr class="${chk ? 'selected' : ''}">
      <td class="sticky-chk"><input type="checkbox" ${chk ? 'checked' : ''} onchange="toggleRowSelect('${esc(row.id)}',this.checked)"></td>
      ${tds}
      <td></td>
    </tr>`;
  }).join('');

  const colSpanCount = cols.length + 2;
  const inlineAddRow = state.inlineAddOpen
    ? `<tr class="t-add-row">
        <td class="sticky-chk">+</td>
        <td class="sticky-col" colspan="${colSpanCount - 1}" style="padding:0 6px;">
          <div style="display:flex;align-items:center;gap:8px;height:32px;">
            <input id="inlineNewInput" class="t-inline-input" style="flex:1;height:28px;border:1px solid var(--t-focus-ring);border-radius:4px;padding:0 8px;background:var(--t-bg-primary);" placeholder="${state.object==='people'?'Vorname Nachname eingeben + Enter...':'Name / Titel eingeben + Enter...'}" onkeydown="onInlineNewKey(event)">
            <button class="t-btn t-btn-primary" style="height:24px;font-size:11.5px;" onclick="submitInlineNew()">Anlegen</button>
            <button class="t-btn" style="height:24px;font-size:11.5px;" onclick="state.inlineAddOpen=false;renderTableView()">Abbrechen</button>
          </div>
        </td>
      </tr>`
    : `<tr class="t-add-row" onclick="activateInlineNew()">
        <td class="sticky-chk">+</td>
        <td class="sticky-col" colspan="${colSpanCount - 1}" style="font-size:12.5px;">+ Neu hinzufügen</td>
      </tr>`;

  const footRow = `<td class="sticky-chk"></td>`
    + cols.map(c => {
      const isNumAgg = (state.object === 'companies' && c.key === 'employees');
      const align = (c.type === 'number' || c.type === 'currency') ? 'text-align:right;' : '';
      return `<td class="${c.sticky ? 'sticky-col' : ''} ${isNumAgg ? 't-agg-click' : ''}" data-foot-col="${esc(c.key)}" style="${align}" ${isNumAgg ? `onclick="toggleAggMode('${esc(c.key)}')" title="Summe / Durchschnitt wechseln"` : ''}>${computeFooterCellHtml(c)}</td>`;
    }).join('')
    + `<td></td>`;

  sc.innerHTML = `<div class="t-table-wrap">
    <table class="t-grid">
      <thead><tr>${headCols}</tr></thead>
      <tbody>
        ${bodyRows || `<tr><td colspan="${colSpanCount}" style="text-align:center;padding:32px;color:var(--t-text-tertiary);">Keine Datensätze gefunden.</td></tr>`}
        ${inlineAddRow}
      </tbody>
      <tfoot><tr>${footRow}</tr></tfoot>
    </table>
  </div>`;
  renderBulkBar();
  if (state.inlineAddOpen) setTimeout(() => document.getElementById('inlineNewInput')?.focus(), 20);
}

function activateInlineNew() {
  state.inlineAddOpen = true;
  renderTableView();
}
function onInlineNewKey(ev) {
  if (ev.key === 'Enter') { ev.preventDefault(); submitInlineNew(); }
  else if (ev.key === 'Escape') { ev.preventDefault(); state.inlineAddOpen = false; renderTableView(); }
}
async function submitInlineNew() {
  const val = document.getElementById('inlineNewInput')?.value?.trim();
  if (!val) { state.inlineAddOpen = false; renderTableView(); return; }
  const obj = state.object;
  const payload = { workspace: state.workspace };
  if (obj === 'people') {
    const parts = val.split(/\s+/);
    payload.firstName = parts[0] || val;
    payload.lastName = parts.slice(1).join(' ') || '—';
  } else if (obj === 'tasks') {
    payload.title = val; payload.status = 'TODO';
  } else if (obj === 'notes') {
    payload.title = val; payload.bodyMarkdown = '';
  } else if (obj === 'opportunities') {
    payload.name = val; payload.stage = 'NEW';
  } else {
    payload.name = val;
  }
  const r = await fetch(`/api/twenty/${obj}`, {
    method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload)
  });
  if (r.ok) {
    state.inlineAddOpen = false;
    toast('Eintrag angelegt ✓');
    loadStats();
    loadCurrentView();
  } else {
    toast('Anlegen fehlgeschlagen');
  }
}

// Inline Cell Focus & Single-Click Editing
function focusCell(td) {
  document.querySelectorAll('td.cell-focused').forEach(el => el.classList.remove('cell-focused'));
  td.classList.add('cell-focused');
  state.focusedCell = td;
}

function onCellClick(ev, td) {
  focusCell(td);
  const type = td.dataset.type || 'text';
  const field = td.dataset.field;
  // Primary name/title text opens Record Drawer; clicking cell whitespace starts inline edit
  if (td.classList.contains('sticky-col') && ev.target.closest('.t-name-text,.t-open-hover')) return;
  if (field === 'idealCustomerProfile') {
    const nextVal = td.dataset.val !== 'true';
    toggleCompanyIcp(td.dataset.id, nextVal);
    return;
  }
  if (type === 'stage' || type === 'task_status') {
    openInlineChoiceMenu(ev, td);
    return;
  }
  startEditCell(td);
}

function onCellDblClick(ev, td) {
  ev.stopPropagation();
  focusCell(td);
  startEditCell(td);
}

function openInlineChoiceMenu(ev, td) {
  ev.stopPropagation();
  const obj = td.dataset.obj, id = td.dataset.id, field = td.dataset.field, cur = td.dataset.val;
  const fm = document.getElementById('floatMenu');
  const rect = td.getBoundingClientRect();
  fm.style.left = Math.min(rect.left, window.innerWidth - 210) + 'px';
  fm.style.top = (rect.bottom + 2) + 'px';
  const items = field === 'stage'
    ? STAGES.map(s => ({ v: s, l: STAGE_LABEL[s] || s, cls: STAGE_CLASS[s] }))
    : TASK_STATUSES.map(s => ({ v: s, l: TASK_STATUS_LABEL[s] || s, cls: TASK_STATUS_CLASS[s] }));
  fm.innerHTML = items.map(it => `
    <button class="t-pop-item" onclick="closePops();patchRecord('${esc(obj)}','${esc(id)}',{'${esc(field)}':'${esc(it.v)}'})">
      <span class="t-tag ${it.cls}">${esc(it.l)}</span>
      <span>${cur === it.v ? '✓' : ''}</span>
    </button>
  `).join('');
  fm.classList.add('open');
}

function startEditCell(td) {
  if (!td || td.classList.contains('cell-editing')) return;
  const obj = td.dataset.obj, id = td.dataset.id, field = td.dataset.field;
  const oldVal = td.dataset.val ?? '';
  const type = td.dataset.type || 'text';
  const isCustom = td.dataset.custom === '1';
  const origHtml = td.innerHTML;
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
    inp.type = (type === 'number' || type === 'currency') ? 'number' : type === 'date' ? 'date' : 'text';
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
      let body = {};
      if (isCustom) {
        const parsed = (type === 'number' || type === 'currency') ? Number(newVal || 0) : type === 'boolean' ? (newVal === 'true') : newVal;
        body = { customFields: { [field]: parsed } };
      } else if (type === 'fullname') {
        const parts = newVal.trim().split(/\s+/);
        body = { firstName: parts[0] || newVal.trim(), lastName: parts.slice(1).join(' ') || '' };
      } else if (type === 'number' || type === 'currency') {
        body[field] = Math.round(Number(newVal || 0));
      } else if (type === 'boolean') {
        body[field] = newVal === 'true';
      } else {
        body[field] = newVal;
      }
      await patchRecord(obj, id, body);
    } else {
      td.innerHTML = origHtml;
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
    const updatedRow = await r.json().catch(() => null);
    toast('Gespeichert ✓');
    if (updatedRow) {
      const idx = state.rows.findIndex(x => x.id === id);
      if (idx >= 0) state.rows[idx] = updatedRow;
      const rIdx = state.rawRows.findIndex(x => x.id === id);
      if (rIdx >= 0) state.rawRows[rIdx] = updatedRow;
    }
    await loadCurrentView();
    if (state.openRecord && state.openRecord.id === id) {
      const updated = state.rows.find(x => x.id === id) || updatedRow;
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
  if (!ids.length || !confirm(`${ids.length} ausgewählte Einträge wirklich löschen?`)) return;
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
      const lbl = STAGE_LABEL[st] || st;
      return `<div class="t-kcol" data-stage="${st}" ondragover="event.preventDefault();this.classList.add('drag-over')" ondragleave="this.classList.remove('drag-over')" ondrop="this.classList.remove('drag-over');dropOpp(event,'${st}')">
        <div class="t-kcol-hdr">
          <div style="display:flex;align-items:center;gap:6px;">
            <span class="t-tag ${cls}">${lbl}</span>
            <span style="font-size:12px;color:var(--t-text-tertiary);">${cards.length}</span>
          </div>
          <div style="display:flex;align-items:center;gap:4px;">
            <span class="mono" style="font-size:11.5px;color:var(--t-text-secondary);">${sum ? fmtEur(sum) : '€0'}</span>
            <button class="t-icon-btn" style="width:22px;height:22px;" onclick="openCreateModal({stage:'${st}'})" title="In ${lbl} anlegen">+</button>
          </div>
        </div>
        <div class="t-kcol-list">
          ${cards.map(o => {
            const amt = o.amount?.amountEur || ((o.amount?.amountMicros || 0) / 1000000);
            return `<div class="t-kcard" draggable="true" ondragstart="this.classList.add('dragging');event.dataTransfer.setData('text/plain','${esc(o.id)}')" ondragend="this.classList.remove('dragging')" onclick="openRecord('opportunities','${esc(o.id)}')">
              <div class="t-kcard-title">${esc(o.name)}</div>
              ${o.company?.name ? `<div class="t-kcard-row">${ICONS.building}<span>${esc(o.company.name)}</span></div>` : ''}
              ${o.pointOfContact?.fullName ? `<div class="t-kcard-row">${ICONS.user}<span>${esc(o.pointOfContact.fullName)}</span></div>` : ''}
              <div style="display:flex;justify-content:space-between;align-items:center;font-size:11.5px;color:var(--t-text-secondary);margin-top:2px;">
                <span style="font-weight:600;color:var(--t-text-primary);">${fmtEur(amt)}</span>
                <span>${fmtDate(o.closeDate)}</span>
              </div>
            </div>`;
          }).join('')}
          <button class="t-btn-subtle" style="justify-content:center;font-size:12px;margin-top:2px;" onclick="openCreateModal({stage:'${st}'})">+ Neu</button>
        </div>
      </div>`;
    }).join('');
    sc.innerHTML = `<div class="t-kanban">${html}</div>`;
  } else if (state.object === 'tasks') {
    sc.innerHTML = `<div class="t-kanban">${TASK_STATUSES.map(st => {
      const cards = state.rows.filter(t => t.status === st);
      return `<div class="t-kcol" data-status="${st}" ondragover="event.preventDefault();this.classList.add('drag-over')" ondragleave="this.classList.remove('drag-over')" ondrop="this.classList.remove('drag-over');dropTask(event,'${st}')">
        <div class="t-kcol-hdr">
          <div style="display:flex;align-items:center;gap:6px;">
            <span class="t-tag ${TASK_STATUS_CLASS[st]}">${TASK_STATUS_LABEL[st] || st}</span>
            <span style="font-size:12px;color:var(--t-text-tertiary);">${cards.length}</span>
          </div>
          <button class="t-icon-btn" style="width:22px;height:22px;" onclick="openCreateModal({status:'${st}'})" title="Aufgabe anlegen">+</button>
        </div>
        <div class="t-kcol-list">
          ${cards.map(t => `<div class="t-kcard" draggable="true" ondragstart="this.classList.add('dragging');event.dataTransfer.setData('text/plain','${esc(t.id)}')" ondragend="this.classList.remove('dragging')" onclick="openRecord('tasks','${esc(t.id)}')">
            <div class="t-kcard-title">${esc(t.title)}</div>
            <div class="t-kcard-row">${ICONS.calendar}<span>${fmtDate(t.dueAt)}</span></div>
            ${t.companyName ? `<div class="t-kcard-row">${ICONS.building}<span>${esc(t.companyName)}</span></div>` : ''}
          </div>`).join('')}
          <button class="t-btn-subtle" style="justify-content:center;font-size:12px;margin-top:2px;" onclick="openCreateModal({status:'${st}'})">+ Neu</button>
        </div>
      </div>`;
    }).join('')}</div>`;
  }
}
async function dropOpp(ev, st) {
  ev.preventDefault(); const id = ev.dataTransfer.getData('text/plain');
  if (!id) return;
  const row = state.rows.find(x => x.id === id);
  if (row) { row.stage = st; renderKanbanView(); }
  await patchRecord('opportunities', id, { stage: st });
}
async function dropTask(ev, st) {
  ev.preventDefault(); const id = ev.dataTransfer.getData('text/plain');
  if (!id) return;
  const row = state.rows.find(x => x.id === id);
  if (row) { row.status = st; renderKanbanView(); }
  await patchRecord('tasks', id, { status: st });
}

// RightDrawer & Fullscreen ShowPage
async function openRecord(obj, id) {
  let row = state.rows.find(x => x.id === id);
  if (!row && obj === 'companies') {
    try {
      const r = await fetch(`/api/twenty/${obj}/${encodeURIComponent(id)}`);
      if (r.ok) row = await r.json();
    } catch {}
  }
  if (!row) {
    try {
      const r = await fetch(`/api/twenty/${obj}?workspace=${encodeURIComponent(state.workspace)}&limit=200`);
      if (r.ok) {
        const d = await r.json();
        row = (d.data || []).find(x => x.id === id);
      }
    } catch {}
  }
  if (!row) return;
  state.openRecord = { obj, id, row };
  state.rdTab = 'timeline';
  state.timeline = null;
  state.editingNoteId = null;
  document.querySelectorAll('#rdTabs .t-rd-tab').forEach((b, i) => b.classList.toggle('active', i === 0));
  const rd = document.getElementById('recordDrawer');
  rd.classList.add('open');
  rd.classList.toggle('fullscreen', state.recordFullscreen);
  renderRecordDrawer();
  await refreshRecordTimeline();
}
async function refreshRecordTimeline() {
  if (!state.openRecord) return;
  const { obj, id } = state.openRecord;
  const qKey = obj === 'companies' ? 'companyId' : obj === 'people' ? 'personId' : obj === 'opportunities' ? 'opportunityId' : null;
  if (qKey) {
    try {
      const r = await fetch(`/api/twenty/timeline?${qKey}=${encodeURIComponent(id)}`);
      state.timeline = await r.json();
      renderRecordDrawer();
    } catch {}
  }
}
function closeRecordDrawer() {
  state.recordFullscreen = false;
  const rd = document.getElementById('recordDrawer');
  rd.classList.remove('open');
  rd.classList.remove('fullscreen');
  document.getElementById('rdFullBtn').innerHTML = ICONS.maximize;
  document.getElementById('rdFullBtn').title = 'Im Vollbild öffnen';
}
function toggleRecordFullscreen() {
  state.recordFullscreen = !state.recordFullscreen;
  const rd = document.getElementById('recordDrawer');
  rd.classList.toggle('fullscreen', state.recordFullscreen);
  const btn = document.getElementById('rdFullBtn');
  btn.innerHTML = state.recordFullscreen ? ICONS.minimize : ICONS.maximize;
  btn.title = state.recordFullscreen ? 'Vollbild verlassen' : 'Im Vollbild öffnen';
  renderRecordDrawer();
}
function setRdTab(tab, btn) {
  state.rdTab = tab;
  document.querySelectorAll('.t-rd-tab').forEach(b => b.classList.remove('active'));
  if (btn) btn.classList.add('active');
  renderRecordDrawer();
}
async function renameOpenRecord(val) {
  if (!state.openRecord || !val.trim()) return;
  const { obj, id } = state.openRecord;
  if (obj === 'people') {
    const parts = val.trim().split(/\s+/);
    await patchRecord('people', id, { firstName: parts[0] || val.trim(), lastName: parts.slice(1).join(' ') || '' });
    return;
  }
  const field = (obj === 'tasks' || obj === 'notes') ? 'title' : 'name';
  await patchRecord(obj, id, { [field]: val.trim() });
}
async function deleteOpenRecord() {
  if (!state.openRecord) return;
  const { obj, id } = state.openRecord;
  if (!confirm('Diesen Datensatz wirklich löschen?')) return;
  const r = await fetch(`/api/twenty/${obj}/${encodeURIComponent(id)}`, { method: 'DELETE' });
  if (r.ok) {
    closeRecordDrawer();
    toast('Gelöscht');
    loadStats();
    loadCurrentView();
  }
}
async function togglePinOpenRecord() {
  if (!state.openRecord) return;
  const { obj, id, row } = state.openRecord;
  const found = (state.favorites || []).find(f => f.targetType === obj && f.targetId === id);
  if (found) { await deleteFavorite(found.id); renderRecordDrawer(); return; }
  const label = obj === 'people' ? row.fullName : (row.name || row.title || 'Eintrag');
  const r = await fetch('/api/twenty/favorites', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ workspace: state.workspace, label, targetType: obj, targetId: id })
  });
  if (r.ok) {
    state.favorites.push(await r.json());
    renderFavoritesSidebar();
    renderRecordDrawer();
    toast('Zu Favoriten hinzugefügt ★');
  }
}

function renderRecordDrawer() {
  if (!state.openRecord) return;
  const { obj, id, row } = state.openRecord;
  const title = obj === 'people' ? row.fullName : (row.name || row.title || 'Datensatz');
  const dom = row.domainName?.primaryLinkUrl || '';
  document.getElementById('rdAvatar').innerHTML = renderFavicon(dom, title);
  document.getElementById('rdTitleInput').value = title || '';
  const singularMap = { companies: 'Unternehmen', people: 'Kontakt', opportunities: 'Verkaufschance', tasks: 'Aufgabe', notes: 'Notiz' };
  document.getElementById('rdSub').textContent = `${singularMap[obj] || obj} · ${id.slice(0, 12)} · Erstellt am ${fmtDate(row.createdAt)}`;

  const isFav = (state.favorites || []).some(f => f.targetType === obj && f.targetId === id);
  const favBtn = document.getElementById('rdFavBtn');
  if (favBtn) {
    favBtn.innerHTML = isFav ? ICONS.starFilled : ICONS.star;
    favBtn.style.color = isFav ? 'var(--t-amber)' : 'var(--t-text-secondary)';
  }

  const body = document.getElementById('rdBody');
  let propsHtml = '';
  if (obj === 'companies') {
    const arr = row.annualRecurringRevenue?.amountEur || ((row.annualRecurringRevenue?.amountMicros || 0) / 1000000);
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.link} Domain</span><input class="t-prop-input" value="${esc(dom)}" onchange="patchRecord('companies','${esc(id)}',{domainUrl:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.pin} Stadt</span><input class="t-prop-input" value="${esc(row.address?.addressCity||'')}" onchange="patchRecord('companies','${esc(id)}',{city:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.users} Mitarbeiter</span><input class="t-prop-input" type="number" value="${row.employees ?? ''}" onchange="patchRecord('companies','${esc(id)}',{employees:Math.round(Number(this.value||0))})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.euro} ARR (EUR)</span><input class="t-prop-input" type="number" value="${arr ? Math.round(arr) : ''}" onchange="patchRecord('companies','${esc(id)}',{arrEur:Math.round(Number(this.value||0))})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.sparkles} ICP</span><select class="t-prop-input" onchange="patchRecord('companies','${esc(id)}',{idealCustomerProfile:this.value==='true'})"><option value="true" ${row.idealCustomerProfile?'selected':''}>Ja</option><option value="false" ${!row.idealCustomerProfile?'selected':''}>Nein</option></select></div>
    `;
  } else if (obj === 'people') {
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.mail} E-Mail</span><input class="t-prop-input" value="${esc(row.emails?.primaryEmail||'')}" onchange="patchRecord('people','${esc(id)}',{email:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.phone} Telefon</span><input class="t-prop-input" value="${esc(row.phones?.primaryPhoneNumber||'')}" onchange="patchRecord('people','${esc(id)}',{phone:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.options} Position</span><input class="t-prop-input" value="${esc(row.jobTitle||'')}" onchange="patchRecord('people','${esc(id)}',{jobTitle:this.value})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.pin} Stadt</span><input class="t-prop-input" value="${esc(row.city||'')}" onchange="patchRecord('people','${esc(id)}',{city:this.value})"></div>
      ${row.company?.name ? `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.building} Unternehmen</span><button class="t-tag t-tag-gray" style="border:none;cursor:pointer;width:fit-content;" onclick="openRecord('companies','${esc(row.company.id)}')">${esc(row.company.name)} ↗</button></div>` : ''}
    `;
  } else if (obj === 'opportunities') {
    const amt = row.amount?.amountEur || ((row.amount?.amountMicros || 0) / 1000000);
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} Phase</span><select class="t-prop-input" onchange="patchRecord('opportunities','${esc(id)}',{stage:this.value})">${STAGES.map(s=>`<option value="${s}" ${row.stage===s?'selected':''}>${STAGE_LABEL[s] || s}</option>`).join('')}</select></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.euro} Betrag (€)</span><input class="t-prop-input" type="number" value="${amt ? Math.round(amt) : ''}" onchange="patchRecord('opportunities','${esc(id)}',{amountEur:Math.round(Number(this.value||0))})"></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.calendar} Abschlussdatum</span><input class="t-prop-input" type="date" value="${esc(fmtDate(row.closeDate)==='—'?'':fmtDate(row.closeDate))}" onchange="patchRecord('opportunities','${esc(id)}',{closeDate:this.value})"></div>
      ${row.company?.name ? `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.building} Unternehmen</span><button class="t-tag t-tag-gray" style="border:none;cursor:pointer;width:fit-content;" onclick="openRecord('companies','${esc(row.company.id)}')">${esc(row.company.name)} ↗</button></div>` : ''}
    `;
  } else if (obj === 'tasks') {
    propsHtml = `
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.checkbox} Status</span><select class="t-prop-input" onchange="patchRecord('tasks','${esc(id)}',{status:this.value})">${TASK_STATUSES.map(s=>`<option value="${s}" ${row.status===s?'selected':''}>${TASK_STATUS_LABEL[s] || s}</option>`).join('')}</select></div>
      <div class="t-prop-row"><span class="t-prop-lbl">${ICONS.calendar} Fällig am</span><input class="t-prop-input" type="date" value="${esc(fmtDate(row.dueAt)==='—'?'':fmtDate(row.dueAt))}" onchange="patchRecord('tasks','${esc(id)}',{dueAt:this.value})"></div>
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
        return `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} ${esc(f.label)}</span><select class="t-prop-input" onchange="patchRecord('${esc(obj)}','${esc(id)}',{customFields:{'${esc(f.name)}':this.value==='true'}})"><option value="false" ${!b?'selected':''}>Nein</option><option value="true" ${b?'selected':''}>Ja</option></select></div>`;
      }
      if (ft === 'SELECT' && Array.isArray(f.options) && f.options.length) {
        return `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} ${esc(f.label)}</span><select class="t-prop-input" onchange="patchRecord('${esc(obj)}','${esc(id)}',{customFields:{'${esc(f.name)}':this.value}})"><option value="">—</option>${f.options.map(o => {
          const ov = typeof o === 'string' ? o : (o.value || o.label || '');
          const ol = typeof o === 'string' ? o : (o.label || o.value || '');
          return `<option value="${esc(ov)}" ${String(val)===ov?'selected':''}>${esc(ol)}</option>`;
        }).join('')}</select></div>`;
      }
      const inType = (ft === 'NUMBER' || ft === 'CURRENCY') ? 'number' : ft === 'DATE' ? 'date' : 'text';
      return `<div class="t-prop-row"><span class="t-prop-lbl">${ICONS.tag} ${esc(f.label)}</span><input class="t-prop-input" type="${inType}" value="${esc(val)}" onchange="patchRecord('${esc(obj)}','${esc(id)}',{customFields:{'${esc(f.name)}':this.type==='number'?Number(this.value||0):this.value}})"></div>`;
    }).join('');
  }

  const tl = state.timeline || { notes: [], tasks: [], opportunities: [], people: [], events: [] };
  let tabHtml = '';
  if (state.rdTab === 'timeline') {
    tabHtml = `
      <div style="display:flex;gap:6px;flex-wrap:wrap;padding:4px 0 8px;">
        <button class="t-btn" data-prompt="${esc('Erstelle einen kurzen personalisierten Cold-Email-Entwurf für '+title+' (draftOnly).')}" onclick="triggerAiPromptAttr(this)">${ICONS.sparkles} E-Mail-Entwurf</button>
        <button class="t-btn" data-prompt="${esc('Erstelle eine Follow-up-Aufgabe für '+title+' morgen um 10:00.')}" onclick="triggerAiPromptAttr(this)">${ICONS.checkbox} Follow-up morgen</button>
      </div>
      <div style="font-weight:600;font-size:12px;color:var(--t-text-secondary);">Aktivitäten &amp; Notizen</div>
      ${(tl.notes||[]).slice(0,5).map(n => `<div class="t-bubble"><strong>${esc(n.title)}</strong><div style="margin-top:3px;color:var(--t-text-secondary);">${esc((n.bodyMarkdown||'').slice(0,220))}</div></div>`).join('') || ''}
      ${(tl.events||[]).slice(0,8).map(ev => `<div style="font-size:12px;padding:6px 0;border-bottom:1px solid var(--t-border);display:flex;justify-content:space-between;gap:8px;"><span>${esc(ev.name)} · <span style="color:var(--t-text-secondary);">${esc(ev.details||'')}</span></span><span style="color:var(--t-text-tertiary);flex-shrink:0;">${fmtDate(ev.happensAt)}</span></div>`).join('') || (!tl.notes?.length ? '<div style="color:var(--t-text-tertiary);font-size:12px;">Noch keine Zeitleisten-Einträge.</div>' : '')}
    `;
  } else if (state.rdTab === 'tasks') {
    tabHtml = `
      <div style="display:flex;gap:6px;">
        <input id="rdNewTask" placeholder="Neue Aufgabe..." style="flex:1;height:30px;padding:0 8px;border:1px solid var(--t-border-strong);border-radius:5px;background:var(--t-bg-primary);" onkeydown="if(event.key==='Enter')addLinkedTask()">
        <button class="t-btn t-btn-primary" style="height:30px;" onclick="addLinkedTask()">+ Hinzufügen</button>
      </div>
      ${(tl.tasks||[]).map(t => `<div style="display:flex;align-items:center;justify-content:space-between;gap:8px;padding:7px 8px;border:1px solid var(--t-border);border-radius:6px;">
        <label style="display:flex;align-items:center;gap:7px;cursor:pointer;flex:1;min-width:0;">
          <input type="checkbox" ${t.status==='DONE'?'checked':''} onchange="patchLinkedTask('${esc(t.id)}',this.checked)">
          <span style="${t.status==='DONE'?'text-decoration:line-through;color:var(--t-text-tertiary);':''}">${esc(t.title)}</span>
        </label>
        <span class="t-tag ${TASK_STATUS_CLASS[t.status]||'t-tag-gray'}">${esc(TASK_STATUS_LABEL[t.status] || t.status)}</span>
        <button class="t-icon-btn" style="width:22px;height:22px;" title="Löschen" onclick="deleteLinkedTask('${esc(t.id)}')">×</button>
      </div>`).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Aufgaben verknüpft.</div>'}
    `;
  } else if (state.rdTab === 'notes') {
    tabHtml = `
      <div style="display:flex;gap:6px;">
        <input id="rdNewNote" placeholder="Schnelle Notiz..." style="flex:1;height:30px;padding:0 8px;border:1px solid var(--t-border-strong);border-radius:5px;background:var(--t-bg-primary);" onkeydown="if(event.key==='Enter')addLinkedNote()">
        <button class="t-btn t-btn-primary" style="height:30px;" onclick="addLinkedNote()">+ Hinzufügen</button>
      </div>
      ${(tl.notes||[]).map(n => state.editingNoteId === n.id
        ? `<div class="t-bubble" style="display:flex;flex-direction:column;gap:6px;">
            <input id="editNoteTitle_${esc(n.id)}" class="t-prop-input" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);" value="${esc(n.title)}">
            <textarea id="editNoteBody_${esc(n.id)}" class="t-prop-input" rows="4" style="border:1px solid var(--t-border-strong);background:var(--t-bg-primary);">${esc(n.bodyMarkdown||'')}</textarea>
            <div style="display:flex;gap:6px;justify-content:flex-end;">
              <button class="t-btn" onclick="state.editingNoteId=null;renderRecordDrawer()">Abbrechen</button>
              <button class="t-btn t-btn-primary" onclick="saveEditedNote('${esc(n.id)}')">Speichern</button>
            </div>
          </div>`
        : `<div class="t-bubble">
            <div style="display:flex;justify-content:space-between;align-items:center;gap:6px;">
              <strong>${esc(n.title)}</strong>
              <span style="display:inline-flex;gap:2px;">
                <button class="t-icon-btn" style="width:22px;height:22px;" title="Notiz bearbeiten" onclick="state.editingNoteId='${esc(n.id)}';renderRecordDrawer()">${ICONS.edit}</button>
                <button class="t-icon-btn" style="width:22px;height:22px;" title="Notiz löschen" onclick="deleteLinkedNote('${esc(n.id)}')">×</button>
              </span>
            </div>
            <div style="margin-top:4px;color:var(--t-text-secondary);white-space:pre-wrap;">${esc((n.bodyMarkdown||'').slice(0,500))}</div>
          </div>`
      ).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Notizen verknüpft.</div>'}
    `;
  } else if (state.rdTab === 'relations') {
    tabHtml = `
      <div style="display:flex;justify-content:space-between;align-items:center;">
        <span style="font-weight:600;font-size:12px;color:var(--t-text-secondary);">Kontakte (${(tl.people||[]).length})</span>
      </div>
      ${(tl.people||[]).map(p => `<div class="t-kcard" style="cursor:pointer;" onclick="openRecord('people','${esc(p.id)}')"><strong>${esc(p.fullName)}</strong><span style="font-size:12px;color:var(--t-text-secondary);">${esc(p.emails?.primaryEmail||p.jobTitle||'')}</span></div>`).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Kontakte.</div>'}
      <div style="font-weight:600;font-size:12px;color:var(--t-text-secondary);margin-top:8px;">Verkaufschancen (${(tl.opportunities||[]).length})</div>
      ${(tl.opportunities||[]).map(o => `<div class="t-kcard" style="cursor:pointer;" onclick="openRecord('opportunities','${esc(o.id)}')"><strong>${esc(o.name)}</strong><span class="t-tag ${STAGE_CLASS[o.stage]||'t-tag-gray'}" style="width:fit-content;">${esc(STAGE_LABEL[o.stage] || o.stage)}</span></div>`).join('') || '<div style="color:var(--t-text-tertiary);font-size:12px;">Keine Verkaufschancen.</div>'}
    `;
  } else if (state.rdTab === 'ai') {
    tabHtml = `
      <div class="t-bubble"><strong>KI-Entwurfsmodus (draftOnly: true)</strong><br>Die Conrad KI Engine erstellt einen Entwurf und speichert ihn als CRM-Notiz — es wird niemals automatisch eine E-Mail versendet.</div>
      <button class="t-btn t-btn-primary" data-prompt="${esc('Recherchiere '+title+' im CRM und erstelle eine Notiz mit einem personalisierten Kaltakquise-E-Mail-Entwurf.')}" onclick="triggerAiPromptAttr(this)">${ICONS.sparkles} E-Mail-Entwurf generieren</button>
    `;
  }

  body.innerHTML = `<div class="t-rd-left-col">${propsHtml}</div><div class="t-rd-right-col">${tabHtml}</div>`;
}

async function addLinkedNote() {
  const txt = document.getElementById('rdNewNote')?.value?.trim();
  if (!txt || !state.openRecord) return;
  const payload = { title: 'Notiz', bodyMarkdown: txt, workspace: state.workspace };
  if (state.openRecord.obj === 'companies') payload.companyId = state.openRecord.id;
  if (state.openRecord.obj === 'people') payload.personId = state.openRecord.id;
  if (state.openRecord.obj === 'opportunities') payload.opportunityId = state.openRecord.id;
  await fetch('/api/twenty/notes', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
  toast('Notiz gespeichert ✓');
  await refreshRecordTimeline();
}
async function saveEditedNote(nid) {
  const title = document.getElementById('editNoteTitle_' + nid)?.value?.trim() || 'Notiz';
  const bodyMarkdown = document.getElementById('editNoteBody_' + nid)?.value || '';
  const r = await fetch(`/api/twenty/notes/${encodeURIComponent(nid)}`, {
    method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ title, bodyMarkdown })
  });
  if (r.ok) {
    state.editingNoteId = null;
    toast('Notiz aktualisiert ✓');
    await refreshRecordTimeline();
  }
}
async function deleteLinkedNote(nid) {
  await fetch(`/api/twenty/notes/${encodeURIComponent(nid)}`, { method: 'DELETE' });
  toast('Notiz gelöscht');
  await refreshRecordTimeline();
}
async function addLinkedTask() {
  const txt = document.getElementById('rdNewTask')?.value?.trim();
  if (!txt || !state.openRecord) return;
  const payload = { title: txt, status: 'TODO', workspace: state.workspace };
  if (state.openRecord.obj === 'companies') payload.companyId = state.openRecord.id;
  if (state.openRecord.obj === 'people') payload.personId = state.openRecord.id;
  if (state.openRecord.obj === 'opportunities') payload.opportunityId = state.openRecord.id;
  await fetch('/api/twenty/tasks', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
  toast('Aufgabe erstellt ✓');
  await refreshRecordTimeline();
}
async function patchLinkedTask(tid, done) {
  await fetch(`/api/twenty/tasks/${encodeURIComponent(tid)}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ status: done ? 'DONE' : 'TODO' }) });
  toast('Aufgabe aktualisiert ✓');
  await refreshRecordTimeline();
}
async function deleteLinkedTask(tid) {
  await fetch(`/api/twenty/tasks/${encodeURIComponent(tid)}`, { method: 'DELETE' });
  toast('Aufgabe gelöscht');
  await refreshRecordTimeline();
}
function triggerAiPromptAttr(btn) {
  const p = btn?.dataset?.prompt; if (!p) return;
  closeRecordDrawer();
  openAiDrawer();
  document.getElementById('aiInput').value = p;
  sendAi();
}

// Settings Mode
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

function inviteDemoMember() {
  const name = document.getElementById('dmName')?.value?.trim();
  const email = document.getElementById('dmEmail')?.value?.trim();
  const role = document.getElementById('dmRole')?.value || 'Mitglied';
  if (!name || !email) { toast('Bitte Name & E-Mail eingeben'); return; }
  state.demoMembers.push({ id: 'm_' + Date.now(), name, email, role, status: 'Eingeladen (Demo)' });
  toast('Demo-Mitglied hinzugefügt ✓');
  renderSettingsView();
}

function renderSettingsView() {
  const sc = document.getElementById('stageContainer');
  const s = state.settingsSection;
  const objs = state.metadata?.objects || [];
  const activeObj = objs.find(o => o.namePlural === state.activeMetaObj) || objs[0];
  const isDark = document.documentElement.classList.contains('dark');
  const curWs = WORKSPACES.find(w => w.id === state.workspace) || WORKSPACES[0];

  let bodyHtml = '';
  if (s === 'data-model') {
    const q = (state.fieldSearchQ || '').toLowerCase();
    const fields = (activeObj?.fields || []).filter(f => !q || f.label.toLowerCase().includes(q) || f.name.toLowerCase().includes(q));
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">Datenmodell</h2>
      <p style="margin:0 0 16px;color:var(--t-text-secondary);">Standard- &amp; benutzerdefinierte Felder für Arbeitsbereich <span class="mono">${esc(curWs.name)}</span></p>
      <div class="t-obj-grid">
        ${objs.map(o => `<div class="t-obj-tile ${activeObj && activeObj.namePlural === o.namePlural ? 'active' : ''}" onclick="state.activeMetaObj='${esc(o.namePlural)}';state.showNewFieldForm=false;renderSettingsView()">
          <div style="display:flex;justify-content:space-between;align-items:center;"><strong>${esc(o.labelPlural)}</strong><span class="t-tag t-tag-gray">${fmtNum(o.recordCount)}</span></div>
          <span class="mono" style="font-size:11px;color:var(--t-text-tertiary);">${esc(o.nameSingular)} · ${(o.fields||[]).length} Felder</span>
        </div>`).join('')}
      </div>
      ${activeObj ? `<div class="t-s-card">
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:10px;gap:8px;flex-wrap:wrap;">
          <strong>${esc(activeObj.labelPlural)} · Feld-Schema</strong>
          <div style="display:flex;gap:6px;flex-wrap:wrap;align-items:center;">
            <input class="t-prop-input" style="width:160px;border:1px solid var(--t-border-strong);background:var(--t-bg-secondary);" placeholder="Feld suchen..." value="${esc(state.fieldSearchQ)}" oninput="state.fieldSearchQ=this.value;renderSettingsView()">
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
        <div style="overflow-x:auto;">
          <table style="width:100%;border-collapse:collapse;">
            <thead><tr style="border-bottom:1px solid var(--t-border);color:var(--t-text-tertiary);font-size:11.5px;text-align:left;"><th style="padding:6px;">Anzeigename</th><th style="padding:6px;">API-Name</th><th style="padding:6px;">Typ</th><th style="padding:6px;">Quelle</th><th style="padding:6px;"></th></tr></thead>
            <tbody>
              ${fields.map(f => `<tr style="border-bottom:1px solid var(--t-border);">
                <td style="padding:7px 6px;font-weight:500;">${esc(f.label)}</td>
                <td class="mono" style="padding:7px 6px;font-size:12px;">${esc(f.name)}</td>
                <td style="padding:7px 6px;"><span class="t-tag t-tag-blue">${esc(f.type)}</span></td>
                <td style="padding:7px 6px;color:var(--t-text-secondary);">${f.isCustom ? '<span class="t-tag t-tag-purple">Benutzerdefiniert</span>' : 'Standard'}</td>
                <td style="padding:7px 6px;text-align:right;">${f.isCustom && f.id ? `<button class="t-btn t-btn-danger" style="height:24px;font-size:11px;" onclick="deleteMetadataField('${esc(f.id)}')">Löschen</button>` : ''}</td>
              </tr>`).join('')}
            </tbody>
          </table>
        </div>
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
          <div><strong>API-Schlüssel</strong><div style="font-size:11.5px;color:var(--t-text-secondary);">SHA-256 gehasht gespeichert; Klartext nur einmalig bei Erstellung sichtbar.</div></div>
        </div>
        ${state.lastCreatedToken ? `<div style="padding:10px;margin-bottom:10px;border-radius:6px;background:var(--t-green-soft);border:1px solid var(--t-green);font-size:12px;">
          <strong>Neuer API-Schlüssel (einmalig kopieren):</strong> <code class="mono" style="user-select:all;">${esc(state.lastCreatedToken)}</code>
        </div>` : ''}
        <div style="display:flex;gap:8px;margin-bottom:10px;flex-wrap:wrap;">
          <input id="akName" class="t-prop-input" style="flex:1;min-width:180px;border:1px solid var(--t-border-strong);" placeholder="Bezeichnung (z.B. Conrad Automation Sync)">
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
        <div class="t-wh-grid" style="display:grid;grid-template-columns:2fr 1fr 1fr auto;gap:8px;margin-bottom:10px;">
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
      <h2 style="margin:0 0 4px;font-size:17px;">Erscheinungsbild</h2>
      <p style="margin:0 0 14px;color:var(--t-text-secondary);">Darstellung, Farbmodus und Oberflächeneinstellungen</p>
      <div class="t-s-card">
        <strong>Design-Modus</strong>
        <div class="t-theme-grid">
          <button class="t-theme-card ${!isDark?'active':''}" onclick="setThemeMode('light')">
            <div style="height:46px;border-radius:5px;background:#ffffff;border:1px solid #e5e5e5;padding:6px;display:flex;gap:6px;">
              <div style="width:24px;background:#f5f5f5;border-radius:3px;"></div>
              <div style="flex:1;display:flex;flex-direction:column;gap:4px;"><div style="height:8px;background:#ebebeb;border-radius:2px;"></div><div style="height:8px;width:70%;background:#edf3ff;border-radius:2px;"></div></div>
            </div>
            <div style="display:flex;justify-content:space-between;align-items:center;"><strong>Hell</strong><span>${!isDark?'✓ Aktiv':''}</span></div>
          </button>
          <button class="t-theme-card ${isDark?'active':''}" onclick="setThemeMode('dark')">
            <div style="height:46px;border-radius:5px;background:#141414;border:1px solid #2c2c2c;padding:6px;display:flex;gap:6px;">
              <div style="width:24px;background:#202020;border-radius:3px;"></div>
              <div style="flex:1;display:flex;flex-direction:column;gap:4px;"><div style="height:8px;background:#282828;border-radius:2px;"></div><div style="height:8px;width:70%;background:#1b2740;border-radius:2px;"></div></div>
            </div>
            <div style="display:flex;justify-content:space-between;align-items:center;"><strong>Dunkel</strong><span>${isDark?'✓ Aktiv':''}</span></div>
          </button>
        </div>
      </div>
    `;
  } else if (s === 'workspace') {
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">Arbeitsbereich</h2>
      <p style="margin:0 0 14px;color:var(--t-text-secondary);">Mandanten-Verwaltung &amp; Demo-Datenbestand</p>
      <div class="t-s-card">
        <div class="t-prop-row"><span class="t-prop-lbl">Aktiver Bereich</span><strong>${esc(curWs.name)}</strong></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Kennung</span><span class="mono">${esc(curWs.id)}</span></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Subdomain</span><span class="mono">${esc(curWs.domain)}</span></div>
      </div>
      <div class="t-s-card">
        <strong>Arbeitsbereich wechseln</strong>
        <div style="display:flex;gap:8px;margin-top:10px;flex-wrap:wrap;">
          ${WORKSPACES.map(w => `<button class="t-btn ${state.workspace===w.id?'t-btn-primary':''}" onclick="switchWorkspace('${w.id}')">${esc(w.name)}</button>`).join('')}
        </div>
      </div>
      <div class="t-s-card" style="display:flex;align-items:center;justify-content:space-between;gap:12px;flex-wrap:wrap;">
        <div><strong>Demo-Datenbestand zurücksetzen</strong><div style="font-size:12px;color:var(--t-text-secondary);">Kuratierte deutsche KMU-Demodaten erneut initialisieren</div></div>
        <button class="t-btn" onclick="syncSqlDump()">Demo-Daten synchronisieren</button>
      </div>
    `;
  } else if (s === 'members') {
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">Mitglieder (Demo)</h2>
      <p style="margin:0 0 14px;color:var(--t-text-secondary);">Teamzugriffe und Rollen im Arbeitsbereich <span class="mono">${esc(curWs.name)}</span></p>
      <div class="t-s-card">
        <div style="display:grid;grid-template-columns:1fr 1fr 140px auto;gap:8px;margin-bottom:12px;" class="t-wh-grid">
          <input id="dmName" class="t-prop-input" style="border:1px solid var(--t-border-strong);" placeholder="Name (z.B. Anna Becker)">
          <input id="dmEmail" class="t-prop-input" style="border:1px solid var(--t-border-strong);" placeholder="anna@beispiel-demo.de">
          <select id="dmRole" class="t-prop-input" style="border:1px solid var(--t-border-strong);"><option value="Mitglied">Mitglied</option><option value="Administrator">Administrator</option></select>
          <button class="t-btn t-btn-primary" onclick="inviteDemoMember()">+ Einladen</button>
        </div>
        <table style="width:100%;border-collapse:collapse;">
          <thead><tr style="border-bottom:1px solid var(--t-border);color:var(--t-text-tertiary);font-size:11.5px;text-align:left;"><th style="padding:6px;">Mitglied</th><th style="padding:6px;">E-Mail</th><th style="padding:6px;">Rolle</th><th style="padding:6px;">Status</th></tr></thead>
          <tbody>
            ${state.demoMembers.map(m => {
              const [bg, fg] = colorFor(m.name);
              return `<tr style="border-bottom:1px solid var(--t-border);">
                <td style="padding:8px 6px;"><span style="display:inline-flex;align-items:center;gap:8px;font-weight:500;"><span class="t-avatar-sm" style="background:${bg};color:${fg};">${esc(initials(m.name))}</span>${esc(m.name)}</span></td>
                <td class="mono" style="padding:8px 6px;font-size:12px;color:var(--t-text-secondary);">${esc(m.email)}</td>
                <td style="padding:8px 6px;"><span class="t-tag t-tag-blue">${esc(m.role)}</span></td>
                <td style="padding:8px 6px;"><span class="t-tag t-tag-green">${esc(m.status)}</span></td>
              </tr>`;
            }).join('')}
          </tbody>
        </table>
      </div>
    `;
  } else {
    bodyHtml = `
      <h2 style="margin:0 0 4px;font-size:17px;">Profil</h2>
      <p style="margin:0 0 14px;color:var(--t-text-secondary);">Benutzerkonto &amp; lokale KI-Agent-Standards</p>
      <div class="t-s-card">
        <div class="t-prop-row"><span class="t-prop-lbl">Name</span><span>Merlin Conrad</span></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Arbeitsbereich</span><span class="mono">${esc(curWs.name)}</span></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Engine</span><span>⚡ Conrad KI Engine (bis zu 10× schneller &amp; 100× kompakter)</span></div>
        <div class="t-prop-row"><span class="t-prop-lbl">Sicherheitsregel</span><span>draftOnly = true (Kein automatischer E-Mail-Versand)</span></div>
      </div>
    `;
  }

  sc.innerHTML = `<div class="t-settings-wrap">
    <div class="t-settings-nav">
      <div class="t-nav-section" style="padding-top:2px;"><span>Benutzer</span></div>
      <button class="t-nav-item ${s==='profile'?'active':''}" onclick="openSettings('profile')"><span class="t-nav-left">${ICONS.user}<span>Profil</span></span></button>
      <button class="t-nav-item ${s==='experience'?'active':''}" onclick="openSettings('experience')"><span class="t-nav-left">${ICONS.sun}<span>Erscheinungsbild</span></span></button>
      <div class="t-nav-section"><span>Arbeitsbereich</span></div>
      <button class="t-nav-item ${s==='workspace'?'active':''}" onclick="openSettings('workspace')"><span class="t-nav-left">${ICONS.building}<span>Allgemein</span></span></button>
      <button class="t-nav-item ${s==='members'?'active':''}" onclick="openSettings('members')"><span class="t-nav-left">${ICONS.users}<span>Mitglieder</span></span></button>
      <button class="t-nav-item ${s==='data-model'?'active':''}" onclick="openSettings('data-model')"><span class="t-nav-left">${ICONS.database}<span>Datenmodell</span></span></button>
      <button class="t-nav-item ${s==='apis'?'active':''}" onclick="openSettings('apis')"><span class="t-nav-left">${ICONS.code}<span>APIs &amp; Webhooks</span></span></button>
      <div class="t-settings-back" style="margin-top:auto;padding-top:12px;"><button class="t-btn" style="width:100%;justify-content:center;" onclick="setObject('companies')">← Zurück zum CRM</button></div>
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
  const wait = document.createElement('div'); wait.className = 't-bubble'; wait.textContent = '⏳ Conrad KI Engine arbeitet...';
  msgs.appendChild(wait); msgs.scrollTop = msgs.scrollHeight;
  try {
    const r = await fetch('/api/twenty/agent', {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ prompt, model, workspace: state.workspace })
    });
    const d = await r.json();
    const calls = (d.toolCalls || []).map(c => `<span class="t-tag t-tag-blue mono">${esc(c.name)}</span>`).join(' ');
    wait.innerHTML = `<div style="margin-bottom:5px;display:flex;gap:5px;flex-wrap:wrap;align-items:center;"><span class="t-tag t-tag-gray">⚡ Conrad KI Engine</span><span class="t-tag t-tag-gray">${d.elapsedMs || 0}ms</span>${calls}</div><div>${esc(d.reply || JSON.stringify(d))}</div>`;
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
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crDom" placeholder="Domain (z.B. elektro-weber-demo.de)">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crCity" placeholder="Stadt">`;
  } else if (state.object === 'people') {
    document.getElementById('createTitle').textContent = 'Neuer Kontakt';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crFirst" placeholder="Vorname *">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crLast" placeholder="Nachname *">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crEmail" placeholder="E-Mail">
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crJob" placeholder="Position">`;
  } else if (state.object === 'opportunities') {
    document.getElementById('createTitle').textContent = 'Neue Verkaufschance';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crOppName" placeholder="Deal-Name *">
      <select class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crOppStage">${STAGES.map(s=>`<option value="${s}" ${s===preStage?'selected':''}>${STAGE_LABEL[s] || s}</option>`).join('')}</select>
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crOppAmt" type="number" placeholder="Wert (EUR)">`;
  } else if (state.object === 'tasks') {
    document.getElementById('createTitle').textContent = 'Neue Aufgabe';
    html = `<input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crTaskTitle" placeholder="Aufgabentitel *">
      <select class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crTaskStatus"><option value="TODO" ${preStatus==='TODO'?'selected':''}>Offen</option><option value="IN_PROGRESS" ${preStatus==='IN_PROGRESS'?'selected':''}>In Arbeit</option><option value="DONE" ${preStatus==='DONE'?'selected':''}>Erledigt</option></select>
      <input class="t-prop-input" style="border:1px solid var(--t-border-strong);" id="crTaskDue" type="date" placeholder="Fälligkeitsdatum">`;
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
  const cfMap = {};
  document.querySelectorAll('.cr-custom-input').forEach(el => {
    const val = el.value.trim();
    if (!val) return;
    const ft = (el.dataset.cftype || '').toUpperCase();
    cfMap[el.dataset.cfname] = (ft === 'NUMBER' || ft === 'CURRENCY') ? Number(val || 0) : ft === 'BOOLEAN' ? (val === 'true' || val === 'Ja') : val;
  });
  if (Object.keys(cfMap).length) payload.customFields = cfMap;
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
  state.cmdIdx = 0;
  document.getElementById('cmdBackdrop').classList.add('open');
  const inp = document.getElementById('cmdInput'); inp.value = ''; inp.focus();
  runCmdSearch();
}
function closeCmd() { document.getElementById('cmdBackdrop').classList.remove('open'); }
function onCmdKey(ev) {
  const items = document.querySelectorAll('#cmdResults .t-cmd-item');
  if (!items.length) return;
  if (ev.key === 'ArrowDown') {
    ev.preventDefault();
    state.cmdIdx = (state.cmdIdx + 1) % items.length;
    items.forEach((el, i) => el.classList.toggle('cmd-active', i === state.cmdIdx));
    items[state.cmdIdx]?.scrollIntoView({ block: 'nearest' });
  } else if (ev.key === 'ArrowUp') {
    ev.preventDefault();
    state.cmdIdx = (state.cmdIdx - 1 + items.length) % items.length;
    items.forEach((el, i) => el.classList.toggle('cmd-active', i === state.cmdIdx));
    items[state.cmdIdx]?.scrollIntoView({ block: 'nearest' });
  } else if (ev.key === 'Enter') {
    ev.preventDefault();
    items[state.cmdIdx]?.click();
  }
}
async function runCmdSearch() {
  const q = document.getElementById('cmdInput').value.trim();
  const out = document.getElementById('cmdResults');
  state.cmdIdx = 0;
  if (!q) {
    out.innerHTML = `
      <div class="t-pop-title">Schnellaktionen</div>
      <button class="t-pop-item t-cmd-item cmd-active" onclick="closeCmd();openCreateModal()"><span>${ICONS.plus} Neuen Eintrag anlegen</span><span class="t-kbd">Neu</span></button>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();openAiDrawer()"><span>${ICONS.sparkles} Conrad KI Engine fragen</span><span class="t-kbd">⌘J</span></button>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();toggleTheme()"><span>${ICONS.sun} Erscheinungsbild (Hell/Dunkel) wechseln</span></button>
      <div class="t-pop-sep"></div>
      <div class="t-pop-title">Navigation</div>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();setObject('companies')"><span>${ICONS.building} Zu Unternehmen wechseln</span></button>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();setObject('people')"><span>${ICONS.user} Zu Kontakten wechseln</span></button>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();setObject('opportunities')"><span>${ICONS.target} Zu Verkaufschancen (Kanban)</span></button>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();setObject('tasks')"><span>${ICONS.checkbox} Zu Aufgaben wechseln</span></button>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();setObject('notes')"><span>${ICONS.notes} Zu Notizen wechseln</span></button>
      <button class="t-pop-item t-cmd-item" onclick="closeCmd();openSettings('data-model')"><span>${ICONS.settings} Einstellungen &amp; Datenmodell</span></button>
      <a class="t-pop-item t-cmd-item" href="/n8n"><span>${ICONS.workflow} Conrad Automation öffnen</span></a>
    `;
    return;
  }
  const [cR, pR, oR, tR] = await Promise.all([
    fetch(`/api/twenty/companies?workspace=${encodeURIComponent(state.workspace)}&q=${encodeURIComponent(q)}&limit=5`).then(r => r.json()),
    fetch(`/api/twenty/people?workspace=${encodeURIComponent(state.workspace)}&q=${encodeURIComponent(q)}&limit=4`).then(r => r.json()),
    fetch(`/api/twenty/opportunities?workspace=${encodeURIComponent(state.workspace)}&q=${encodeURIComponent(q)}&limit=4`).then(r => r.json()),
    fetch(`/api/twenty/tasks?workspace=${encodeURIComponent(state.workspace)}&q=${encodeURIComponent(q)}&limit=4`).then(r => r.json())
  ]);
  let html = '';
  (cR.data || []).forEach(c => { html += `<button class="t-pop-item t-cmd-item" onclick="closeCmd();openRecord('companies','${esc(c.id)}')"><span>${ICONS.building} ${esc(c.name)}</span><span class="t-tag t-tag-gray">${esc(c.address?.addressCity || 'Unternehmen')}</span></button>`; });
  (pR.data || []).forEach(p => { html += `<button class="t-pop-item t-cmd-item" onclick="closeCmd();openRecord('people','${esc(p.id)}')"><span>${ICONS.user} ${esc(p.fullName)}</span><span class="t-tag t-tag-gray">${esc(p.emails?.primaryEmail || 'Kontakt')}</span></button>`; });
  (oR.data || []).forEach(o => { html += `<button class="t-pop-item t-cmd-item" onclick="closeCmd();setObject('opportunities');setTimeout(()=>openRecord('opportunities','${esc(o.id)}'),250)"><span>${ICONS.target} ${esc(o.name)}</span><span class="t-tag t-tag-blue">${esc(STAGE_LABEL[o.stage] || o.stage)}</span></button>`; });
  (tR.data || []).forEach(t => { html += `<button class="t-pop-item t-cmd-item" onclick="closeCmd();setObject('tasks');setTimeout(()=>openRecord('tasks','${esc(t.id)}'),250)"><span>${ICONS.checkbox} ${esc(t.title)}</span><span class="t-tag t-tag-gray">${esc(TASK_STATUS_LABEL[t.status] || t.status)}</span></button>`; });
  out.innerHTML = html || '<div style="padding:16px;text-align:center;color:var(--t-text-tertiary);">Keine Treffer.</div>';
  out.querySelector('.t-cmd-item')?.classList.add('cmd-active');
}

// Export, CSV Import & Sync helpers
function exportCurrentJson() {
  const blob = new Blob([JSON.stringify(state.rows, null, 2)], { type: 'application/json' });
  const a = document.createElement('a'); a.href = URL.createObjectURL(blob); a.download = `conrad_crm_${state.object}.json`; a.click();
}
function exportBackendCsv(selectedOnly) {
  const ids = selectedOnly ? Array.from(state.selectedIds).join(',') : '';
  const url = `/api/twenty/objects/${encodeURIComponent(state.object)}/export-csv?workspace=${encodeURIComponent(state.workspace)}${ids ? '&ids=' + encodeURIComponent(ids) : ''}`;
  const a = document.createElement('a'); a.href = url; a.download = `conrad_crm_${state.object}.csv`; document.body.appendChild(a); a.click(); a.remove();
}

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
  toast('Synchronisiere Demo-Daten...');
  const r = await fetch('/api/twenty/sync-sql', { method: 'POST' });
  if (r.ok) { toast('Synchronisierung abgeschlossen ✓'); loadStats(); loadCurrentView(); }
}

document.addEventListener('keydown', e => {
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') { e.preventDefault(); openCmd(); }
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'j') { e.preventDefault(); openAiDrawer(); }
  if (e.key === 'Enter' && state.focusedCell && !document.activeElement?.matches('input,textarea,select,button')) {
    e.preventDefault();
    startEditCell(state.focusedCell);
  }
  if (e.key === 'Escape') { closeCmd(); closeRecordDrawer(); closeAiDrawer(); closeCreateModal(); closeSaveViewModal(); closeQuickFieldModal(); closeCsvModal(); closePops(); }
});

function syncResponsiveDrawer() {
  const isNarrow = window.innerWidth <= 900;
  const d = document.getElementById('tDrawer');
  const btn = document.getElementById('expandDrawerBtn');
  if (isNarrow && !d.classList.contains('collapsed')) {
    d.classList.add('collapsed');
    document.body.classList.add('drawer-collapsed');
    if (btn) btn.style.display = 'inline-flex';
  }
}
window.addEventListener('resize', syncResponsiveDrawer);
syncResponsiveDrawer();
updateWsBadge();
loadMetadata();
loadSavedViews();
loadFavorites();
loadStats();
setObject('companies');
</script>
</body>
</html>"######
}
