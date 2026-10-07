//! The client holds its text to WCAG's 4.5:1 in every theme: on the flat
//! surfaces of `static/theme.css` and on every composited background the
//! stylesheets draw.
//!
//! The three palettes are read from `theme.css` itself: `:root` is the light
//! theme, and `[data-theme="dark"]` and `[data-theme="cappuccino"]` override
//! it, as the cascade does on `<html>`. Tokens that name another
//! (`--lethal: var(--danger)`) resolve within the theme. A text colour added
//! to the client belongs in [`TEXT`] or [`INK_ON_FILL`]; the mana palette is
//! chrome fill, not text.
//!
//! Composited backgrounds (a gradient, a translucent wash or scrim, a glow
//! behind a title) are listed in [`COMPOSITES`], each with how it is held:
//! the strongest point of a glow is computed from its declaration and the
//! gradient sampled down to its base, and a scrim over card art is taken over
//! white, the brightest art there is. Text on the halls' and the champion
//! panel's glows is held to [`GLOW_TEXT`]; the browser check holds the text
//! it renders on a glow to those inks. Every such background or text-shadow in
//! `theme.css` and the four pages must be listed there, so a new one fails
//! the suite until it is modelled. Box-shadows are not modelled: they draw
//! outside an element, around text the stylesheet cannot see; the browser
//! check (`scripts/browser-check.mjs`) measures rendered text instead.

use std::collections::BTreeMap;

const THEME_CSS: &str = include_str!("../static/theme.css");
const FILES: &[(&str, &str)] = &[
    ("theme.css", THEME_CSS),
    ("index.html", include_str!("../static/index.html")),
    ("stage.html", include_str!("../static/stage.html")),
    ("deck.html", include_str!("../static/deck.html")),
    ("decks.html", include_str!("../static/decks.html")),
];

/// Colors the client sets text in, on any of the page surfaces.
const TEXT: &[&str] = &[
    "--text", "--text-muted", "--text-faint", "--link", "--link-hover",
    "--danger", "--lethal", "--energy", "--poison", "--added", "--removed", "--text-on-glow",
];
/// The inks text on a glow may take: the Stage's and My Decks' halls and the
/// champion panel, where --text-faint and --text-muted lose 4.5:1.
const GLOW_TEXT: &[&str] = &["--text", "--text-on-glow", "--link", "--link-hover"];
/// The page surfaces text sits on.
const SURFACES: &[&str] = &["--bg", "--bg-nav", "--bg-offset"];
/// Text on a filled control: inverted buttons and chips, the selection, and
/// the cockpit's target chip and combat badges.
const INK_ON_FILL: &[(&str, &str)] = &[
    ("--bg", "--text"),
    ("--bg", "--text-muted"),
    ("--bg", "--link"),
    ("--selection-text", "--selection-bg"),
    ("--target-ink", "--target"),
    ("--attack-ink", "--attack"),
    ("--block-ink", "--block"),
];
/// State marks drawn as rings, outlines and arrows: 3:1 against every
/// surface, WCAG's floor for the parts of a control.
const MARKS: &[&str] = &["--target", "--attack", "--block"];
/// The colours the Stage's champion panel takes as `--faction`: `--link` by
/// default, else the deck's first colour (`MANA_VAR` in `stage.html`).
const FACTIONS: &[&str] = &[
    "--link", "--mana-w", "--mana-u", "--mana-b", "--mana-r", "--mana-g", "--mana-c",
];

const THEMES: &[&str] = &["light", "dark", "cappuccino"];

type Rgb = [f64; 3];

// ---- the stylesheets -------------------------------------------------------

fn without_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        let end = rest[start + 2..].find("*/").expect("unterminated comment");
        rest = &rest[start + 2 + end + 2..];
    }
    out.push_str(rest);
    out
}

/// A file's CSS: `theme.css` whole, a page's `<style>` blocks.
fn stylesheet(file: &str, text: &str) -> String {
    if !file.ends_with(".html") {
        return without_comments(text);
    }
    let mut css = String::new();
    let mut rest = text;
    while let Some(open) = rest.find("<style") {
        let body = &rest[open..];
        let start = body.find('>').unwrap() + 1;
        let end = body.find("</style>").expect("unclosed <style>");
        css.push_str(&body[start..end]);
        css.push('\n');
        rest = &body[end..];
    }
    without_comments(&css)
}

