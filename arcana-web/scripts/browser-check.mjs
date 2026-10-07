#!/usr/bin/env node
// browser-check.mjs — the web client in headless Chrome: every page, in every
// theme, at three widths.
//
//   node arcana-web/scripts/browser-check.mjs --server "$CARGO_TARGET_DIR/debug/arcana-web" \
//        [--chrome PATH] [--shots DIR] [--require-narrow]
//   node arcana-web/scripts/browser-check.mjs --base http://127.0.0.1:8080 [...]
//
// With --server it starts that binary on a spare loopback port with scratch
// deck-store and art-cache directories and Scryfall unreachable (the proxy
// variables point at a closed port), so the check is offline and leaves
// nothing behind, and seeds three decks (a list, its variation and a fork)
// so My Decks and the Stage have something to lay out; with --base it checks
// a server already running and seeds nothing. Chrome is
// --chrome, $CHROME, or the Playwright cache's Chrome for Testing; it is
// driven over the DevTools protocol with Node's own WebSocket, no package.
//
// For each of /, /duel, /deck and /decks, in dark, light and cappuccino, at
// 390x844, 1000x800 and 1440x900, it requires:
//   - no uncaught exception;
//   - no failed request for /assets/, /theme.css or /app.js, and no
//     @font-face that failed to load;
//   - the page's serif and sans text, and any mana symbol, drawn from the
//     served web fonts (Spectral, Fira Sans, Mana), not from fonts installed
//     on this machine (CSS.getPlatformFontsForNode);
//   - the theme it was asked for on <html>;
//   - text at 4.5:1 (3:1 when large) against the background as rendered:
//     each text box's pixels are sampled from a screenshot taken with the
//     text hidden, so gradients, washes and glows count. Text over card art,
//     in the zoom, on a disabled control or covered by another element is
//     skipped; the contrast test in arcana-web/src/theme_contrast.rs holds
//     scrims over art at their worst;
//   - text on a glow (a radial-gradient background: the Stage's and My
//     Decks' halls, the champion panel) in one of the inks the contrast test
//     holds there: --text, --text-on-glow, --link or --link-hover;
//   - in the cockpit, lethal poison drawn opaque through its whole pulse,
//     with the pulse on its ring;
//   - no sideways scroll at 1000 px and wider. At 390 px it reports the
//     overflow and fails only with --require-narrow (A5.3 makes it a
//     requirement).
// --shots DIR writes a PNG per case as <page>-<theme>-<width>.png. For
// screenshots with card art, --online lets the server reach Scryfall and
// --art-cache DIR gives it a cache to fill and reuse across runs.
// Exit status 1 on any failure, with the failures listed.

import { spawn } from "node:child_process";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync, existsSync } from "node:fs";
import { createServer } from "node:net";
import { inflateSync } from "node:zlib";
import { tmpdir, homedir } from "node:os";
import { join } from "node:path";

const PAGES = [["stage", "/"], ["cockpit", "/duel"], ["deckbuilder", "/deck"], ["decks", "/decks"]];
const THEMES = ["dark", "light", "cappuccino"];
const WIDTHS = [[390, 844], [1000, 800], [1440, 900]];
// A served face may be reported by its CSS family or by the name in the font
// itself: Fira Sans's subsets are named Arcana Sans (its Reserved Font Name).
const WEB_FAMILIES = { serif: ["Spectral"], sans: ["Fira Sans", "Arcana Sans"], mana: ["Mana"] };

const args = process.argv.slice(2);
const opt = (name) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : undefined; };
const flag = (name) => args.includes(name);
const shots = opt("--shots");
const requireNarrow = flag("--require-narrow");

function chromePath() {
  const given = opt("--chrome") || process.env.CHROME;
  if (given) return given;
  const pw = join(homedir(), ".cache/ms-playwright/chromium-1243/chrome-linux64/chrome");
  if (existsSync(pw)) return pw;
  throw new Error("no Chrome: pass --chrome or set CHROME");
}

function freePort() {
  return new Promise((resolve, reject) => {
    const s = createServer();
    s.listen(0, "127.0.0.1", () => { const { port } = s.address(); s.close(() => resolve(port)); });
    s.on("error", reject);
  });
}

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function waitFor(url, ms) {
  const until = Date.now() + ms;
  while (Date.now() < until) {
    try { if ((await fetch(url)).ok) return; } catch (e) { /* not up yet */ }
    await sleep(200);
  }
  throw new Error("server did not answer " + url);
}

