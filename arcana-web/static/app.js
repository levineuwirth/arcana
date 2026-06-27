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
    if (data.mana_cost) { const m = document.createElement("span"); m.innerHTML = Arcana.renderMana(data.mana_cost); meta.appendChild(m); }
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

  Arcana.Api = {
    // game
    state: () => call("GET", "/state"),
    action: (index) => call("POST", "/action", { index }),
    combat: (body) => call("POST", "/combat", body),
    autotap: (objectId) => call("POST", "/autotap", { object_id: objectId }),
    activate: (objectId) => call("POST", "/activate", { object_id: objectId }),
    autoPass: (level) => call("POST", "/autopass", { level }),
    warmArt: (names) => call("POST", "/art/warm", { names }),
    bottom: (ids) => call("POST", "/bottom", { ids }),
    suggest: (deep) => call("GET", "/suggest" + (deep ? "?deep=true" : "")),
    newGame: (body) => call("POST", "/new", body || {}),
    // catalog / deckbuilder
    search: (query) => call("POST", "/search", query),
    formats: () => call("GET", "/formats"),
    import: (text) => call("POST", "/import", { text }),
    legality: (body) => call("POST", "/legality", body),
    sampleDeck: () => call("GET", "/sample-deck"),
    glossary: () => call("GET", "/glossary"),
  };

  window.Arcana = Arcana;
})();