fn squash(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A style rule with no rule nested in it.
struct Rule {
    selector: String,
    decls: Vec<(String, String)>,
}

impl Rule {
    fn get(&self, property: &str) -> Option<&str> {
        self.decls.iter().find(|(p, _)| p == property).map(|(_, v)| v.as_str())
    }
}

/// Every innermost rule: the text between the last brace and a `{` is its
/// selector, so a rule inside `@media` keeps its own.
fn rules(css: &str) -> Vec<Rule> {
    let mut out = Vec::new();
    // After the last brace; the selector start and `{` of the open rule.
    let (mut after_brace, mut selector_start, mut open) = (0, 0, None);
    for (i, c) in css.char_indices() {
        match c {
            '{' => {
                selector_start = after_brace;
                open = Some(i);
                after_brace = i + 1;
            }
            '}' => {
                if let Some(o) = open.take() {
                    let decls = css[o + 1..i]
                        .split(';')
                        .filter_map(|d| d.split_once(':'))
                        .map(|(p, v)| {
                            let v = squash(&v.replace("!important", ""));
                            (p.trim().to_string(), v)
                        })
                        .collect();
                    out.push(Rule { selector: squash(&css[selector_start..o]), decls });
                }
                after_brace = i + 1;
            }
            _ => {}
        }
    }
    out
}

/// `@keyframes` name → body.
fn keyframes(css: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut rest = css;
    while let Some(at) = rest.find("@keyframes") {
        let after = &rest[at + "@keyframes".len()..];
        let brace = after.find('{').unwrap();
        let name = after[..brace].trim().to_string();
        let mut depth = 0;
        let mut end = brace;
        for (i, c) in after[brace..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = brace + i;
                        break;
                    }
                }
                _ => {}
            }
        }
        out.insert(name, after[brace + 1..end].to_string());
        rest = &after[end..];
    }
    out
}

struct Sheets(BTreeMap<&'static str, Vec<Rule>>);

impl Sheets {
    fn load() -> Self {
        Sheets(FILES.iter().map(|&(f, t)| (f, rules(&stylesheet(f, t)))).collect())
    }
    fn rule(&self, file: &str, selector: &str) -> &Rule {
        self.0[file]
            .iter()
            .find(|r| r.selector == selector)
            .unwrap_or_else(|| panic!("{file} has no rule `{selector}`"))
    }
    /// The colour a rule sets its text in, else its page's `body` colour.
    fn own_color<'a>(&'a self, file: &str, selector: &str) -> &'a str {
        self.rule(file, selector)
            .get("color")
            .or_else(|| self.0[file].iter().find(|r| r.selector == "body").and_then(|r| r.get("color")))
            .unwrap_or("var(--text)")
    }
}

// ---- the palettes ------------------------------------------------------------

fn declarations(css: &str, selector: &str) -> BTreeMap<String, String> {
    rules(css)
        .into_iter()
        .find(|r| r.selector == selector)
        .unwrap_or_else(|| panic!("theme.css has no `{selector}` rule"))
        .decls
        .into_iter()
        .filter(|(p, _)| p.starts_with("--"))
        .collect()
}

struct Theme {
    name: &'static str,
    tokens: BTreeMap<String, String>,
}

impl Theme {
    fn load(name: &'static str) -> Self {
        let css = without_comments(THEME_CSS);
        let mut tokens = declarations(&css, ":root");
        if name != "light" {
            tokens.extend(declarations(&css, &format!("[data-theme=\"{name}\"]")));
        }
        Theme { name, tokens }
    }

    fn value<'a>(&'a self, expr: &'a str) -> &'a str {
        let expr = expr.trim();
        if let Some(inner) = expr.strip_prefix("var(") {
            let target = inner.split([',', ')']).next().unwrap().trim();
            return self.value(target);
        }
        if expr.starts_with("--") {
            let v = self
                .tokens
                .get(expr)
                .unwrap_or_else(|| panic!("{}: {expr} is not defined", self.name));
            return self.value(v);
        }
        expr
    }

