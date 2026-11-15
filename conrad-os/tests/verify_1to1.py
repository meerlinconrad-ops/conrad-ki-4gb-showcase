import asyncio
import json
import os
import urllib.request
from playwright.async_api import async_playwright

OUT_DIR = "/Users/merlinconrad/.gemini/antigravity/brain/169cc662-c4df-4bbe-9580-724be8ddd48e/scratch"
BASE = "https://crm.conrad-ki.com"

def http_req(path, method="GET", body=None):
    url = BASE + path
    data = json.dumps(body).encode("utf-8") if body is not None else None
    headers = {"Content-Type": "application/json"} if body is not None else {}
    req = urllib.request.Request(url, data=data, headers=headers, method=method)
    with urllib.request.urlopen(req, timeout=45) as resp:
        return resp.status, json.loads(resp.read().decode("utf-8"))

async def run_playwright():
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        console_errs = []
        page_errs = []

        # 1. Desktop Twenty CRM (1440x900)
        ctx = await browser.new_context(viewport={"width": 1440, "height": 900})
        page = await ctx.new_page()
        page.on("console", lambda m: console_errs.append(f"[twenty] {m.text}") if m.type == "error" else None)
        page.on("pageerror", lambda e: page_errs.append(f"[twenty] {e}"))

        await page.goto(f"{BASE}/twenty", wait_until="networkidle")
        await page.wait_for_selector("table.t-grid tbody tr td.sticky-col .t-name-text", timeout=15000)
        await page.wait_for_timeout(800)
        # Verify single-click cell focus ring works
        await page.locator("table.t-grid tbody tr td[data-field='city']").first.click()
        assert await page.locator("table.t-grid tbody tr td.cell-focused").count() == 1, "Cell focus ring missing!"
        overflow = await page.evaluate("document.documentElement.scrollWidth > window.innerWidth")
        assert not overflow, "Twenty desktop horizontal overflow!"
        await page.screenshot(path=f"{OUT_DIR}/twenty_desktop_table.png")

        # Open RightDrawer
        await page.locator("table.t-grid tbody tr td.sticky-col .t-name-text").first.click()
        await page.wait_for_selector("#recordDrawer.open", timeout=8000)
        await page.wait_for_timeout(500)
        await page.screenshot(path=f"{OUT_DIR}/twenty_right_drawer.png")
        await page.locator("#rdCloseBtn").click()
        await page.wait_for_timeout(200)

        # Opportunities By Stage Kanban
        await page.locator("#nav-opportunities").click()
        await page.wait_for_selector(".t-kanban .t-kcol", timeout=8000)
        await page.wait_for_timeout(400)
        await page.screenshot(path=f"{OUT_DIR}/twenty_kanban.png")

        # Settings -> Data model + APIs & Webhooks GraphQL
        await page.locator("#nav-settings").click()
        await page.wait_for_selector(".t-settings-wrap .t-obj-tile", timeout=8000)
        await page.wait_for_timeout(300)
        await page.screenshot(path=f"{OUT_DIR}/twenty_settings_datamodel.png")
        await page.get_by_text("APIs & Webhooks").click()
        await page.wait_for_selector("#gqlEditor", timeout=5000)
        await page.get_by_role("button", name="Execute GraphQL").click()
        await page.wait_for_timeout(700)
        gql_txt = await page.locator("#gqlOut").inner_text()
        assert "totalCount" in gql_txt, f"GraphQL output unexpected: {gql_txt[:150]}"
        await page.screenshot(path=f"{OUT_DIR}/twenty_graphql_playground.png")
        await ctx.close()

        # 2. Mobile Twenty CRM (390x844)
        ctx_m = await browser.new_context(viewport={"width": 390, "height": 844})
        pm = await ctx_m.new_page()
        pm.on("pageerror", lambda e: page_errs.append(f"[twenty-mob] {e}"))
        await pm.goto(f"{BASE}/twenty", wait_until="networkidle")
        await pm.wait_for_selector("table.t-grid tbody tr", timeout=15000)
        await pm.wait_for_timeout(500)
        assert not await pm.evaluate("document.documentElement.scrollWidth > window.innerWidth"), "Twenty mobile overflow!"
        await pm.screenshot(path=f"{OUT_DIR}/twenty_mobile.png")
        await ctx_m.close()

        # 3. Desktop n8n Editor-UI (1440x900)
        ctx_n = await browser.new_context(viewport={"width": 1440, "height": 900})
        pn = await ctx_n.new_page()
        pn.on("console", lambda m: console_errs.append(f"[n8n] {m.text}") if m.type == "error" else None)
        pn.on("pageerror", lambda e: page_errs.append(f"[n8n] {e}"))

        await pn.goto(f"{BASE}/n8n", wait_until="networkidle")
        await pn.wait_for_selector("#nodesLayer .n8n-node-box", timeout=15000)
        await pn.wait_for_timeout(600)
        assert not await pn.evaluate("document.documentElement.scrollWidth > window.innerWidth"), "n8n desktop overflow!"
        await pn.screenshot(path=f"{OUT_DIR}/n8n_desktop_canvas.png")

        # Verify Node Creator Slide-Over ("What happens next?")
        await pn.locator(".n8n-float-plus").click()
        await pn.wait_for_selector("#creatorDrawer.open .n8n-creator-item", timeout=5000)
        await pn.screenshot(path=f"{OUT_DIR}/n8n_creator_drawer.png")
        await pn.locator("#creatorDrawer button", has_text="×").click()
        await pn.wait_for_timeout(200)

        # Run Test workflow
        await pn.locator("#testWfBtn").click()
        await pn.wait_for_selector("#nodesLayer .n8n-exec-badge", timeout=45000)
        await pn.wait_for_timeout(400)
        await pn.screenshot(path=f"{OUT_DIR}/n8n_desktop_executed.png")

        # Double click first node to open 3-Column NDV Modal + Test step
        await pn.locator("#nodesLayer .n8n-node-box").first.dblclick()
        await pn.wait_for_selector("#ndvBackdrop.open", timeout=5000)
        await pn.locator("#ndvTestStepBtn").click()
        await pn.wait_for_timeout(1200)
        ndv_out = await pn.locator("#ndvOutBody").inner_text()
        assert "Noch keine Daten" not in ndv_out, "NDV Test step returned empty output!"
        await pn.screenshot(path=f"{OUT_DIR}/n8n_ndv_modal.png")
        await pn.get_by_text("Back to canvas").click()
        await pn.wait_for_timeout(200)

        # Open Overview (58 workflows)
        await pn.locator("#nav-overview").click()
        await pn.wait_for_selector("#ovList .n8n-wf-card", timeout=10000)
        await pn.wait_for_timeout(300)
        await pn.screenshot(path=f"{OUT_DIR}/n8n_overview.png")
        await ctx_n.close()

        # 4. Mobile n8n (390x844)
        ctx_nm = await browser.new_context(viewport={"width": 390, "height": 844})
        pnm = await ctx_nm.new_page()
        pnm.on("pageerror", lambda e: page_errs.append(f"[n8n-mob] {e}"))
        await pnm.goto(f"{BASE}/n8n", wait_until="networkidle")
        await pnm.wait_for_selector("#nodesLayer .n8n-node-box", timeout=15000)
        await pnm.wait_for_timeout(500)
        assert not await pnm.evaluate("document.documentElement.scrollWidth > window.innerWidth"), "n8n mobile overflow!"
        await pnm.screenshot(path=f"{OUT_DIR}/n8n_mobile.png")
        await ctx_nm.close()

        # 5. Showcase (/showcase)
        ctx_s = await browser.new_context(viewport={"width": 1440, "height": 900})
        ps = await ctx_s.new_page()
        await ps.goto(f"{BASE}/showcase", wait_until="networkidle")
        await ps.wait_for_timeout(900)
        rss = await ps.locator("#kRss").inner_text()
        assert rss != "— MB", f"Showcase metrics not loaded: {rss}"
        await ps.screenshot(path=f"{OUT_DIR}/showcase_desktop.png")
        await ctx_s.close()

        await browser.close()
        return console_errs, page_errs

