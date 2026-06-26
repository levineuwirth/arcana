//! Deck import/export (Arena/MTGO list format) and format legality. The
//! [`FormatSpec`] is plain serde data, so built-in formats and user-defined ones
//! are the same thing — a future "custom format" is just another spec. Canonical
//! here in the engine so the deckbuilder UI and RL deck tooling share one rule
//! set.

use serde::{Deserialize, Serialize};

use crate::registry::CardRegistry;
use crate::types::CardId;

/// A parsed/built deck: maindeck + sideboard as `(card id, count)`, the deck
/// name, and any list names that couldn't be resolved against the catalog.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ParsedDeck {
    pub name: String,
    pub main: Vec<(CardId, u32)>,
    pub sideboard: Vec<(CardId, u32)>,
    pub unresolved: Vec<(String, u32)>,
}

fn add_entry(into: &mut Vec<(CardId, u32)>, id: CardId, count: u32) {
    match into.iter_mut().find(|(i, _)| *i == id) {
        Some(e) => e.1 += count,
        None => into.push((id, count)),
    }
}

fn add_unresolved(into: &mut Vec<(String, u32)>, name: String, count: u32) {
    match into.iter_mut().find(|(n, _)| *n == name) {
        Some(e) => e.1 += count,
        None => into.push((name, count)),
    }
}

/// Strip an Arena set/collector annotation: "Opt (M21) 59" / "Opt (M21)" -> "Opt".
fn strip_set_annotation(name: &str) -> String {
    if let Some(idx) = name.rfind(" (") {
        let tail = &name[idx + 2..];
        if let Some(close) = tail.find(')') {
            let code = &tail[..close];
            let after = tail[close + 1..].trim();
            let code_ok = !code.is_empty() && code.len() <= 6
                && code.chars().all(|c| c.is_ascii_alphanumeric());
            let after_ok = after.is_empty() || after.chars().all(|c| c.is_ascii_digit());
            if code_ok && after_ok {
                return name[..idx].trim().to_string();
            }
        }
    }
    name.to_string()
}

/// Parse a "<count> <name>" line (also "4x Name"). Returns `None` for non-card
/// lines (headers, blanks).
fn parse_card_line(line: &str) -> Option<(u32, String)> {
    let (count_str, rest) = line.split_once(char::is_whitespace)?;
    let count: u32 = count_str.trim_end_matches('x').parse().ok()?;
    let name = strip_set_annotation(rest.trim());
    if name.is_empty() { None } else { Some((count, name)) }
}

/// Resolve a list name to a card id, with front/back fallbacks for DFC/split
/// names ("A // B") which the catalog stores under a single face.
fn resolve_name(reg: &CardRegistry, name: &str) -> Option<CardId> {
    if let Some(id) = reg.card_id_by_name(name) {
        return Some(id);
    }
    if let Some((front, back)) = name.split_once(" // ") {
        return reg.card_id_by_name(front.trim()).or_else(|| reg.card_id_by_name(back.trim()));
    }
    None
}

#[derive(Clone, Copy)]
enum Section { About, Main, Sideboard }

/// Parse the Arena/MTGO deck list format. Recognizes the `About` / `Deck` /
/// `Sideboard` headers and `Name <deck name>`; without headers (MTGO style), the
/// first blank line after the maindeck starts the sideboard. Unknown names are
/// collected in [`ParsedDeck::unresolved`] rather than dropped.
pub fn parse_deck_text(text: &str, reg: &CardRegistry) -> ParsedDeck {
    let mut d = ParsedDeck::default();
    let mut section = Section::Main;
    let mut headers_used = false;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            // MTGO: a blank line after the maindeck (and before any explicit
            // header) starts the sideboard.
            if !headers_used && matches!(section, Section::Main) && !d.main.is_empty() {
                section = Section::Sideboard;
            }
            continue;
        }
        match line.to_ascii_lowercase().as_str() {
            "about" => { headers_used = true; section = Section::About; continue; }
            "deck" | "maindeck" | "commander" | "companion" => {
                headers_used = true; section = Section::Main; continue;
            }
            "sideboard" => { headers_used = true; section = Section::Sideboard; continue; }
            _ => {}
        }
        if matches!(section, Section::About) {
            if let Some(rest) = line.strip_prefix("Name ").or_else(|| line.strip_prefix("name ")) {
                d.name = rest.trim().to_string();
            }
            continue;
        }
        if let Some((count, name)) = parse_card_line(line) {
            let target = match section {
                Section::Sideboard => &mut d.sideboard,
                _ => &mut d.main,
            };
            match resolve_name(reg, &name) {
                Some(id) => add_entry(target, id, count),
                None => add_unresolved(&mut d.unresolved, name, count),
            }
        }
    }
    d
}