    /// A token, `var()`, hex colour, or `color-mix(in srgb, a p%, b)`.
    fn color(&self, expr: &str) -> Rgb {
        let value = self.value(expr);
        if let Some(args) = value.strip_prefix("color-mix(in srgb,").and_then(|a| a.strip_suffix(')')) {
            let (first, second) = split_top_level(args);
            let (a, p) = first.trim().rsplit_once(' ').expect("color-mix needs a percentage");
            return mix(self.color(a), self.color(second), self.fraction(p));
        }
        let hex = value
            .strip_prefix('#')
            .unwrap_or_else(|| panic!("{}: {expr} is `{value}`, not a hex color", self.name));
        let digits: Vec<f64> = match hex.len() {
            3 => hex.chars().map(|c| (c.to_digit(16).unwrap() * 17) as f64).collect(),
            6 => (0..3).map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap() as f64).collect(),
            _ => panic!("{}: {expr} is `{value}`", self.name),
        };
        [digits[0], digits[1], digits[2]]
    }

    /// A token or literal percentage, as a fraction.
    fn fraction(&self, expr: &str) -> f64 {
        let value = self.value(expr);
        let pct = value
            .strip_suffix('%')
            .unwrap_or_else(|| panic!("{}: {expr} is `{value}`, not a percentage", self.name));
        pct.trim().parse::<f64>().unwrap() / 100.0
    }
}

/// `a, b` split at the comma outside any parentheses.
fn split_top_level(args: &str) -> (&str, &str) {
    let mut depth = 0;
    for (i, c) in args.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => return (&args[..i], &args[i + 1..]),
            _ => {}
        }
    }
    panic!("no top-level comma in `{args}`")
}

// ---- colour arithmetic -------------------------------------------------------

/// `color-mix(in srgb, a w, b)`, which is also `a` at alpha `w` over `b`.
fn mix(a: Rgb, b: Rgb, w: f64) -> Rgb {
    [0, 1, 2].map(|i| a[i] * w + b[i] * (1.0 - w))
}