if __name__ == "__main__":
    print("== 1. API Smoke Checks ==")
    st1, d1 = http_req("/api/twenty/companies?limit=10&icp=true")
    print(f"GET /api/twenty/companies?icp=true -> {st1}, count={len(d1.get('data', []))}, total={d1.get('totalCount')}, agg={d1.get('aggregates')}")
    st2, d2 = http_req("/api/n8n/workflows/conradKiAgent001/execute-node", "POST", {"nodeName": "Input Guard & Schema Prep", "payload": {"message": "Hallo Ohrladen"}})
    print(f"POST execute-node -> {st2}, ok={d2.get('ok')}, status={d2.get('status')}, ms={d2.get('durationMs')}")
    st3, d3 = http_req("/api/n8n/workflows/conradKiAgent001/execute", "POST", {"payload": {"message": "Fasse kurz zusammen"}})
    print(f"POST execute workflow -> {st3}, ok={d3.get('ok')}, ms={d3.get('durationMs')}, steps={len(d3.get('nodeLogs', []))}")
    st4, d4 = http_req("/api/twenty/agent", "POST", {"prompt": "Suche Firmen mit Name Ohrladen", "workspace": "conrad-ki", "model": "conrad-n8n-agent:0.6b"})
    print(f"POST /api/twenty/agent -> {st4}, model={d4.get('model')}, tools={[c.get('name') for c in d4.get('toolCalls', [])]}")

    print("== 2. Playwright UI Verification ==")
    c_errs, p_errs = asyncio.run(run_playwright())
    print(f"Console errors: {len(c_errs)} -> {c_errs}")
    print(f"Page errors: {len(p_errs)} -> {p_errs}")
    assert len(c_errs) == 0, f"Console errors detected: {c_errs}"
    assert len(p_errs) == 0, f"Page errors detected: {p_errs}"
    print("ALL VERIFICATIONS PASSED!")