async function startServer(bin, scratch) {
  const port = await freePort();
  const env = { ...process.env, HOST: "127.0.0.1", PORT: String(port),
    ARCANA_DECK_STORE: join(scratch, "decks"), ARCANA_ART_CACHE: opt("--art-cache") || join(scratch, "art") };
  if (!flag("--online")) {
    for (const k of ["HTTP_PROXY", "HTTPS_PROXY", "http_proxy", "https_proxy"]) env[k] = "http://127.0.0.1:9";
  }
  delete env.MATCH_STATE_DIR;
  const child = spawn(bin, [], { env, stdio: ["ignore", "ignore", "pipe"] });
  child.stderr.on("data", () => {});
  const base = "http://127.0.0.1:" + port;
  await waitFor(base + "/theme.css", 120000);
  return { child, base };
}

async function startChrome(path, scratch) {
  const env = { ...process.env };
  for (const k of ["HTTP_PROXY", "HTTPS_PROXY", "http_proxy", "https_proxy"]) delete env[k];
  const child = spawn(path, ["--headless=new", "--remote-debugging-port=0", "--no-proxy-server",
    "--user-data-dir=" + join(scratch, "chrome"), "--no-first-run", "--no-default-browser-check",
    "--hide-scrollbars", "about:blank"], { env, stdio: ["ignore", "ignore", "pipe"] });
  const port = await new Promise((resolve, reject) => {
    let buf = "";
    child.stderr.on("data", (d) => {
      buf += d;
      const m = buf.match(/DevTools listening on ws:\/\/[^:]+:(\d+)\//);
      if (m) resolve(Number(m[1]));
    });
    child.on("exit", (code) => reject(new Error("chrome exited " + code)));
  });
  // The port is announced before the first tab is listed.
  for (let i = 0; i < 100; i++) {
    const targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
    const page = targets.find((t) => t.type === "page");
    if (page) return { child, wsUrl: page.webSocketDebuggerUrl };
    await sleep(100);
  }
  throw new Error("chrome opened no page");
}

class Cdp {
  constructor(ws) {
    this.ws = ws; this.id = 0; this.pending = new Map(); this.listeners = [];
    ws.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      if (msg.id && this.pending.has(msg.id)) {
        const { resolve, reject } = this.pending.get(msg.id);
        this.pending.delete(msg.id);
        msg.error ? reject(new Error(msg.error.message)) : resolve(msg.result);
      } else if (msg.method) {
        for (const l of this.listeners) l(msg);
      }
    });
  }
  static open(url) {
    return new Promise((resolve, reject) => {
      const ws = new WebSocket(url);
      ws.addEventListener("open", () => resolve(new Cdp(ws)));
      ws.addEventListener("error", reject);
    });
  }
  send(method, params = {}) {
    const id = ++this.id;
    this.ws.send(JSON.stringify({ id, method, params }));
    return new Promise((resolve, reject) => this.pending.set(id, { resolve, reject }));
  }
  once(method, ms) {
    return new Promise((resolve, reject) => {
      const t = setTimeout(() => reject(new Error("timed out waiting for " + method)), ms);
      const l = (msg) => {
        if (msg.method === method) {
          clearTimeout(t); this.listeners.splice(this.listeners.indexOf(l), 1); resolve(msg.params);
        }
      };
      this.listeners.push(l);
    });
  }
  async eval(expression) {
    const r = await this.send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
    if (r.exceptionDetails) throw new Error("evaluate: " + r.exceptionDetails.text);
    return r.result.value;
  }
}

