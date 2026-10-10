//! The client's shared components stay shared (A5.2, by D15). The top bar,
//! panel, modal, meter, buttons, badge, chip, empty state, identity dots and
//! the dialog are defined once, in `theme.css` and `app.js`; a page's own
//! `<style>` keeps its layout and the variants only it needs. These tests read
//! the static sources: a page that calls the browser's `confirm`, `prompt` or
//! `alert`, writes its own top bar, redefines a shared class, or brings back a
//! rule A5.2 removed as dead fails the suite.

use crate::theme_contrast::{rules, stylesheet, FILES};

const APP_JS: &str = include_str!("../static/app.js");
const PAGES: &[&str] = &["index.html", "stage.html", "deck.html", "decks.html"];

fn page(name: &str) -> &'static str {
    FILES.iter().find(|f| f.0 == name).map(|f| f.1).expect("a page in FILES")
}

/// Calls of `confirm(`, `prompt(` or `alert(` that reach the browser's own:
/// bare, or through `window.`. A method of another object (`Arcana.Dialog.
/// confirm(`) and app.js's own method definitions (`confirm(message, …) {`)
/// are not calls of the browser's.
fn browser_dialog_calls(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for (n, line) in text.lines().enumerate() {
        for word in ["confirm(", "prompt(", "alert("] {
            let mut from = 0;
            while let Some(at) = line[from..].find(word) {
                let i = from + at;
                from = i + word.len();
                let before = &line[..i];
                let prev = before.chars().last();
                let window = before.ends_with("window.");
                let bare = !matches!(prev, Some(c) if c.is_alphanumeric() || c == '_' || c == '$' || c == '.');
                let definition = before.trim().is_empty() && line[i..].contains(") {");
                if (bare || window) && !definition {
                    found.push(format!("line {}: {}", n + 1, line.trim()));
                }
            }
        }
    }
    found
}

#[test]
fn no_page_uses_the_browsers_confirm_prompt_or_alert() {
    let mut calls = Vec::new();
    for &name in PAGES {
        calls.extend(browser_dialog_calls(page(name)).into_iter().map(|c| format!("{name} {c}")));
    }
    calls.extend(browser_dialog_calls(APP_JS).into_iter().map(|c| format!("app.js {c}")));
    assert!(calls.is_empty(), "use Arcana.Dialog instead:\n{}", calls.join("\n"));
    // The scan sees a call: it is not passing for want of reading the text.
    assert_eq!(browser_dialog_calls("  if (!confirm(msg)) return;").len(), 1);
    assert_eq!(browser_dialog_calls("x = window.prompt(\"a\")").len(), 1);
    assert!(browser_dialog_calls("await A.Dialog.alert(\"a\")").is_empty());
}

/// The wordmark, the theme slot and the page links are built by app.js
/// (`Arcana.mountTopbar`); each page writes one `<header class="topbar">`
/// holding only its own tools.
#[test]
fn every_page_has_one_top_bar_built_by_app_js() {
    let mut fails = Vec::new();
    for &name in PAGES {
        let html = page(name);
        let headers = html.matches("<header class=\"topbar\"").count();
        if headers != 1 {
            fails.push(format!("{name}: {headers} <header class=\"topbar\">, not one"));
        }
        if html.matches("class=\"topbar\"").count() != headers {
            fails.push(format!("{name}: a top bar that is not the <header>"));
        }
        for own in ["class=\"wordmark\"", "id=\"themeslot\"", "mountThemeSwitcher("] {
            if html.contains(own) {
                fails.push(format!("{name}: writes `{own}`, which app.js builds"));
            }
        }
        if let Some(start) = html.find("<header class=\"topbar\"") {
            let end = start + html[start..].find("</header>").expect("an unclosed top bar");
            if html[start..end].contains("<a ") {
                fails.push(format!("{name}: a link in its top bar; the link set is app.js's NAV"));
            }
        }
    }
    for link in ["[\"/\", \"Stage\"]", "[\"/decks\", \"My Decks\"]", "[\"/deck\", \"Deckbuilder\"]"] {
        if !APP_JS.contains(link) {
            fails.push(format!("app.js's NAV lacks {link}"));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

/// The shared classes a page may extend (`#io .modal-card`, `.deck.panel`) but
/// not redefine as a bare selector.
const SHARED: &[&str] = &[
    ".topbar", ".wordmark", ".topnav", ".btn", ".panel", ".panel-head", ".panel-title", ".panel-body",
    ".modal", ".modal-card", ".modal-head", ".modal-title", ".modal-body", ".modal-foot", ".dialog",
    ".meter", ".meter-fill", ".badge", ".chip", ".kbd", ".empty", ".pips", ".pip",
];

#[test]
fn no_page_redefines_a_shared_component() {
    let mut fails = Vec::new();
    for &name in PAGES {
        for r in rules(&stylesheet(name, page(name))) {
            for sel in r.selector.split(',').map(str::trim) {
                if SHARED.contains(&sel) {
                    fails.push(format!("{name}: `{sel} {{ … }}` redefines a shared component"));
                }
            }
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
    let css = stylesheet("theme.css", FILES[0].1);
    let defined: Vec<String> = rules(&css).into_iter().map(|r| r.selector).collect();
    for class in SHARED {
        assert!(
            defined.iter().any(|s| s.split(',').any(|p| p.trim() == *class || p.trim().starts_with(&format!("{class}["))
                || p.trim().starts_with(&format!("{class}.")) || p.trim().starts_with(&format!("{class} "))
                || p.trim().ends_with(&format!(" > {class}")) || p.trim() == format!("button{class}"))),
            "theme.css defines no rule for {class}"
        );
    }
}

/// Rules and code A5.2 removed because nothing used them.
#[test]
fn the_dead_rules_stay_gone() {
    let dead = [".autopass-ctl", ".ptitle ", ".ptitle{", "buildDeferred", ".deferred-note", ".zoom-chip",
                ".zoom-info", ".zoom-kw", ".zoom-reminder", ".mana-pip"];
    let mut fails = Vec::new();
    for (name, text) in FILES.iter().copied().chain([("app.js", APP_JS)]) {
        for d in dead {
            if text.contains(d) {
                fails.push(format!("{name} contains `{d}`"));
            }
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}
