# Conrad KI — 4-GB-RAM Self-Hosted AI & Rust Showcase Blueprint

**Version:** 2026-11-15  
**Autor:** Merlin Conrad · [conrad-ki.com](https://conrad-ki.com) · `info@conrad-ki.com`  
**Online lesen:** [conrad-ki.com/showcase-doku.html](https://conrad-ki.com/showcase-doku.html) · **GitHub:** [meerlinconrad-ops/conrad-ki-4gb-showcase](https://github.com/meerlinconrad-ops/conrad-ki-4gb-showcase)

Dieses Blueprint-Paket dokumentiert die produktive Architektur von **Conrad KI** auf einem einzelnen **3,7-GiB-RAM-Cloud-Server (2 vCPU)** — inklusive **25 aktiver Docker-Container**, **n8n-Automatisierung**, **Qdrant-Vektorsuche**, **lokaler LLM-Inferenz** und einem in **Rust** neu gebauten Microkernel-Relay (`4,3 MiB` RAM, `< 1 ms` Latenz).

---

## 1. Kernprinzip: Maximale Leistung pro Megabyte RAM

Typische KI-Setups verschwenden Gigabyte-weise RAM durch unkomprimierte KV-Caches, laufende Python/Node-Wrapper und unbereinigte Datenbanken. Mit vier gezielten Maßnahmen läuft derselbe Funktionsumfang stabil auf einem kleinen `4-GB`-Server ohne Cloud-API-Abhängigkeit:

| Optimierung | Vorher | Nachher | Effekt |
|---|---|---|---|
| **Rust Microkernel (`conrad-relays`)** | 4 Python-Daemons (`~10,2 MiB`) + `python:3.11-alpine` Container (`2,55 MiB`) | 1 Rust-Binary (`4,3 MiB` RSS) + `alpine/socat` Bridge (`992 KiB`) | **-59 % RAM**, **`0,43–0,70 ms`** Latenz, Backlog `5 -> 128` |
| **Ollama KV-Cache & Flash Attention** | FP16 KV-Cache (`1,6 GB` Modell-RAM) | `OLLAMA_KV_CACHE_TYPE=q8_0` + `OLLAMA_FLASH_ATTENTION=1` | **`1,4 GB` Modell-RAM**, **`20,19 tok/s`** warm (`1.7B Q3_K_S`) |
| **n8n SQLite Kompaktierung** | `2,8 GB` `database.sqlite` (`n8n` `366 MiB` RAM) | Bereinigung verwaister Executions + `VACUUM` | **`20 MB`** Datei, `n8n` sinkt auf ~`190 MiB` RAM |
| **Container & Kernel Tuning** | 33 Container, `2,1 GiB` Swap | 25 aktive Container, `vm.swappiness=10`, Container Memory-Caps | `1,7 GiB` Leerlauf-Baseline (`0 B` Swap) |

---

## 2. System-Architektur

```text
[Website conrad-ki.com (Vercel)]
   ├── POST /webhook/kira-chat-hardened  ──▶ Traefik (:443) ──▶ n8n
   │                                            ├── Regex Injection Guard
   │                                            ├── Ollama nomic-embed-text (768d)
   │                                            ├── Qdrant Top-4 Cosine Search
   │                                            └── Ollama 1.7B Q3_K_S (q8_0 KV)
   ├── POST /webhook/contact-form        ──▶ n8n (Name, E-Mail, Telefon, Quelle)
   └── Terminbuchung Modal               ──▶ Cal.com Self-Hosted (Memory-Cap 2 GiB)

[Webhooks / Buchungen / Solar-Leads]
   └── Ports :3010, :3050, :3051, :8080  ──▶ conrad-relays (1 Rust Prozess, 4,3 MiB RSS)
                                                ├── Async STARTTLS SMTP (tokio-rustls)
                                                ├── Gleichzeitiger Multi-Channel (tokio::join!)
                                                └── Strikte HTML-Escape-Sanitierung
```

---

## 3. Sicherheits- & Datenschutz-Leitplanken (DSGVO by Design)

1. **Keine öffentliche KI-API:** `ollama.service` bindet ausschließlich auf `127.0.0.1:11434`. Docker-Container sprechen Ollama nur intern über eine lokale `socat`/`nginx`-Bridge (`172.19.0.1:11434`) an.
2. **Kein automatischer E-Mail-Versand durch KI:** Sowohl `inbox-zero-classify` als auch `conrad-agent` erzwingen schema-seitig `draftOnly: true` und `markAsRead: false`.
3. **Deterministisches JSON statt Halluzinations-Prosa:** Produktive KI-Webhooks nutzen `response_format: { type: "json_object" }`, niedrige Temperatur (`0.1`) und Try-Catch-Schema-Validatoren in `n8n`.
4. **Prompt-Injection-Schutz:** Alle öffentlichen Chat- und Agent-Webhooks prüfen Eingaben vorab per Regex/Sanitizer und geben bei Treffern sofort eine neutrale Ablehnung (`blocked: true`) zurück.
5. **XSS-Schutz im Frontend & Mail-Relay:** Sowohl `chatbot.js` im Browser als auch `conrad-relays` im Rust-Backend escapen Nutzereingaben konsequent vor jeder HTML-Darstellung.

---

## 4. Enthaltene Blueprint-Dateien (im `.zip`-Paket)

- `4GB-SHOWCASE-BLUEPRINT.md` — Diese vollständige Dokumentation
- `relays-rs/` — Bereinigter Rust-Quellcode (`Cargo.toml`, `main.rs`, `relays.rs`, `smtp.rs`, `util.rs`, inkl. 7 Unit-Tests) ohne Zugangsdaten
- `workflows/` — Drei gehärtete, direkt in n8n importierbare Workflows:
  - `kira_rag_hardened_hybrid.json` (Qdrant Pre-Fetch RAG + History + CORS + Injection Guard)
  - `conrad_ki_local_agent_webhook.json` (Strict JSON Agent Router + `draft_only: true`)
  - `pala_inbox_zero_classifier.json` (Unread-Safe E-Mail-Klassifizierung & Antwort-Entwurf)
- `configs/` — Produktionsnahe Konfigurationsvorlagen:
  - `ollama-override.conf` (KV-Cache `q8_0`, Flash Attention, Localhost-Binding)
  - `99-conrad-low-ram.conf` (`vm.swappiness=10`)
  - `conrad-relays.service` (`systemd`-Unit für `--mode all`)
  - `Modelfile.1.7b_q3_k_s` (`PARSER qwen3` Tool-Calling-Fix für Ollama)

---

## 5. Schnellstart auf einem eigenen Ubuntu-22.04/24.04-VPS

### Schritt 1: Kernel & Ollama auf Minimal-RAM konfigurieren
```bash
sudo cp configs/99-conrad-low-ram.conf /etc/sysctl.d/
sudo sysctl --system

sudo mkdir -p /etc/systemd/system/ollama.service.d
sudo cp configs/ollama-override.conf /etc/systemd/system/ollama.service.d/override.conf
sudo systemctl daemon-reload && sudo systemctl restart ollama
```

### Schritt 2: Zero-Overhead-Aliase anlegen (kein doppelter RAM/Disk-Bedarf)
```bash
ollama pull nomic-embed-text
# Nach Anlegen des Zielmodells (z. B. conrad-n8n-agent:4b):
ollama cp conrad-n8n-agent:4b conrad-n8n-agent:1.7b
ollama cp conrad-n8n-agent:4b qwen2.5:1.5b
ollama cp conrad-n8n-agent:4b qwen2.5:latest
```

### Schritt 3: Rust-Relays testen & bauen
```bash
cd relays-rs
cargo test
cargo build --release
sudo cp target/release/conrad-relays /usr/local/bin/
```

### Schritt 4: n8n-Workflows importieren & Live testen
```bash
# Beispiel Test auf gehärtetes RAG:
curl -s -X POST -H "Content-Type: application/json" \
  -d '{"message":"Was kostet der Starter-Workflow?"}' \
  https://DEINE-DOMAIN/webhook/kira-chat-hardened
```

---

**Kontakt & individuelle Umsetzung für Ihren Betrieb:**  
Merlin Conrad · [conrad-ki.com](https://conrad-ki.com) · Erstgespräch unter [buchung.conrad-ki.com/merlin/30min](https://buchung.conrad-ki.com/merlin/30min)