// An 8-bit RGB or RGBA PNG, as Chrome captures one, to raw pixels.
function decodePng(buf) {
  let pos = 8, width = 0, height = 0, bpp = 0;
  const idat = [];
  while (pos < buf.length) {
    const len = buf.readUInt32BE(pos), type = buf.toString("ascii", pos + 4, pos + 8);
    const data = buf.subarray(pos + 8, pos + 8 + len);
    if (type === "IHDR") {
      width = data.readUInt32BE(0); height = data.readUInt32BE(4);
      if (data[8] !== 8 || data[12] !== 0 || ![2, 6].includes(data[9])) throw new Error("unexpected PNG format");
      bpp = data[9] === 6 ? 4 : 3;
    } else if (type === "IDAT") idat.push(data);
    else if (type === "IEND") break;
    pos += 12 + len;
  }
  const raw = inflateSync(Buffer.concat(idat));
  const stride = width * bpp, px = Buffer.alloc(height * stride);
  for (let y = 0; y < height; y++) {
    const filter = raw[y * (stride + 1)], line = y * (stride + 1) + 1;
    for (let x = 0; x < stride; x++) {
      const a = x >= bpp ? px[y * stride + x - bpp] : 0;
      const b = y > 0 ? px[(y - 1) * stride + x] : 0;
      const c = x >= bpp && y > 0 ? px[(y - 1) * stride + x - bpp] : 0;
      let v = raw[line + x];
      if (filter === 1) v += a;
      else if (filter === 2) v += b;
      else if (filter === 3) v += (a + b) >> 1;
      else if (filter === 4) {
        const p = a + b - c, pa = Math.abs(p - a), pb = Math.abs(p - b), pc = Math.abs(p - c);
        v += pa <= pb && pa <= pc ? a : pb <= pc ? b : c;
      }
      px[y * stride + x] = v & 255;
    }
  }
  return { width, height, bpp, px };
}

const lin = (c) => { c /= 255; return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4; };
const luminance = ([r, g, b]) => 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
const contrast = (a, b) => { const [x, y] = [luminance(a), luminance(b)].sort((p, q) => q - p); return (x + 0.05) / (y + 0.05); };

// Every visible text box the check can measure: its colour (with the
// opacity of its element and ancestors), size, weight and rectangle; and,
// on- or off-screen, every text on a glow that is not in a glow ink.
const TEXT_BOXES = `(() => {
  const out = [], glowFaults = [];
  const skipped = (el) => el.closest(".ac-card, .ac-zoom, i.ms, [hidden], :disabled, [aria-disabled='true']");
  const imgs = [...document.images].filter((i) => i.getClientRects().length).map((i) => i.getBoundingClientRect());
  const overImage = (r) => imgs.some((i) => r.left < i.right && r.right > i.left && r.top < i.bottom && r.bottom > i.top);
  const name = (el) => el.tagName.toLowerCase() + (el.id ? "#" + el.id : "") + [...el.classList].map((c) => "." + c).join("");
  // The nearest box that paints a background, and whether it is a glow.
  const onGlow = (el) => {
    for (let a = el; a; a = a.parentElement) {
      const cs = getComputedStyle(a);
      if (cs.backgroundImage !== "none") return cs.backgroundImage.includes("radial-gradient");
      if (cs.backgroundColor !== "rgba(0, 0, 0, 0)" && cs.backgroundColor !== "transparent") return false;
    }
    return false;
  };
  const probe = document.createElement("span");
  document.body.append(probe);
  const glowInks = ["--text", "--text-on-glow", "--link", "--link-hover"].map((t) => {
    probe.style.color = "var(" + t + ")";
    return getComputedStyle(probe).color;
  });
  probe.remove();
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  for (let n; (n = walker.nextNode());) {
    const el = n.parentElement;
    if (!el || !n.textContent.trim() || skipped(el)) continue;
    const cs = getComputedStyle(el);
    if (cs.visibility !== "visible" || !el.getClientRects().length) continue;
    if (onGlow(el) && !glowInks.includes(cs.color)) {
      glowFaults.push({ text: n.textContent.trim().slice(0, 40), el: name(el), color: cs.color });
    }
    let opacity = 1;
    for (let a = el; a; a = a.parentElement) opacity *= parseFloat(getComputedStyle(a).opacity);
    if (opacity < 0.05) continue;
    const range = document.createRange();
    range.selectNodeContents(n);
    for (const r of range.getClientRects()) {
      if (r.width < 3 || r.height < 6 || r.left < 0 || r.top < 0 || r.right > innerWidth || r.bottom > innerHeight) continue;
      if (overImage(r)) continue;
      const top = document.elementFromPoint((r.left + r.right) / 2, (r.top + r.bottom) / 2);
      if (!top || !(top === el || el.contains(top) || top.contains(el))) continue;
      out.push({ text: n.textContent.trim().slice(0, 40), el: name(el), color: cs.color, opacity,
        size: parseFloat(cs.fontSize), weight: parseInt(cs.fontWeight, 10), rect: [r.left, r.top, r.right, r.bottom] });
    }
  }
  return { boxes: out, glowFaults };
})()`;
const HIDE_TEXT = `(() => {
  const s = document.createElement("style");
  s.id = "check-hide-text";
  s.textContent = "*, *::before, *::after { color: transparent !important; -webkit-text-fill-color: transparent !important;" +
    " text-decoration-color: transparent !important; caret-color: transparent !important; transition: none !important; }";
  document.head.appendChild(s);
  return new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)));
})()`;

