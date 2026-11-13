//! CI GATE — activated-ability timing (A1.5s). An ability its text does not
//! restrict can be activated whenever its controller has priority (CR 602.2),
//! so an ability is gated to sorcery timing (`is_instant_speed: false`)
//! exactly when its text says so: "only as a sorcery", "could cast a
//! sorcery", or a sorcery-speed keyword (equip, level up, a Class level...).
//! FAILS on a gated ability whose text names no such restriction unless
//! `timing_allowlist.txt` lists it with a reason; on an ungated ability
//! whose text names one; and on an allowlist line that matches nothing.
//! Mana abilities are never gated and loyalty abilities always are, whatever
//! the flag says, so neither is checked.

use std::collections::BTreeSet;

const SORCERY_KEYWORDS: &[&str] = &[
    "equip", "fortify", "level up", "outlast", "reconfigure", "scavenge", "embalm",
    "eternalize", "unearth", "encore", "craft with", "transmute", "transfigure",
    "saddle", "station", "specialize",
];

/// Does `text` restrict its ability to sorcery timing?
fn says_sorcery(text: &str) -> bool {
    let t = text.trim().to_lowercase();
    if t.contains("only as a sorcery") || t.contains("could cast a sorcery") {
        return true;
    }
    if SORCERY_KEYWORDS.iter().any(|k| t.strip_prefix(k).is_some_and(|r| !r.starts_with(char::is_alphabetic))) {
        return true;
    }
    // A Class level: "{1}{R}: Level 2."
    let mut rest = t.as_str();
    while let Some(r) = rest.strip_prefix('{') {
        match r.find('}') { Some(i) => rest = &r[i + 1..], None => return false }
    }
    rest != t && rest.starts_with(": level ")
}

#[test]
fn activated_ability_timing_matches_text() {
    let allow: BTreeSet<(String, String)> = include_str!("timing_allowlist.txt")
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let mut f = l.split('\t');
            let (card, text) = (f.next().unwrap_or(""), f.next().unwrap_or(""));
            assert!(f.next().is_some_and(|r| !r.trim().is_empty()), "allowlist line without a reason: {l}");
            (card.to_string(), text.to_string())
        })
        .collect();
    let reg = arcana_cards::build_catalog();
    let (mut unstated, mut contradicted, mut seen) = (Vec::new(), Vec::new(), BTreeSet::new());
    for cid in 0..reg.len() as u32 {
        let Some(def) = reg.get(cid) else { continue };
        let card = reg.interner().resolve(def.name).unwrap_or("?").to_string();
        for a in &def.activated_abilities {
            if a.is_mana_ability || a.is_loyalty_ability { continue; }
            let key = (card.clone(), a.text.clone());
            match (a.is_instant_speed, says_sorcery(&a.text)) {
                (false, false) if allow.contains(&key) => { seen.insert(key); }
                (false, false) => unstated.push(format!("{}\t{}", key.0, key.1)),
                (true, true) => contradicted.push(format!("{}\t{}", key.0, key.1)),
                _ => {}
            }
        }
    }
    let stale: Vec<_> = allow.difference(&seen).collect();
    assert!(unstated.is_empty(),
        "{} abilities gated to sorcery timing whose text names no restriction; set \
         `is_instant_speed: true`, or allowlist with a reason:\n{}", unstated.len(), unstated.join("\n"));
    assert!(contradicted.is_empty(),
        "{} abilities whose text restricts them to sorcery timing but are flagged instant:\n{}",
        contradicted.len(), contradicted.join("\n"));
    assert!(stale.is_empty(), "allowlist lines matching no gated ability (remove them): {stale:?}");
}