/// WCAG 2.2 relative luminance.
fn luminance(c: Rgb) -> f64 {
    let channel = |c: f64| {
        let c = c / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * channel(c[0]) + 0.7152 * channel(c[1]) + 0.0722 * channel(c[2])
}

fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

const WHITE: Rgb = [255.0; 3];

/// A gradient from `peak` (weight `w` of `color` over `base`) to `base`,
/// sampled: the points between are mixes of the same two colours.
fn glow(color: Rgb, base: Rgb, w: f64) -> Vec<Rgb> {
    (0..=32).map(|k| mix(color, base, w * k as f64 / 32.0)).collect()
}

fn hold(fails: &mut Vec<String>, theme: &Theme, what: &str, fg_name: &str, fg: Rgb, bg: Rgb, floor: f64) {
    let ratio = contrast(fg, bg);
    if ratio < floor {
        fails.push(format!(
            "{}: {fg_name} on {what} is {ratio:.2}:1, under {floor}:1 (background {:?})",
            theme.name,
            bg.map(|c| c.round() as u8)
        ));
    }
}

/// Every text colour on every sampled point of a surface.
fn any_text(t: &Theme, what: &str, surfaces: &[Rgb]) -> Vec<String> {
    inks_on(t, TEXT, what, surfaces)
}

fn inks_on(t: &Theme, inks: &[&str], what: &str, surfaces: &[Rgb]) -> Vec<String> {
    let mut fails = Vec::new();
    for &s in surfaces {
        for &fg in inks {
            hold(&mut fails, t, what, fg, t.color(fg), s, 4.5);
        }
    }
    fails.dedup_by(|a, b| a.split(" (background").next() == b.split(" (background").next());
    fails
}

// ---- the composited backgrounds ----------------------------------------------

/// One background (or text-shadow) that is not a flat token, as it is
/// written, and the check that holds the text drawn on it.
struct Composite {
    file: &'static str,
    selector: &'static str,
    property: &'static str,
    value: &'static str,
    check: fn(&Theme, &Sheets) -> Vec<String>,
}

/// A hall's radial light: `--link` at `w` over `--bg` at its peak.
fn hall(t: &Theme, w: f64) -> Vec<Rgb> {
    glow(t.color("--link"), t.color("--bg"), w)
}

/// The champion panel: a radial glow of the deck's colour at 16% over a
/// wash of it at 8% over `--bg-offset`. Every point is a mix of the two
/// colours, at most 16% + 84% × 8% of the deck's colour at the top left.
const PANEL_PEAK: f64 = 0.16 + 0.84 * 0.08;

/// A large title's halo: its colour on `--link` at `--glow-title` over the
/// brightest point of its hall (`hall_w`), at WCAG's 3:1 for large text (the
/// titles are 24 px and up, semibold).
fn halo(t: &Theme, s: &Sheets, file: &str, selector: &str, hall_w: f64) -> Vec<String> {
    let hall_peak = *hall(t, hall_w).last().unwrap();
    let mut fails = Vec::new();
    let ink = s.own_color(file, selector);
    for bg in glow(t.color("--link"), hall_peak, t.fraction("--glow-title")) {
        hold(&mut fails, t, &format!("{file} {selector}'s halo"), ink, t.color(ink), bg, 3.0);
    }
    fails.truncate(1);
    fails
}

/// The rule's own text on a dark scrim of `alpha` (its weakest under the
/// text) over white, the brightest art a scrim can sit on.
fn scrim(t: &Theme, s: &Sheets, file: &str, selector: &str, rgb: Rgb, alpha: f64) -> Vec<String> {
    let mut fails = Vec::new();
    let ink = s.own_color(file, selector);
    hold(&mut fails, t, &format!("{selector} over white art"), ink, t.color(ink), mix(rgb, WHITE, alpha), 4.5);
    fails
}

/// A backdrop draws no text: its text sits on `panel`, whose background must
/// be one of the flat surfaces.
fn backdrop(s: &Sheets, file: &str, panel: &str) -> Vec<String> {
    let bg = s.rule(file, panel).get("background").unwrap_or("").to_string();
    let flat = SURFACES.iter().any(|t| bg == format!("var({t})"));
    if flat { Vec::new() } else { vec![format!("{file} {panel} is the backdrop's panel but its background is `{bg}`")] }
}

const ZOOM_SCRIM: (Rgb, f64) = ([8.0, 7.0, 6.0], 0.82);

const COMPOSITES: &[Composite] = &[
    // The halls: a radial light of --link atop the Stage and My Decks. Text
    // on them takes a glow ink.
    Composite {
        file: "stage.html", selector: "body", property: "background",
        value: "radial-gradient(1200px 600px at 50% -10%, color-mix(in srgb, var(--link) 12%, transparent), transparent 70%), var(--bg)",
        check: |t, _| inks_on(t, GLOW_TEXT, "the Stage's hall glow", &hall(t, 0.12)),
    },
    Composite {
        file: "decks.html", selector: "body", property: "background",
        value: "radial-gradient(1100px 520px at 50% -10%, color-mix(in srgb, var(--link) 10%, transparent), transparent 70%), var(--bg)",
        check: |t, _| inks_on(t, GLOW_TEXT, "My Decks' hall glow", &hall(t, 0.10)),
    },
    Composite {
        file: "stage.html", selector: ".hall-title h1", property: "text-shadow",
        value: "0 0 24px color-mix(in srgb, var(--link) var(--glow-title), transparent)",
        check: |t, s| halo(t, s, "stage.html", ".hall-title h1", 0.12),
    },
    Composite {
        file: "decks.html", selector: ".hall-title h1", property: "text-shadow",
        value: "0 0 24px color-mix(in srgb, var(--link) var(--glow-title), transparent)",
        check: |t, s| halo(t, s, "decks.html", ".hall-title h1", 0.10),
    },
    // The champion panel: a glow of the deck's colour, whichever it is. Text
    // on it takes a glow ink.
    Composite {
        file: "stage.html", selector: ".throne", property: "background",
        value: "radial-gradient(420px 200px at 12% 0%, color-mix(in srgb, var(--faction) 16%, transparent), transparent 70%), linear-gradient(180deg, color-mix(in srgb, var(--faction) 8%, var(--bg-offset)), var(--bg-offset))",
        check: |t, _| {
            let stage = FILES.iter().find(|f| f.0 == "stage.html").unwrap().1;
            let mut fails: Vec<String> = stage
                .match_indices("\"--mana-")
                .map(|(i, _)| &stage[i + 1..i + 1 + stage[i + 1..].find('"').unwrap()])
                .filter(|v| !FACTIONS.contains(v))
                .map(|v| format!("stage.html names faction colour {v}, which FACTIONS does not list"))
                .collect();
            for f in FACTIONS {
                let surfaces = glow(t.color(f), t.color("--bg-offset"), PANEL_PEAK);
                fails.extend(inks_on(t, GLOW_TEXT, &format!("the champion panel's {f} glow"), &surfaces));
            }
            fails
        },
    },
    // A face-down card in the zone viewer: between two flat surfaces.
    Composite {
        file: "index.html", selector: ".zv-back", property: "background",
        value: "linear-gradient(145deg, var(--bg) 0%, var(--bg-offset) 100%)",
        check: |t, _| {
            let (a, b) = (t.color("--bg"), t.color("--bg-offset"));
            any_text(t, ".zv-back", &(0..=32).map(|k| mix(a, b, k as f64 / 32.0)).collect::<Vec<_>>())
        },
    },
    // The decision prompt carries only its own text, on a wash of it.
    Composite {
        file: "index.html", selector: ".decision-prompt", property: "background",
        value: "color-mix(in srgb, var(--link) 10%, transparent)",
        check: |t, s| {
            let ink = s.own_color("index.html", ".decision-prompt");
            let mut fails = Vec::new();
            for bg in glow(t.color("--link"), t.color("--bg-offset"), 0.10) {
                hold(&mut fails, t, ".decision-prompt's wash", ink, t.color(ink), bg, 4.5);
            }
            fails.truncate(1);
            fails
        },
    },
    // Scrims over card art.
    Composite {
        file: "theme.css", selector: ".ac-pt", property: "background", value: "rgba(12, 10, 8, .72)",
        check: |t, s| scrim(t, s, "theme.css", ".ac-pt", [12.0, 10.0, 8.0], 0.72),
    },
    Composite {
        file: "theme.css", selector: ".ac-badge", property: "background", value: "rgba(12, 10, 8, .78)",
        check: |t, s| scrim(t, s, "theme.css", ".ac-badge", [12.0, 10.0, 8.0], 0.78),
    },
    Composite {
        file: "theme.css", selector: ".ac-name", property: "background",
        value: "linear-gradient(to top, rgba(8, 7, 6, .92), rgba(8, 7, 6, .85) calc(100% - 14px), rgba(8, 7, 6, 0))",
        // .85 from the text's top down; the fade is the 14 px padding above it.
        check: |t, s| scrim(t, s, "theme.css", ".ac-name", [8.0, 7.0, 6.0], 0.85),
    },
    Composite {
        file: "theme.css", selector: ".ac-zoom", property: "background", value: "rgba(8, 7, 6, .82)",
        // The zoom's text is coloured by the `.ac-zoom …` rules.
        check: |t, s| {
            let mut fails = Vec::new();
            let bg = mix(ZOOM_SCRIM.0, WHITE, ZOOM_SCRIM.1);
            let inks: Vec<&Rule> = s.0["theme.css"]
                .iter()
                .filter(|r| r.selector.starts_with(".ac-zoom ") && r.get("color").is_some())
                .collect();
            assert!(inks.len() >= 4, "the zoom's text rules were not found");
            for r in inks {
                let ink = r.get("color").unwrap();
                hold(&mut fails, t, &format!("the zoom scrim ({})", r.selector), ink, t.color(ink), bg, 4.5);
            }
            fails
        },
    },
    Composite {
        file: "theme.css", selector: ".ac-zoom .zoom-chip:hover", property: "background",
        value: "rgba(243, 239, 230, .14)",
        check: |t, s| {
            let ink = s.own_color("theme.css", ".ac-zoom .zoom-chip");
            let bg = mix([243.0, 239.0, 230.0], mix(ZOOM_SCRIM.0, WHITE, ZOOM_SCRIM.1), 0.14);
            let mut fails = Vec::new();
            hold(&mut fails, t, "a hovered zoom chip", ink, t.color(ink), bg, 4.5);
            fails
        },
    },
    // Backdrops behind modals, and a decoration: no text is drawn on them.
    Composite {
        file: "index.html", selector: ".zoneviewer", property: "background", value: "rgba(8, 7, 6, .5)",
        check: |_, s| backdrop(s, "index.html", ".zv-box"),
    },
    Composite {
        file: "index.html", selector: "#search-modal", property: "background",
        value: "color-mix(in srgb, var(--bg) 82%, transparent)",
        check: |_, s| backdrop(s, "index.html", ".search-card"),
    },
    Composite {
        file: "decks.html", selector: ".modal", property: "background",
        value: "color-mix(in srgb, var(--bg) 78%, transparent)",
        check: |_, s| backdrop(s, "decks.html", ".modal-card"),
    },
    Composite {
        file: "deck.html", selector: "#io", property: "background", value: "rgba(8,7,6,.8)",
        check: |_, s| backdrop(s, "deck.html", "#io .iobox"),
    },
    Composite {
        file: "theme.css", selector: ".ac-card.foil .ac-frame::after", property: "background",
        value: "linear-gradient(115deg, transparent 40%, rgba(255, 255, 255, .14) 50%, transparent 60%)",
        // A sheen over the card image, drawn by an empty pseudo-element.
        check: |_, _| Vec::new(),
    },
];

// ---- tests -----------------------------------------------------------------

#[test]
fn every_text_pair_holds_contrast_in_every_theme() {
    let mut failures = Vec::new();
    let mut checked = 0;
    for &name in THEMES {
        let t = Theme::load(name);
        let mut check = |fg: &str, bg: &str, floor: f64| {
            checked += 1;
            hold(&mut failures, &t, bg, fg, t.color(fg), t.color(bg), floor);
        };
        for &bg in SURFACES {
            for &fg in TEXT {
                check(fg, bg, 4.5);
            }
            for &mark in MARKS {
                check(mark, bg, 3.0);
            }
        }
        for &(fg, bg) in INK_ON_FILL {
            check(fg, bg, 4.5);
        }
    }
    assert_eq!(checked, THEMES.len() * (SURFACES.len() * (TEXT.len() + MARKS.len()) + INK_ON_FILL.len()));
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn text_holds_contrast_on_every_composited_background() {
    let sheets = Sheets::load();
    let mut failures = Vec::new();
    for &name in THEMES {
        let t = Theme::load(name);
        for c in COMPOSITES {
            failures.extend((c.check)(&t, &sheets));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Every background or text-shadow that is not a flat token is modelled in
/// [`COMPOSITES`], exactly as written, and every model still matches one.
#[test]
fn every_composited_background_is_modelled() {
    let sheets = Sheets::load();
    let plain = |v: &str| {
        matches!(v, "transparent" | "none" | "inherit" | "initial")
            || (v.starts_with("var(--") && v.ends_with(')') && !v[4..].contains(['(', ',']))
    };
    let mut unmodelled = Vec::new();
    let mut seen = vec![0; COMPOSITES.len()];
    for (&file, rules) in &sheets.0 {
        for r in rules {
            for (p, v) in &r.decls {
                if !matches!(p.as_str(), "background" | "background-color" | "background-image" | "text-shadow")
                    || plain(v)
                {
                    continue;
                }
                match COMPOSITES.iter().position(|c| {
                    c.file == file && c.selector == r.selector && c.property == p && c.value == v
                }) {
                    Some(i) => seen[i] += 1,
                    None => unmodelled.push(format!("{file}: {} {{ {p}: {v} }}", r.selector)),
                }
            }
        }
    }
    let stale: Vec<String> = COMPOSITES
        .iter()
        .zip(&seen)
        .filter(|(_, &n)| n != 1)
        .map(|(c, n)| format!("{}: {} {{ {}: {} }} matched {n} times", c.file, c.selector, c.property, c.value))
        .collect();
    assert!(unmodelled.is_empty(), "model these in COMPOSITES:\n{}", unmodelled.join("\n"));
    assert!(stale.is_empty(), "these COMPOSITES no longer match the stylesheets:\n{}", stale.join("\n"));
}

/// Elements that pulse their opacity indefinitely without drawing text.
const DECORATIVE_PULSES: &[(&str, &str)] = &[("index.html", "#netwait .dot")];

/// No text fades in a sustained animation: one that lowers opacity and runs
/// forever may only animate an empty pseudo-element or a listed decoration.
#[test]
fn no_text_fades_in_a_sustained_animation() {
    let sheets = Sheets::load();
    let fading: Vec<String> = FILES
        .iter()
        .flat_map(|&(f, t)| keyframes(&stylesheet(f, t)))
        .filter(|(_, body)| body.contains("opacity"))
        .map(|(name, _)| name)
        .collect();
    assert!(fading.iter().any(|n| n == "poison-pulse"), "poison-pulse not found: {fading:?}");
    let mut failures = Vec::new();
    let mut pulses = 0;
    for (&file, rules) in &sheets.0 {
        for r in rules {
            let Some(anim) = r.get("animation").or_else(|| r.get("animation-name")) else { continue };
            if !anim.contains("infinite") || !anim.split_whitespace().any(|w| fading.iter().any(|n| n == w)) {
                continue;
            }
            pulses += 1;
            let pseudo = (r.selector.ends_with("::after") || r.selector.ends_with("::before"))
                && r.get("content") == Some("\"\"");
            if !pseudo && !DECORATIVE_PULSES.contains(&(file, r.selector.as_str())) {
                failures.push(format!("{file}: {} fades text with `{anim}`", r.selector));
            }
        }
    }
    assert!(pulses >= 2, "found only {pulses} sustained opacity animations; the scan is not reading the pages");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Lethal poison is the danger colour at full opacity; its pulse is a ring.
#[test]
fn lethal_poison_stays_opaque() {
    let sheets = Sheets::load();
    let count = sheets.rule("index.html", ".phud .poison.lethal");
    assert_eq!(count.get("color"), Some("var(--lethal)"));
    assert_eq!(count.get("opacity"), None);
    assert_eq!(count.get("animation"), None, "the count itself must not animate");
    let ring = sheets.rule("index.html", ".phud .poison.lethal::after");
    assert!(ring.get("animation").is_some_and(|a| a.starts_with("poison-pulse ")));
    assert_eq!(ring.get("content"), Some("\"\""));
}

/// The parser reads three distinct palettes, so the checks above are not
/// comparing the light theme with itself three times.
#[test]
fn the_three_palettes_are_read_apart() {
    let bgs: Vec<Rgb> = THEMES.iter().map(|&t| Theme::load(t).color("--bg")).collect();
    assert_eq!(bgs, [[250.0, 248.0, 244.0], [18.0, 18.0, 18.0], [85.0, 58.0, 40.0]]);
}

/// `--text-on-glow` is a `color-mix()`; the parser mixes it as a browser does.
#[test]
fn color_mix_tokens_resolve() {
    let dark = Theme::load("dark");
    let got = dark.color("--text-on-glow").map(|c| c.round());
    // 56% of #d4d0c8 into #8c8881, channel by channel in sRGB.
    assert_eq!(got, [180.0, 176.0, 169.0]);
    assert_eq!(Theme::load("cappuccino").color("--text-on-glow"), [234.0, 208.0, 160.0]);
}

#[test]
fn contrast_matches_wcag_examples() {
    assert!((contrast([0.0; 3], WHITE) - 21.0).abs() < 1e-9);
    // The site's light --text-faint before and after its 2026-09-06 fix.
    assert!((contrast([136.0; 3], [250.0, 248.0, 244.0]) - 3.34).abs() < 0.01);
    assert!((contrast([110.0, 106.0, 100.0], [250.0, 248.0, 244.0]) - 5.07).abs() < 0.01);
}

#[test]
fn the_rule_parser_keeps_nested_selectors() {
    let css = "@media (x) { .a .b { color: red; } } .c{background: var(--bg) !important}";
    let r = rules(css);
    assert_eq!(r.len(), 2);
    assert_eq!(r[0].selector, ".a .b");
    assert_eq!(r[1].selector, ".c");
    assert_eq!(r[1].get("background"), Some("var(--bg)"));
}