// Text against what is rendered behind it. Each box's pixels are sampled on
// a 2 px grid from a screenshot with the text hidden; the box holds if its
// tenth-percentile contrast does, so a stray border pixel does not fail it.
async function renderedContrast(cdp, fail) {
  const { boxes, glowFaults } = await cdp.eval(TEXT_BOXES);
  for (const g of glowFaults) {
    fail(`text "${g.text}" (${g.el}) sits on a glow in ${g.color}, not --text, --text-on-glow or a link colour`);
  }
  await cdp.eval(HIDE_TEXT);
  const { data } = await cdp.send("Page.captureScreenshot", { format: "png" });
  await cdp.eval(`document.getElementById("check-hide-text").remove()`);
  const img = decodePng(Buffer.from(data, "base64"));
  const reported = new Set();
  for (const b of boxes) {
    // rgb()/rgba(), or color(srgb r g b / a) for a color-mix().
    const m = b.color.match(/[\d.]+/g).map(Number);
    const scale = b.color.startsWith("color(") ? 255 : 1;
    const ink = m.slice(0, 3).map((c) => c * scale), alpha = (m.length > 3 ? m[3] : 1) * b.opacity;
    const [l, t, r, btm] = b.rect.map(Math.round);
    // Inside the screenshot: a mobile layout viewport can be wider than it.
    if (r > img.width || btm > img.height) continue;
    const ratios = [];
    for (let y = t + 1; y < btm - 1; y += 2) for (let x = l + 1; x < r - 1; x += 2) {
      const i = (y * img.width + x) * img.bpp;
      const bg = [img.px[i], img.px[i + 1], img.px[i + 2]];
      ratios.push(contrast(ink.map((c, k) => c * alpha + bg[k] * (1 - alpha)), bg));
    }
    if (!ratios.length) continue;
    ratios.sort((p, q) => p - q);
    const ratio = ratios[Math.floor(ratios.length / 10)];
    const large = b.size >= 24 || (b.size >= 18.66 && b.weight >= 700);
    const floor = large ? 3 : 4.5;
    const key = b.el + " " + b.text;
    if (ratio < floor && !reported.has(key)) {
      reported.add(key);
      fail(`text "${b.text}" (${b.el}) is ${ratio.toFixed(2)}:1 on its rendered background, under ${floor}:1`);
    }
  }
  return boxes.length;
}

// Lethal poison, put on the cockpit's first player strip and stepped through
// its pulse: the count stays opaque and the ring is what fades.
const LETHAL_STATE = `(() => {
  const hud = document.querySelector(".phud");
  if (!hud) return { error: "no .phud on the cockpit" };
  const el = document.createElement("span");
  el.className = "poison tnum lethal";
  el.innerHTML = '\u2620' + '9<span class="unit">/10 poison</span>';
  hud.append(el);
  const ring = el.getAnimations({ subtree: true }).find((a) => a.animationName === "poison-pulse");
  if (!ring) { el.remove(); return { error: "lethal poison has no poison-pulse animation" }; }
  ring.pause();
  const duration = ring.effect.getComputedTiming().duration;
  const steps = [];
  for (let k = 0; k <= 12; k++) {
    ring.currentTime = duration * k / 12;
    let count = 1;
    for (let a = el; a; a = a.parentElement) count *= parseFloat(getComputedStyle(a).opacity);
    const unit = parseFloat(getComputedStyle(el.querySelector(".unit")).opacity);
    steps.push({ count: count * unit, ring: parseFloat(getComputedStyle(el, "::after").opacity) });
  }
  const target = ring.effect.pseudoElement;
  el.remove();
  return { target, steps };
})()`;

