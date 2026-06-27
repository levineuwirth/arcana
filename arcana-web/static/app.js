"use strict";
/* =============================================================================
   Arcana — shared frontend module.

   Exposes a single global `Arcana` namespace shared by the game page and the
   deckbuilder:

     Arcana.Theme            theme apply/persist + a mountable switcher
     Arcana.art(name)        Scryfall art URL helper
     Arcana.renderMana(cost) "{2}{R}" -> a fragment of muted mana pips
     Arcana.renderCard(...)  the art-first card tile (board/hand/stack/deck/zoom)
     Arcana.Zoom             the right-click zoom overlay (+ keyword chips)
     Arcana.loadGlossary()   fetch + cache keyword reminder text
     Arcana.Api              thin data-transport client (one place to swap to WS)

   All network access goes through `Arcana.Api` so a future move to a remote /
   WebSocket game is a transport change, not a UI rewrite.
   ========================================================================== */
(function () {
  const Arcana = {};

  /* ---- Theme ------------------------------------------------------------- */
  const THEMES = ["dark", "light", "cappuccino"];
  const THEME_KEY = "arcana.theme";

  function currentTheme() {
    try { return localStorage.getItem(THEME_KEY) || "dark"; } catch (e) { return "dark"; }
  }
  function applyTheme(t) {
    if (THEMES.indexOf(t) < 0) t = "dark";
    document.documentElement.dataset.theme = t;
    try { localStorage.setItem(THEME_KEY, t); } catch (e) { /* ignore */ }
    // reflect the active state in any mounted switchers
    document.querySelectorAll(".theme-switch [data-theme-opt]").forEach((b) => {
      b.classList.toggle("active", b.dataset.themeOpt === t);
    });
  }
  // Apply ASAP so there is no flash (also mirrored by an inline <head> call).
  applyTheme(currentTheme());

  Arcana.Theme = { apply: applyTheme, current: currentTheme, list: THEMES.slice() };

  /** Build a small WB/light/cappuccino segmented switcher into `container`. */
  Arcana.mountThemeSwitcher = function (container) {
    const wrap = document.createElement("div");
    wrap.className = "theme-switch";
    wrap.title = "Theme";
    const LABELS = { dark: "Dark", light: "Light", cappuccino: "Cappuccino" };
    for (const t of THEMES) {
      const b = document.createElement("button");
      b.type = "button";
      b.dataset.themeOpt = t;
      b.textContent = LABELS[t];
      b.classList.toggle("active", t === currentTheme());
      b.onclick = () => applyTheme(t);
      wrap.appendChild(b);
    }
    container.appendChild(wrap);
    return wrap;
  };

  /* ---- Card art ---------------------------------------------------------- */
  // Go through our own /art proxy, which caches images to disk and throttles the
  // Scryfall fetch — so browsing many cards doesn't hit Scryfall's rate limit.
  Arcana.art = function (name) {
    return "/art?name=" + encodeURIComponent(name);
  };
  // The illustration crop (no frame/title) — Scryfall art_crop, cached the same
  // way as the full card.
  Arcana.artCrop = function (name) {
    return "/art?crop=art&name=" + encodeURIComponent(name);
  };

  /* ---- Mana pips --------------------------------------------------------- */
  const COLOR_CLASS = { W: "w", U: "u", B: "b", R: "r", G: "g", C: "c" };

  // Real MTG mana symbols, served by Scryfall (the game-provided symbology):
  // "{2}"->2.svg, "{R}"->R.svg, "{W/U}"->WU.svg, "{G/P}"->GP.svg, "{T}"->T.svg, "{C}"->C.svg.
  Arcana.manaSymbolUrl = function (code) {
    const c = String(code).replace(/[{}]/g, "").replace(/\//g, "").toUpperCase();
    return "https://svgs.scryfall.io/card-symbols/" + encodeURIComponent(c) + ".svg";
  };
  /** A single inline mana-symbol <img> as an HTML string (for innerHTML contexts). */
  Arcana.manaSymbolHtml = function (code) {
    return '<img class="ac-sym" src="' + Arcana.manaSymbolUrl(code) + '" alt="' + code + '" />';
  };
  /** Render a cost string ("{2}{R}{R}") as a fragment of real mana symbols. */
  Arcana.renderMana = function (cost) {
    const frag = document.createDocumentFragment();
    const tokens = (cost || "").match(/\{[^}]+\}/g);
    if (!tokens) return frag;
    for (const tok of tokens) {
      const img = document.createElement("img");
      img.className = "ac-sym";
      img.alt = tok;
      img.src = Arcana.manaSymbolUrl(tok);
      frag.appendChild(img);
    }
    return frag;
  };

  /* ---- Card component ---------------------------------------------------- */
  const DEFAULT_COSMETIC = { artSource: Arcana.art, frameStyle: "default", foil: false };

  /**
   * The shared art-first card tile, used by the board, hand, stack, deckbuilder
   * grid and the zoom. The image is the primary interactive object: `opts.onClick`
   * fires on a click of the art. On-card text is minimal — a subtle cost pip and
   * a P/T overlay; the name shows on hover (and in the zoom).
   *
   * @param {object} data   a CardView or CardInfo (name, mana_cost, type_line,
   *                         power, toughness, keywords, is_land, tapped, ...)
   * @param {object} [opts]  { onClick, classes:[], badge, count, tapped, zoom,
   *                           interactive } display + interaction options
   * @param {object} [cosmetic] future-cosmetics profile { artSource, frameStyle,
   *                         foil } — defaulted; no selection UI is built here.
   */
  Arcana.renderCard = function (data, opts, cosmetic) {
    opts = opts || {};
    const cos = Object.assign({}, DEFAULT_COSMETIC, cosmetic || {});

    const tapped = opts.tapped != null ? opts.tapped : !!data.tapped;
    const card = document.createElement("div");
    card.className = "ac-card" + (tapped ? " tapped" : "");
    if (data.id != null) card.dataset.cardId = data.id; // for arrows / target lookup
    if (cos.foil) card.classList.add("foil");
    if (opts.classes) card.classList.add.apply(card.classList, opts.classes);
    if (opts.onClick) card.classList.add("clickable");

    const frame = document.createElement("div");
    frame.className = "ac-frame";

    const name = data.name && data.name.length ? data.name : "";
    if (name) {
      const img = document.createElement("img");
      img.className = "ac-art";
      img.loading = "lazy";
      img.alt = name;
      img.src = (cos.artSource || Arcana.art)(name);
      img.onerror = () => { img.remove(); frame.appendChild(fallback(name, data)); };
      frame.appendChild(img);
    } else {
      frame.appendChild(fallback("", data));
    }

    // (No on-card cost overlay: the full card image already prints the cost.)

    // P/T overlay (creatures only) — shows the LIVE computed P/T, which can differ
    // from the card image's printed value (counters/pumps), so it's not redundant.
    if (data.power !== null && data.power !== undefined) {
      const pt = document.createElement("div");
      pt.className = "ac-pt";
      pt.textContent = data.power + "/" + data.toughness;
      frame.appendChild(pt);
    }

    // Deckbuilder count.
    if (opts.count) {
      const c = document.createElement("div");
      c.className = "ac-count";
      c.textContent = opts.count;
      frame.appendChild(c);
    }

    // Status badge (combat etc.).
    if (opts.badge) {
      const b = document.createElement("div");
      b.className = "ac-badge";
      b.textContent = opts.badge;
      frame.appendChild(b);
    }

    // Hover name caption.
    if (name) {
      const nm = document.createElement("div");
      nm.className = "ac-name";
      nm.textContent = name;
      frame.appendChild(nm);
    }

    card.appendChild(frame);

    // Tooltip with the fuller detail (the art hides the text).
    const tip = [name || "(hidden card)"];
    if (data.mana_cost) tip.push(data.mana_cost);
    if (data.type_line) tip.push("\n" + data.type_line);
    card.title = tip.join("  ");

    if (opts.onClick) card.onclick = opts.onClick;
    if (opts.zoom !== false && name) {
      card.oncontextmenu = (e) => { e.preventDefault(); Arcana.Zoom.show(data, cos); };
    }
    return card;
  };

  function fallback(name, data) {
    const fb = document.createElement("div");
    fb.className = "ac-fallback";
    const t = document.createElement("div");
    t.textContent = name || "(hidden)";
    fb.appendChild(t);
    if (data && data.type_line) {
      const tl = document.createElement("div");
      tl.style.fontSize = "9px";
      tl.style.opacity = ".7";
      tl.textContent = data.type_line;
      fb.appendChild(tl);
    }
    return fb;
  }

  /* ---- Zoom overlay ------------------------------------------------------ */
  let zoomEl = null;
  /**
   * Render a rich card preview into `container`: large card image, a
   * name·mana·type·P/T line, and EVERY keyword's reminder text (not just on
   * hover). Shared by the zoom modal and the deckbuilder's hover pane. Pass
   * `data = null` to show an empty-state hint.
   */
  Arcana.renderPreview = function (container, data, cosmetic) {
    container.innerHTML = "";
    if (!data) {
      container.classList.add("pv-empty");
      container.textContent = "Hover a card to preview it.";
      return;
    }
    container.classList.remove("pv-empty");
    const cos = Object.assign({}, DEFAULT_COSMETIC, cosmetic || {});

    const imgWrap = document.createElement("div");
    imgWrap.className = "pv-img";
    if (data.name) {
      const img = document.createElement("img");
      img.alt = data.name; img.loading = "lazy";
      img.src = (cos.artSource || Arcana.art)(data.name);
      img.onerror = () => { imgWrap.innerHTML = ""; imgWrap.appendChild(fallback(data.name, data)); };
      imgWrap.appendChild(img);
    } else {
      imgWrap.appendChild(fallback("(hidden card)", data));
    }
    container.appendChild(imgWrap);

    const info = document.createElement("div");
    info.className = "pv-info";
    if (data.name) {
      const nm = document.createElement("div"); nm.className = "pv-name"; nm.textContent = data.name;
      info.appendChild(nm);
    }
    const meta = document.createElement("div"); meta.className = "pv-meta";
    if (data.mana_cost) { const m = document.createElement("span"); m.appendChild(Arcana.renderMana(data.mana_cost)); meta.appendChild(m); }
    if (data.type_line) { const t = document.createElement("span"); t.className = "pv-type"; t.textContent = data.type_line; meta.appendChild(t); }
    if (data.power !== null && data.power !== undefined) {
      const pt = document.createElement("span"); pt.className = "pv-pt tnum"; pt.textContent = data.power + "/" + data.toughness; meta.appendChild(pt);
    }
    if (meta.childNodes.length) info.appendChild(meta);
    container.appendChild(info);

    const kws = data.keywords || [];
    if (kws.length) {
      const box = document.createElement("div"); box.className = "pv-kw";
      for (const kw of kws) {
        const row = document.createElement("div"); row.className = "pv-kw-row";
        const b = document.createElement("b"); b.textContent = kw; row.appendChild(b);
        row.appendChild(document.createTextNode(" — " + (Arcana.glossary[kw] || "(no reminder text)")));
        box.appendChild(row);
      }
      container.appendChild(box);
    }
  };

  function ensureZoom() {
    if (zoomEl) return zoomEl;
    zoomEl = document.createElement("div");
    zoomEl.className = "ac-zoom";
    zoomEl.innerHTML = '<div class="ac-zoom-box"></div>';
    zoomEl.onclick = hideZoom;
    zoomEl.oncontextmenu = (e) => { e.preventDefault(); hideZoom(); };
    document.body.appendChild(zoomEl);
    return zoomEl;
  }
  function showZoom(data, cosmetic) {
    const z = ensureZoom();
    Arcana.renderPreview(z.querySelector(".ac-zoom-box"), data, cosmetic);
    z.classList.add("show");
  }
  function hideZoom() { if (zoomEl) zoomEl.classList.remove("show"); }
  Arcana.Zoom = { show: showZoom, hide: hideZoom };
  document.addEventListener("keydown", (e) => { if (e.key === "Escape") hideZoom(); });

  /* ---- Glossary ---------------------------------------------------------- */
  Arcana.glossary = {};
  Arcana.loadGlossary = async function () {
    try { Arcana.glossary = await Arcana.Api.glossary(); } catch (e) { /* chips still show */ }
    return Arcana.glossary;
  };

  /* ---- Api (thin transport client) -------------------------------------- */
  async function call(method, url, body) {
    const init = { method, headers: {} };
    if (body !== undefined) {
      init.headers["Content-Type"] = "application/json";
      init.body = JSON.stringify(body);
    }
    const r = await fetch(url, init);
    if (!r.ok) {
      let msg = r.statusText || ("HTTP " + r.status);
      try { const e = await r.json(); if (e && e.error) msg = e.error; } catch (x) { /* keep statusText */ }
      const err = new Error(msg);
      err.isApi = true;
      err.status = r.status;
      throw err;
    }
    return r.json();
  }

  /* ---- Networked-match context -------------------------------------------
     When the cockpit is opened for a networked duel it carries (code, seat,
     token) in the URL; we hold them here so the game Api routes to the
     authenticated `/m/*` endpoints instead of the solo ones. A solo game leaves
     this null and everything uses the original routes unchanged. */
  Arcana.Match = {
    ctx: null, // { code, seat, token } | null
    set(ctx) { this.ctx = ctx || null; },
    active() { return !!this.ctx; },
    seat() { return this.ctx ? this.ctx.seat : 0; },
    /// "?code=…&seat=…&token=…" (the /m/* routes take these as query params).
    qs() {
      const c = this.ctx; if (!c) return "";
      return "?code=" + encodeURIComponent(c.code) +
             "&seat=" + c.seat + "&token=" + encodeURIComponent(c.token);
    },
  };
  const M = Arcana.Match;

  Arcana.Api = {
    // game — routed to the per-seat /m/* endpoints during a networked duel.
    state: () => M.active() ? call("GET", "/m/state" + M.qs()) : call("GET", "/state"),
    action: (index) => M.active() ? call("POST", "/m/action" + M.qs(), { index }) : call("POST", "/action", { index }),
    combat: (body) => M.active() ? call("POST", "/m/combat" + M.qs(), body) : call("POST", "/combat", body),
    autotap: (objectId) => M.active() ? call("POST", "/m/autotap" + M.qs(), { object_id: objectId }) : call("POST", "/autotap", { object_id: objectId }),
    activate: (objectId) => M.active() ? call("POST", "/m/activate" + M.qs(), { object_id: objectId }) : call("POST", "/activate", { object_id: objectId }),
    autoPass: (level) => call("POST", "/autopass", { level }),
    warmArt: (names) => call("POST", "/art/warm", { names }),
    warmAll: () => call("POST", "/art/warm-all"),
    warmStatus: () => call("GET", "/art/warm-status"),
    bottom: (ids) => M.active() ? call("POST", "/m/bottom" + M.qs(), { ids }) : call("POST", "/bottom", { ids }),
    suggest: (deep) => M.active()
      ? call("GET", "/m/suggest" + M.qs() + (deep ? "&deep=true" : ""))
      : call("GET", "/suggest" + (deep ? "?deep=true" : "")),
    newGame: (body) => call("POST", "/new", body || {}),
    // networked lobby (host / join / poll)
    lobbyCreate: (body) => call("POST", "/lobby/create", body),
    lobbyJoin: (body) => call("POST", "/lobby/join", body),
    lobbyInfo: (code) => call("GET", "/lobby/info?code=" + encodeURIComponent(code)),
    // catalog / deckbuilder
    search: (query) => call("POST", "/search", query),
    formats: () => call("GET", "/formats"),
    import: (text) => call("POST", "/import", { text }),
    legality: (body) => call("POST", "/legality", body),
    sampleDeck: () => call("GET", "/sample-deck"),
    glossary: () => call("GET", "/glossary"),
    // World Stage: derive a decklist's faction identity, and the rival roster.
    deckIdentity: (deck, name) => call("POST", "/deck-identity", { deck, name }),
    personalities: () => call("GET", "/personalities"),
  };

  /* ---- Decks: a versioned, lineage-aware deck store (localStorage) --------
     Shared by the deckbuilder, the Stage, and the My Decks page. Each deck has
     a stable id, a lineage (parent + relation: native|fork|variation), and a
     full version HISTORY — every save appends a commit you can diff/restore.
     Deck content is `[{info, count}]` per zone (full CardInfo, so any surface
     can render without a round-trip). One-time migration upgrades the old
     name-keyed map (and folds in arcana.identities overrides). */
  const DECKS_KEY = "arcana.decks";
  const genId = () =>
    (crypto.randomUUID && crypto.randomUUID()) ||
    (Date.now().toString(36) + Math.random().toString(36).slice(2));
  const nowTs = () => Date.now();

  function readStore() {
    let raw;
    try { raw = JSON.parse(localStorage.getItem(DECKS_KEY) || "{}"); } catch { raw = {}; }
    if (raw && raw.version === 2) return raw;
    // --- migrate the old `{ name: {main,side} }` (or `{name:[...]}`) map ---
    let overrides = {};
    try { overrides = JSON.parse(localStorage.getItem("arcana.identities") || "{}"); } catch {}
    const store = { version: 2, decks: {} };
    for (const [name, d] of Object.entries(raw || {})) {
      if (!d) continue;
      const main = Array.isArray(d) ? d : (d.main || []);
      const side = Array.isArray(d) ? [] : (d.side || d.sideboard || []);
      if (!Array.isArray(main)) continue;
      const id = genId(), vid = genId();
      store.decks[id] = {
        id, name: String(name), parent: null, relation: "native", created: nowTs(),
        identity: overrides[name] || null, head: vid,
        versions: [{ vid, ts: nowTs(), label: "imported", main, side }],
      };
    }
    writeStore(store);
    return store;
  }
  function writeStore(store) {
    try { localStorage.setItem(DECKS_KEY, JSON.stringify(store)); } catch {}
  }
  function headVersion(deck) {
    return (deck && deck.versions.find((v) => v.vid === deck.head)) ||
           (deck && deck.versions[deck.versions.length - 1]) || null;
  }
  // Normalize content to ids+counts (drops the CardInfo) — for the engine/Stage.
  function toIds(content) {
    const out = [];
    for (const e of content || []) {
      const id = e.info ? e.info.id : e.id;
      for (let i = 0; i < (e.count || 0); i++) out.push(id);
    }
    return out;
  }

  const Decks = {
    KEY: DECKS_KEY,
    all() { return readStore().decks; },
    list() { return Object.values(readStore().decks); },
    get(id) { return readStore().decks[id] || null; },
    /** Head version's `{ main, side }` content (full-info entries). */
    content(id) {
      const v = headVersion(this.get(id));
      return v ? { main: v.main || [], side: v.side || [] } : { main: [], side: [] };
    },
    /** Head content as engine ids `{ main:[id…], side:[id…] }`. */
    ids(id) { const c = this.content(id); return { main: toIds(c.main), side: toIds(c.side) }; },
    create(name, main, side, relation, parent) {
      const store = readStore();
      const id = genId(), vid = genId();
      store.decks[id] = {
        id, name: name || "New deck", parent: parent || null,
        relation: relation || "native", created: nowTs(), identity: null, head: vid,
        versions: [{ vid, ts: nowTs(), label: "created", main: main || [], side: side || [] }],
      };
      writeStore(store);
      return id;
    },
    /** Append a new version (a commit) and make it head. */
    save(id, main, side, label) {
      const store = readStore();
      const d = store.decks[id];
      if (!d) return null;
      const vid = genId();
      d.versions.push({ vid, ts: nowTs(), label: label || "edit", main: main || [], side: side || [] });
      d.head = vid;
      writeStore(store);
      return vid;
    },
    rename(id, name) {
      const store = readStore(); const d = store.decks[id];
      if (d) { d.name = name; writeStore(store); }
    },
    setIdentity(id, identity) {
      const store = readStore(); const d = store.decks[id];
      if (d) { d.identity = identity; writeStore(store); }
    },
    /** Fork (new line) / vary (clustered tweak): a child copying the head. */
    fork(id, name, relation) {
      const c = this.content(id);
      const src = this.get(id);
      const nm = name || ((src ? src.name : "Deck") + (relation === "variation" ? " (var)" : " (fork)"));
      return this.create(nm, c.main, c.side, relation || "fork", id);
    },
    /** Independent copy with NO lineage. */
    duplicate(id, name) {
      const c = this.content(id);
      const src = this.get(id);
      return this.create(name || ((src ? src.name : "Deck") + " copy"), c.main, c.side, "native", null);
    },
    remove(id) {
      const store = readStore();
      // Re-parent children to this deck's parent so a family isn't orphaned.
      const gone = store.decks[id];
      if (!gone) return;
      for (const d of Object.values(store.decks)) if (d.parent === id) d.parent = gone.parent;
      delete store.decks[id];
      writeStore(store);
    },
    history(id) { const d = this.get(id); return d ? d.versions.slice() : []; },
    /** Restore an old version by appending a copy of it as the new head. */
    restore(id, vid) {
      const d = this.get(id);
      const v = d && d.versions.find((x) => x.vid === vid);
      if (!v) return null;
      return this.save(id, v.main, v.side, "restore");
    },
    /** A deck plus all its descendants (for family grouping on the page). */
    family(id) {
      const all = this.all(); const out = [];
      const walk = (pid) => { for (const d of Object.values(all)) if (d.parent === pid) { out.push(d); walk(d.id); } };
      const root = all[id]; if (root) { out.push(root); walk(id); }
      return out;
    },
    /** Delta from content A to content B (per the card id), for compare views. */
    diff(aContent, bContent) {
      const idx = (c) => { const m = new Map(); for (const e of c || []) { const id = e.info ? e.info.id : e.id; m.set(id, { info: e.info, count: (m.get(id)?.count || 0) + e.count }); } return m; };
      const A = idx(aContent), B = idx(bContent);
      const added = [], removed = [], changed = [];
      for (const [id, b] of B) { const a = A.get(id);
        if (!a) added.push({ info: b.info, count: b.count });
        else if (a.count !== b.count) changed.push({ info: b.info, from: a.count, to: b.count }); }
      for (const [id, a] of A) if (!B.has(id)) removed.push({ info: a.info, count: a.count });
      return { added, removed, changed };
    },
  };
  Arcana.Decks = Decks;

  window.Arcana = Arcana;
})();