fn card_name<'a>(reg: &'a CardRegistry, id: CardId) -> &'a str {
    reg.get(id).and_then(|d| reg.interner().resolve(d.name)).unwrap_or("Unknown Card")
}

/// Serialize a deck to the Arena list format (the `About`/`Deck`/`Sideboard`
/// text that imports cleanly back into Arena/MTGO and [`parse_deck_text`]).
pub fn format_deck_text(
    name: &str, main: &[(CardId, u32)], sideboard: &[(CardId, u32)], reg: &CardRegistry,
) -> String {
    let mut s = String::new();
    if !name.is_empty() {
        s.push_str("About\nName ");
        s.push_str(name);
        s.push_str("\n\n");
    }
    s.push_str("Deck\n");
    for (id, count) in main {
        s.push_str(&format!("{count} {}\n", card_name(reg, *id)));
    }
    if !sideboard.is_empty() {
        s.push_str("\nSideboard\n");
        for (id, count) in sideboard {
            s.push_str(&format!("{count} {}\n", card_name(reg, *id)));
        }
    }
    s
}

// =============================================================================
// Formats / legality
// =============================================================================

/// A deck-construction format. Pure data — built-in and user-defined formats are
/// the same shape, so the legality engine never special-cases a format.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FormatSpec {
    pub name: String,
    /// Minimum maindeck size.
    pub min_deck: usize,
    /// Maximum maindeck size (`None` = unbounded).
    pub max_deck: Option<usize>,
    /// Maximum sideboard size.
    pub max_sideboard: usize,
    /// Max copies of a card by name across main + sideboard (basic lands exempt).
    pub max_copies: u32,
    /// Banned card names.
    pub banned: Vec<String>,
}

/// The built-in formats. `Freeform` is permissive (the prototype default);
/// `Constructed` and `Limited` apply the usual structural rules. Real card-pool
/// restrictions (Standard/Modern) need per-card legality data we don't carry yet
/// — that's a future field on [`FormatSpec`].
pub fn builtin_formats() -> Vec<FormatSpec> {
    vec![
        FormatSpec {
            name: "Freeform".into(),
            min_deck: 1, max_deck: None, max_sideboard: usize::MAX,
            max_copies: u32::MAX, banned: Vec::new(),
        },
        FormatSpec {
            name: "Constructed (60)".into(),
            min_deck: 60, max_deck: None, max_sideboard: 15,
            max_copies: 4, banned: Vec::new(),
        },
        FormatSpec {
            name: "Limited (40)".into(),
            min_deck: 40, max_deck: None, max_sideboard: usize::MAX,
            max_copies: u32::MAX, banned: Vec::new(),
        },
    ]
}

/// The verdict of checking a deck against a [`FormatSpec`].
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LegalityReport {
    pub format: String,
    pub legal: bool,
    pub deck_size: u32,
    pub sideboard_size: u32,
    pub violations: Vec<String>,
}

fn is_basic_land(reg: &CardRegistry, id: CardId) -> bool {
    reg.get(id).map_or(false, |d| {
        d.base_characteristics.supertypes.is_basic() && d.base_characteristics.types.is_land()
    })
}