async function lethalState(cdp, fail) {
  const r = await cdp.eval(LETHAL_STATE);
  if (r.error) return fail(r.error);
  if (r.target !== "::after") fail(`the lethal pulse animates ${r.target || "the count itself"}, not its ring`);
  const dim = r.steps.filter((s) => s.count < 1);
  if (dim.length) fail(`lethal poison text fades to opacity ${Math.min(...dim.map((s) => s.count))} during its pulse`);
  const rings = r.steps.map((s) => s.ring);
  if (!(Math.min(...rings) < 0.5 && Math.max(...rings) === 1)) fail(`the lethal ring does not pulse: ${rings.join(", ")}`);
}

// Marks the first element with its own text in each family, and the first
// mana symbol, so their platform fonts can be asked for.
const MARK_SAMPLES = `(() => {
  const own = (el) => [...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim());
  const found = {};
  for (const el of document.querySelectorAll("body *")) {
    if (el.closest("[hidden]") || !el.getClientRects().length) continue;
    const fam = getComputedStyle(el).fontFamily;
    if (!found.serif && /^"?Spectral"?,/.test(fam) && own(el)) { el.dataset.checkFont = "serif"; found.serif = true; }
    if (!found.sans && /^"?Fira Sans"?,/.test(fam) && own(el)) { el.dataset.checkFont = "sans"; found.sans = true; }
    if (!found.mana && el.matches("i.ms")) { el.dataset.checkFont = "mana"; found.mana = true; }
  }
  return found;
})()`;

// Three decks through the page's own store: /import resolves the list, and
// Arcana.Decks writes it to this origin's localStorage and the server's store.
const SEED_DECKS = `(async () => {
  await Arcana.Decks.ready;
  const r = await fetch("/import", { method: "POST", headers: { "content-type": "application/json" },
    body: JSON.stringify({ text: "4 Lightning Bolt\\n4 Shock\\n4 Goblin Guide\\n4 Monastery Swiftspear\\n4 Lava Spike\\n20 Mountain" }) });
  const d = await r.json();
  const id = Arcana.Decks.create("Mono-Red Burn", d.main, d.sideboard || []);
  Arcana.Decks.fork(id, null, "variation");
  Arcana.Decks.fork(id, "Gruul Burn", "fork");
  return Arcana.Decks.list().length;
})()`;

async function seed(cdp, base) {
  const loaded = cdp.once("Page.loadEventFired", 60000);
  await cdp.send("Page.navigate", { url: base + "/decks" });
  await loaded;
  const n = await cdp.eval(SEED_DECKS);
  if (n !== 3) throw new Error("seeding left " + n + " decks");
}

