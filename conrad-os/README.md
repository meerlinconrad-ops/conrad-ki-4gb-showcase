# 🦀 Conrad OS — 1:1 Twenty CRM & n8n Engine in Pure Rust + Tiny-AI

**5.000 €/Monat Cloud- & SaaS-Stack ersetzt durch einen <20 €/Monat Server in 100 % Pure Rust.**  
Single Static Binary (`conrad-crm`, ~`14 MB`), **~`21,3 MB` VmRSS RAM** im laufenden Betrieb, **SQLite WAL** und nachtrainierte lokale **Tiny-AI-Modelle** (`conrad-n8n-agent:0.6b`, `conrad-gemma:270m`, `conrad-lfm2:1.2b`, `conrad-n8n-agent:1.7b`) via Ollama (`127.0.0.1:11434`).

---

## 🎯 Warum Pure Rust statt Node.js / Docker-Zoo?

| Komponente | Klassischer Stack (~5.000 €/Monat) | Conrad OS Pure-Rust Stack (<20 €/Monat) |
|---|---|---|
| **CRM** | Salesforce / HubSpot / Node Twenty CRM | **1:1 Twenty CRM** (`/twenty`, `/api/twenty/*`, `/graphql`) |
| **Workflows** | Zapier Enterprise / Make / n8n Cloud | **1:1 n8n Engine & Editor-UI** (`/n8n`, `/api/n8n/*`, `/webhook-rust/*`) |
| **KI & Tool Calling** | OpenAI / Claude Team + Cloud-APIs | **Lokales Ollama** (`0.6B ORPO`, `270M Gemma`, `1.2B LFM2.5`, `1.7B ORPO`) |
| **Datenbank & Hosting** | Managed Postgres + Redis + K8s | **1x Hetzner VPS (<20 €)** + **SQLite WAL** (`conrad_os.db`) |

---

## 🧩 1. 1:1 Twenty CRM in Pure Rust (`/twenty` · `/graphql` · `/api/twenty/*`)

Vollständige Pure-Rust-Implementierung in `src/twenty.rs` & `src/ui_twenty_n8n.rs`:

- **Workspace Navigation & Views:** `220px` `WorkspaceNavigationDrawer` mit Workspace-Wechsel (`Conrad KI` / `Pala Marketing`), `Favorites` (`All Companies`, `ICP Companies`, `By Stage`, `My Tasks`), `Objects` (`Companies`, `People`, `Opportunities`, `Tasks`, `Notes`, `Workflows`).
- **Spreadsheet `RecordTable`:** Sticky Name-Spalte mit Expand `[⤢]`, Domain-Pills, Avatar-Badges, Zell-Fokusrahmen, Doppelklick-Inline-Editing (`PATCH /api/twenty/*`), serverseitigem Sortieren (`sort=arr_desc`), Live-Filterung und **Bottom-Aggregate-Zeile** (`Count all`, `Percent not empty`, `Sum €`, `Count true`).
- **Drag-&-Drop Kanban:** `By Stage` (`NEW`, `SCREENING`, `MEETING`, `PROPOSAL`, `CUSTOMER`) und `By Status` mit Live-Summen pro Spalte.
- **`RightDrawer` Record Show Page:** Editierbare Record-Properties, Quick-Actions (`E-Mail-Entwurf` mit erzwungenem `draftOnly: true`, `Follow-up morgen`) und Tabs (`Timeline | Tasks | Notes | Relations | AI Drafts`).
- **Command Menu & Ask AI:** `⌘K` Global Search über Firmen, Kontakte, Deals & Workflows; `⌘J` autonomer CRM-Copilot (`POST /api/twenty/agent`) mit Tool Calling.
- **Settings & GraphQL Playground:** `Profile`, `Experience` (Light/Dark), `Data model` Schema-Inspektor und `APIs & Webhooks` mit interaktivem **GraphQL Playground** (`/graphql` & `/api/twenty/graphql`).

---

## ⚡ 2. 1:1 n8n Workflow Engine & Editor-UI in Pure Rust (`/n8n` · `/api/n8n/*` · `/webhook-rust/*`)

Vollständige Pure-Rust DAG-Engine & Editor-UI in `src/n8n_engine.rs` & `src/ui_twenty_n8n.rs`:

- **58 produktive Workflows** aus SQLite WAL inkl. `Tags`, `Settings`, `webhookPath`, `Duplicate` (`POST /api/n8n/workflows/:id/duplicate`) und `Delete` (`DELETE /api/n8n/workflows/:id`).
- **Editor-UI:** Kollabierbare dunkle Sidebar (`Overview`, `Templates`, `Variables`, `Insights`), `52px` Header mit Inline-Titel, `+ Add tag`, `Editor | Executions | Evaluations`, Modellwahl (`0.6B`/`270M`/`1.2B`/`1.7B`), `Active`-Toggle und `Save`.
- **Dot-Grid-Canvas:** SVG-Node-Icons für **29 Node-Typen**, D-förmige Trigger mit Blitz-Badge, AI-Agent-Root-Nodes mit Diamant-Ports & Sub-Nodes, Drag-&-Drop/Pan/Zoom, `What happens next?` Node-Creator-Drawer und Bottom-Logs-Panel.
- **3-Spaltiges NDV-Modal (`▶ Test step`):** `INPUT | Parameters/Settings + ▶ Test step | OUTPUT` über `POST /api/n8n/workflows/:id/execute-node` inkl. Ausführung vorgelagerter DAG-Nodes und ungespeicherter Parameter-Overrides.
- **Native Rust-Nodes:** `Webhook`, `Manual/Schedule/Chat/Email Trigger`, `HTTP Request`, `Code` (Sandbox), `If`, `Switch`, `Merge`, `Set`, `SplitInBatches`, `Postgres -> SQLite CRM Bridge`, `Google Sheets`, `Qdrant Vector Store`, `Ollama Embeddings & Chat Model`, `OpenAI Chat Model`, `Window Buffer Memory`, `AI Agent`, `RespondToWebhook`, `NoOp`, `ErrorTrigger` und `Twenty CRM Tool`.

---

## 📊 3. Showcase & Echtzeit-Telemetrie (`/showcase` · `/api/showcase/metrics`)

Zeigt den Live-Vergleich **5.000 €/Monat Cloud vs. <20 €/Monat Pure Rust**, echte Prozessmetriken (`VmRSS` ~`21,3 MB`), Produktivdaten (`2.677+` Firmen, `10.407+` Notizen, `58` Workflows), lokale Ollama-Tiny-Models und Live-Smoke-Tests direkt im Browser.

---

## 🚀 Build, Start & E2E-Verifikation

### Lokal / Bare-Metal bauen
```bash
cargo build --release
./target/release/conrad-crm
```

### Tests & Playwright-E2E-Verifikation
```bash
cargo test
python3 tests/verify_1to1.py
```

### Docker Compose
```bash
docker compose up -d
```

Standardmäßig erreichbar unter `http://localhost:8090` (`/twenty`, `/n8n`, `/showcase`, `/graphql`, `/mcp`).

---

## 🔒 Sicherheit & Lizenz
- DSGVO-Leitplanken: E-Mail-Entwürfe nutzen strikt `draftOnly: true` (kein automatischer Versand), Rate-Limiter & Strict CSP im Axum-Middleware-Layer.
- **Lizenz:** MIT License — frei modifizierbar und kommerziell nutzbar.
