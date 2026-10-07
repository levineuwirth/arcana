//! The web client's vendored assets, embedded so the binary stays
//! self-contained, and served under `/assets/`.
//!
//! Everything here is written by `arcana-web/scripts/vendor-assets.py` into
//! `static/assets/`: WOFF2 subsets of Spectral, Fira Sans and JetBrains Mono,
//! the Mana symbol font and its stylesheet, and each font's licence. The table
//! lists every file once; a test holds it to the directory, so a file the
//! script adds and the table misses fails the suite rather than a browser.

/// One embedded file: its path under `/assets/`, its bytes and its media type.
pub struct Asset {
    pub path: &'static str,
    pub bytes: &'static [u8],
    pub content_type: &'static str,
}

macro_rules! asset {
    ($path:literal, $content_type:expr) => {
        Asset {
            path: $path,
            bytes: include_bytes!(concat!("../static/assets/", $path)),
            content_type: $content_type,
        }
    };
}

const WOFF2: &str = "font/woff2";
const TEXT: &str = "text/plain; charset=utf-8";
const CSS: &str = "text/css; charset=utf-8";

pub static ASSETS: &[Asset] = &[
    asset!("fonts/Spectral-Regular.woff2", WOFF2),
    asset!("fonts/Spectral-Italic.woff2", WOFF2),
    asset!("fonts/Spectral-Medium.woff2", WOFF2),
    asset!("fonts/Spectral-SemiBold.woff2", WOFF2),
    asset!("fonts/Spectral-SemiBoldItalic.woff2", WOFF2),
    asset!("fonts/Spectral-Bold.woff2", WOFF2),
    asset!("fonts/FiraSans-Regular.woff2", WOFF2),
    asset!("fonts/FiraSans-Medium.woff2", WOFF2),
    asset!("fonts/FiraSans-SemiBold.woff2", WOFF2),
    asset!("fonts/JetBrainsMono-Regular.woff2", WOFF2),
    asset!("fonts/mana.woff2", WOFF2),
    asset!("fonts/OFL-Spectral.txt", TEXT),
    asset!("fonts/OFL-FiraSans.txt", TEXT),
    asset!("fonts/OFL-JetBrainsMono.txt", TEXT),
    asset!("fonts/OFL-Mana.txt", TEXT),
    asset!("mana.css", CSS),
];

/// The asset at `path` (relative to `/assets/`), if there is one.
pub fn get(path: &str) -> Option<&'static Asset> {
    ASSETS.iter().find(|a| a.path == path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::Path;

    fn walk(dir: &Path, root: &Path, out: &mut BTreeSet<String>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, root, out);
            } else {
                out.insert(path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/"));
            }
        }
    }

    /// The table and `static/assets/` hold the same files: a file the
    /// vendoring script writes and the table misses would 404 in a browser.
    #[test]
    fn the_table_lists_every_vendored_file() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("static/assets");
        let mut on_disk = BTreeSet::new();
        walk(&root, &root, &mut on_disk);
        let listed: BTreeSet<String> = ASSETS.iter().map(|a| a.path.to_string()).collect();
        assert_eq!(listed.len(), ASSETS.len(), "a path is listed twice");
        assert_eq!(listed, on_disk);
    }

    #[test]
    fn media_types_follow_the_extension_and_fonts_are_woff2() {
        for a in ASSETS {
            let want = match a.path.rsplit('.').next() {
                Some("woff2") => WOFF2,
                Some("txt") => TEXT,
                Some("css") => CSS,
                other => panic!("{}: no media type for {other:?}", a.path),
            };
            assert_eq!(a.content_type, want, "{}", a.path);
            if want == WOFF2 {
                assert!(a.bytes.starts_with(b"wOF2"), "{} is not a WOFF2 file", a.path);
            }
        }
    }

    /// Every `/assets/…` URL the client names resolves: the `@font-face`
    /// rules in `theme.css` and `mana.css`, and the pages' stylesheet links.
    #[test]
    fn every_asset_the_client_names_is_served() {
        let sources = [
            ("theme.css", include_str!("../static/theme.css")),
            ("index.html", include_str!("../static/index.html")),
            ("stage.html", include_str!("../static/stage.html")),
            ("deck.html", include_str!("../static/deck.html")),
            ("decks.html", include_str!("../static/decks.html")),
            ("app.js", include_str!("../static/app.js")),
            ("assets/mana.css", include_str!("../static/assets/mana.css")),
        ];
        let mut named = 0;
        for (file, text) in sources {
            for (i, _) in text.match_indices("/assets/") {
                let rest = &text[i + "/assets/".len()..];
                let end = rest
                    .find(|c: char| !(c.is_ascii_alphanumeric() || "._-/".contains(c)))
                    .unwrap_or(rest.len());
                let path = &rest[..end];
                if path.is_empty() || path.ends_with('/') {
                    continue; // a directory named in prose
                }
                assert!(get(path).is_some(), "{file} names /assets/{path}, which is not served");
                named += 1;
            }
        }
        // Ten faces in theme.css, Mana's in mana.css, and four page links.
        assert!(named >= 15, "found only {named} asset URLs; the scan is not reading the client");
    }
}