async function checkCase(cdp, base, [pageName, path], theme, [w, h], failures, notes, tally) {
  const label = `${pageName} ${theme} ${w}px`;
  const fail = (what) => failures.push(`${label}: ${what}`);
  await cdp.send("Emulation.setDeviceMetricsOverride", { width: w, height: h, deviceScaleFactor: 1, mobile: w < 800 });
  const { identifier } = await cdp.send("Page.addScriptToEvaluateOnNewDocument", {
    source: `try { localStorage.setItem("arcana.theme", ${JSON.stringify(theme)}); } catch (e) {}` });
  const exceptions = [];
  const requests = new Map();
  const failedAssets = [];
  const firstParty = (url) => url.startsWith(base) && /\/(assets\/|theme\.css|app\.js)/.test(url);
  const listener = (msg) => {
    if (msg.method === "Runtime.exceptionThrown") {
      const d = msg.params.exceptionDetails;
      exceptions.push((d.exception && d.exception.description) || d.text);
    } else if (msg.method === "Network.requestWillBeSent") {
      requests.set(msg.params.requestId, msg.params.request.url);
    } else if (msg.method === "Network.responseReceived") {
      const { url, status } = msg.params.response;
      if (firstParty(url) && status >= 400) failedAssets.push(`${url} ${status}`);
    } else if (msg.method === "Network.loadingFailed") {
      const url = requests.get(msg.params.requestId) || "?";
      if (firstParty(url)) failedAssets.push(`${url} ${msg.params.errorText}`);
    }
  };
  cdp.listeners.push(listener);
  try {
    const loaded = cdp.once("Page.loadEventFired", 60000);
    await cdp.send("Page.navigate", { url: base + path });
    await loaded;
    await cdp.eval("document.fonts.ready.then(() => new Promise((r) => setTimeout(r, 800)))");
    // Content width against the emulated width, not innerWidth: a mobile
    // layout viewport widens to fit content that overflows it.
    const m = await cdp.eval(`({ sw: Math.max(document.documentElement.scrollWidth, document.body.scrollWidth),
      theme: document.documentElement.dataset.theme,
      faces: [...document.fonts].map((f) => [f.family, f.weight, f.style, f.status]) })`);
    if (m.theme !== theme) fail(`<html data-theme> is ${m.theme}`);
    for (const [family, weight, style, status] of m.faces) {
      if (status === "error") fail(`@font-face ${family} ${weight} ${style} failed to load`);
    }
    if (m.sw > w) {
      const what = `overflows sideways: ${m.sw}px of content in ${w}px`;
      if (w >= 1000 || requireNarrow) fail(what); else notes.push(`${label}: ${what}`);
    }
    const found = await cdp.eval(MARK_SAMPLES);
    const { root } = await cdp.send("DOM.getDocument", { depth: 0 });
    for (const kind of ["serif", "sans", "mana"]) {
      if (!found[kind]) {
        if (kind !== "mana") fail(`no visible ${kind} text found to check`);
        continue;
      }
      const { nodeId } = await cdp.send("DOM.querySelector", { nodeId: root.nodeId, selector: `[data-check-font="${kind}"]` });
      const { fonts } = await cdp.send("CSS.getPlatformFontsForNode", { nodeId });
      const web = fonts.filter((f) => f.isCustomFont).map((f) => f.familyName);
      if (!web.some((name) => WEB_FAMILIES[kind].some((f) => name.startsWith(f)))) {
        fail(`${kind} text drawn from ${fonts.map((f) => f.familyName + (f.isCustomFont ? " (web)" : " (installed)")).join(", ")}`);
      }
    }
    const measured = await renderedContrast(cdp, fail);
    tally.boxes += measured;
    if (measured < 5) fail(`only ${measured} text boxes measured`);
    if (pageName === "cockpit") await lethalState(cdp, fail);
    for (const e of exceptions) fail("exception: " + e.split("\n")[0]);
    for (const a of failedAssets) fail("request failed: " + a);
    if (shots) {
      const { data } = await cdp.send("Page.captureScreenshot", { format: "png" });
      writeFileSync(join(shots, `${pageName}-${theme}-${w}.png`), Buffer.from(data, "base64"));
    }
  } finally {
    cdp.listeners.splice(cdp.listeners.indexOf(listener), 1);
    await cdp.send("Page.removeScriptToEvaluateOnNewDocument", { identifier });
  }
}

async function main() {
  const scratch = mkdtempSync(join(tmpdir(), "arcana-browser-check-"));
  let server = null, chrome = null;
  try {
    let base = opt("--base");
    if (!base) {
      const bin = opt("--server");
      if (!bin) throw new Error("pass --server <arcana-web binary> or --base <url>");
      server = await startServer(bin, scratch);
      base = server.base;
    }
    if (shots) mkdirSync(shots, { recursive: true });
    chrome = await startChrome(chromePath(), scratch);
    const cdp = await Cdp.open(chrome.wsUrl);
    for (const domain of ["Page", "Runtime", "Network", "DOM", "CSS"]) await cdp.send(domain + ".enable");
    if (server) await seed(cdp, base);
    const failures = [], notes = [], tally = { boxes: 0 };
    let cases = 0;
    for (const page of PAGES) for (const theme of THEMES) for (const size of WIDTHS) {
      await checkCase(cdp, base, page, theme, size, failures, notes, tally);
      cases++;
    }
    for (const n of notes) console.log("note: " + n);
    if (failures.length) {
      for (const f of failures) console.log("FAIL " + f);
      console.log(`${failures.length} failure(s) in ${cases} cases`);
      process.exitCode = 1;
    } else {
      console.log(`ok: ${cases} cases, ${tally.boxes} text boxes measured, ${notes.length} note(s)`);
    }
  } finally {
    if (chrome) chrome.child.kill("SIGTERM");
    if (server) server.child.kill("SIGTERM");
    await sleep(300);
    rmSync(scratch, { recursive: true, force: true });
  }
}

main().catch((e) => { console.error(e); process.exit(2); });