/// Check `main`/`sideboard` against `spec`. Copy limits combine printings by name
/// and exempt basic lands (CR 100.2a). Pure over the registry.
pub fn check_legality(
    main: &[(CardId, u32)], sideboard: &[(CardId, u32)], spec: &FormatSpec, reg: &CardRegistry,
) -> LegalityReport {
    let deck_size: u32 = main.iter().map(|(_, c)| c).sum();
    let sb_size: u32 = sideboard.iter().map(|(_, c)| c).sum();
    let mut violations = Vec::new();

    if (deck_size as usize) < spec.min_deck {
        violations.push(format!("maindeck has {deck_size}, minimum is {}", spec.min_deck));
    }
    if let Some(max) = spec.max_deck {
        if (deck_size as usize) > max {
            violations.push(format!("maindeck has {deck_size}, maximum is {max}"));
        }
    }
    if (sb_size as usize) > spec.max_sideboard {
        violations.push(format!("sideboard has {sb_size}, maximum is {}", spec.max_sideboard));
    }

    // Copy limits + bans: combine main+sideboard counts per card name.
    use std::collections::HashMap;
    let mut by_name: HashMap<&str, (u32, bool)> = HashMap::new(); // name -> (count, is_basic)
    for (id, count) in main.iter().chain(sideboard.iter()) {
        let name = card_name(reg, *id);
        let e = by_name.entry(name).or_insert((0, is_basic_land(reg, *id)));
        e.0 += count;
    }
    let banned: std::collections::HashSet<&str> = spec.banned.iter().map(|s| s.as_str()).collect();
    let mut names: Vec<&str> = by_name.keys().copied().collect();
    names.sort_unstable();
    for name in names {
        let (count, basic) = by_name[name];
        if !basic && count > spec.max_copies {
            violations.push(format!("{count}x {name} — limit is {}", spec.max_copies));
        }
        if banned.contains(name) {
            violations.push(format!("{name} is banned"));
        }
    }

    LegalityReport {
        format: spec.name.clone(),
        legal: violations.is_empty(),
        deck_size,
        sideboard_size: sb_size,
        violations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::Characteristics;
    use crate::registry::CardDefinition;
    use crate::types::{SupertypeSet, TypeLine};

    fn reg() -> CardRegistry {
        let mut r = CardRegistry::new();
        let bolt = Characteristics {
            mana_cost: Some(crate::mana::ManaCost::parse("{R}").unwrap()),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        let n = r.interner_mut().intern("Lightning Bolt");
        r.register(CardDefinition::new(n, bolt));
        // a basic land
        let mut mtn = Characteristics { types: TypeLine::LAND.into(), ..Default::default() };
        mtn.supertypes = SupertypeSet::new().with(SupertypeSet::BASIC);
        let n = r.interner_mut().intern("Mountain");
        r.register(CardDefinition::new(n, mtn));
        r
    }

    #[test]
    fn parse_arena_format() {
        let r = reg();
        let text = "About\nName Burn\n\nDeck\n4 Lightning Bolt\n20 Mountain\n\nSideboard\n2 Lightning Bolt (M21) 59\n1 Nonexistent Card\n";
        let d = parse_deck_text(text, &r);
        assert_eq!(d.name, "Burn");
        assert_eq!(d.main.iter().map(|(_, c)| c).sum::<u32>(), 24);
        assert_eq!(d.sideboard.iter().map(|(_, c)| c).sum::<u32>(), 2, "set annotation stripped + resolved");
        assert_eq!(d.unresolved, vec![("Nonexistent Card".to_string(), 1)]);
    }

    #[test]
    fn parse_mtgo_blankline_sideboard() {
        let r = reg();
        let text = "4 Lightning Bolt\n20 Mountain\n\n3 Lightning Bolt\n";
        let d = parse_deck_text(text, &r);
        assert_eq!(d.main.iter().map(|(_, c)| c).sum::<u32>(), 24);
        assert_eq!(d.sideboard.iter().map(|(_, c)| c).sum::<u32>(), 3);
    }

    #[test]
    fn export_roundtrips() {
        let r = reg();
        let text = "About\nName Burn\n\nDeck\n4 Lightning Bolt\n20 Mountain\n";
        let d = parse_deck_text(text, &r);
        let out = format_deck_text(&d.name, &d.main, &d.sideboard, &r);
        let d2 = parse_deck_text(&out, &r);
        assert_eq!(d2.name, "Burn");
        assert_eq!(d2.main, d.main);
    }

    #[test]
    fn legality_size_copies_basics() {
        let r = reg();
        let bolt = r.card_id_by_name("Lightning Bolt").unwrap();
        let mtn = r.card_id_by_name("Mountain").unwrap();
        let con = builtin_formats().into_iter().find(|f| f.name.starts_with("Constructed")).unwrap();

        // 5x bolt (over the 4 limit) + 20 basics, total 25 (< 60 min)
        let rep = check_legality(&[(bolt, 5), (mtn, 20)], &[], &con, &r);
        assert!(!rep.legal);
        assert!(rep.violations.iter().any(|v| v.contains("minimum is 60")));
        assert!(rep.violations.iter().any(|v| v.contains("Lightning Bolt") && v.contains("limit is 4")));

        // 40 Mountains (basic, exempt) + 20 bolt-legal? still under 60... make it 60.
        let legal = check_legality(&[(bolt, 4), (mtn, 56)], &[], &con, &r);
        assert!(legal.legal, "60 cards, 4 bolts, basics unlimited: {:?}", legal.violations);

        // 30 basics is fine under Freeform
        let free = builtin_formats().into_iter().find(|f| f.name == "Freeform").unwrap();
        assert!(check_legality(&[(mtn, 30)], &[], &free, &r).legal);
    }
}
