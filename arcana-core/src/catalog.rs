//! Game-state-INDEPENDENT card-knowledge layer over the catalog. Where
//! [`crate::view::CardView`] projects a live *object* (a card on the battlefield
//! or in a hand), [`CardInfo`] projects a card *type* straight from its
//! [`CardDefinition`] — the substrate the deckbuilder UI and RL deck tooling both
//! read. [`query`] filters the whole registry by the usual deckbuilding axes
//! (name, color, type, keyword, mana value).

use serde::{Deserialize, Serialize};

use crate::objects::Characteristics;
use crate::registry::{CardDefinition, CardRegistry};
use crate::types::{CardId, TypeLine};

/// A serializable projection of a card *type* (independent of any game state).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CardInfo {
    pub id: CardId,
    pub name: String,
    /// Rendered mana cost ("{2}{R}"); `None` for lands/tokens.
    pub mana_cost: Option<String>,
    pub mana_value: u32,
    /// Color letters, sorted/deduped (['R'], ['G','W']); empty = colorless.
    pub colors: Vec<char>,
    /// "Legendary Creature — Elf Warrior", "Basic Land — Forest", "Instant".
    pub type_line: String,
    /// Printed keyword abilities by glossary key ("Flying", "Ward", …).
    pub keywords: Vec<String>,
    /// Printed power/toughness when fixed (`None` for `*`/CDA values).
    pub power: Option<i32>,
    pub toughness: Option<i32>,
    pub loyalty: Option<i32>,
    pub is_land: bool,
    pub is_creature: bool,
    pub is_instant: bool,
    pub is_sorcery: bool,
    pub is_artifact: bool,
    pub is_enchantment: bool,
    pub is_planeswalker: bool,
    pub is_battle: bool,
}

impl CardInfo {
    /// Does this card have the named card-type category ("creature", "land", …)?
    fn has_type(&self, t: &str) -> bool {
        match t.to_ascii_lowercase().as_str() {
            "land" => self.is_land,
            "creature" => self.is_creature,
            "instant" => self.is_instant,
            "sorcery" => self.is_sorcery,
            "artifact" => self.is_artifact,
            "enchantment" => self.is_enchantment,
            "planeswalker" => self.is_planeswalker,
            "battle" => self.is_battle,
            _ => false,
        }
    }
}

/// Build the printed type line: "{supertypes} {types} — {subtypes}" (the em-dash
/// + subtype clause only when subtypes exist). Subtypes are interner-resolved and
/// sorted for determinism (the set is unordered). Shared by [`CardInfo`] and
/// [`crate::view::CardView`] so the in-game and catalog views agree.
pub fn card_type_line(c: &Characteristics, registry: &CardRegistry) -> String {
    let mut head: Vec<&str> = Vec::new();
    let s = &c.supertypes;
    if s.is_basic() { head.push("Basic"); }
    if s.is_legendary() { head.push("Legendary"); }
    if s.is_snow() { head.push("Snow"); }
    if s.is_world() { head.push("World"); }
    let t = &c.types;
    if t.is_artifact() { head.push("Artifact"); }
    if t.is_battle() { head.push("Battle"); }
    if t.is_creature() { head.push("Creature"); }
    if t.is_enchantment() { head.push("Enchantment"); }
    if t.is_instant() { head.push("Instant"); }
    if t.has(TypeLine::KINDRED) { head.push("Kindred"); }
    if t.is_land() { head.push("Land"); }
    if t.is_planeswalker() { head.push("Planeswalker"); }
    if t.is_sorcery() { head.push("Sorcery"); }

    let mut subs: Vec<&str> = c.subtypes.iter()
        .filter_map(|sm| registry.interner().resolve(sm))
        .collect();
    subs.sort_unstable();

    let head = head.join(" ");
    if subs.is_empty() { head } else { format!("{head} — {}", subs.join(" ")) }
}

fn project(registry: &CardRegistry, id: CardId, def: &CardDefinition) -> CardInfo {
    let c = &def.base_characteristics;
    let name = registry.interner().resolve(def.name).unwrap_or_default().to_string();
    let mut colors: Vec<char> = c.colors.iter().map(|col| col.letter()).collect();
    colors.sort_unstable();
    colors.dedup();
    let mut keywords: Vec<String> = c.keywords.iter().map(crate::glossary::keyword_key).collect();
    keywords.sort_unstable();
    keywords.dedup();
    let t = &c.types;
    CardInfo {
        id,
        name,
        mana_cost: c.mana_cost.as_ref().map(|m| m.to_string()),
        mana_value: c.mana_value(),
        colors,
        type_line: card_type_line(c, registry),
        keywords,
        power: c.power.as_ref().and_then(|p| p.resolve(None)),
        toughness: c.toughness.as_ref().and_then(|p| p.resolve(None)),
        loyalty: c.loyalty,
        is_land: t.is_land(),
        is_creature: t.is_creature(),
        is_instant: t.is_instant(),
        is_sorcery: t.is_sorcery(),
        is_artifact: t.is_artifact(),
        is_enchantment: t.is_enchantment(),
        is_planeswalker: t.is_planeswalker(),
        is_battle: t.is_battle(),
    }
}

/// Project a single card type, or `None` if `id` isn't registered.
pub fn card_info(registry: &CardRegistry, id: CardId) -> Option<CardInfo> {
    registry.get(id).map(|def| project(registry, id, def))
}

/// Project a card type by name (exact, case-sensitive — the catalog's keys).
pub fn card_info_by_name(registry: &CardRegistry, name: &str) -> Option<CardInfo> {
    registry.card_id_by_name(name).and_then(|id| card_info(registry, id))
}

/// A catalog filter. Every field is optional; an absent field doesn't constrain.
/// Within a field, semantics are: `colors`/`types` = match ANY listed; `keywords`
/// = match ALL listed; `name` = case-insensitive substring.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CardQuery {
    pub name: Option<String>,
    pub colors: Option<Vec<char>>,
    /// If `Some(true)`, restrict to colorless cards.
    pub colorless: Option<bool>,
    pub types: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub cmc_min: Option<u32>,
    pub cmc_max: Option<u32>,
    /// Cap the number of results (`None` = unbounded; callers serving a UI should
    /// set a sane limit — the catalog has ~20k cards).
    pub limit: Option<usize>,
}

impl CardQuery {
    fn matches(&self, ci: &CardInfo) -> bool {
        if let Some(n) = &self.name {
            if !ci.name.to_lowercase().contains(&n.to_lowercase()) { return false; }
        }
        if self.colorless == Some(true) && !ci.colors.is_empty() {
            return false;
        }
        if let Some(cs) = &self.colors {
            if !cs.is_empty()
                && !cs.iter().any(|c| ci.colors.contains(&c.to_ascii_uppercase())) {
                return false;
            }
        }
        if let Some(ts) = &self.types {
            if !ts.is_empty() && !ts.iter().any(|t| ci.has_type(t)) { return false; }
        }
        if let Some(kws) = &self.keywords {
            if !kws.iter().all(|k| ci.keywords.iter().any(|ck| ck.eq_ignore_ascii_case(k))) {
                return false;
            }
        }
        if let Some(lo) = self.cmc_min { if ci.mana_value < lo { return false; } }
        if let Some(hi) = self.cmc_max { if ci.mana_value > hi { return false; } }
        true
    }
}

/// Filter the whole catalog by `q`, sorted by mana value then name (stable
/// deckbuilder ordering), truncated to `q.limit` if set.
pub fn query(registry: &CardRegistry, q: &CardQuery) -> Vec<CardInfo> {
    let mut out: Vec<CardInfo> = registry.iter()
        .map(|(id, def)| project(registry, id, def))
        .filter(|ci| q.matches(ci))
        .collect();
    out.sort_by(|a, b| a.mana_value.cmp(&b.mana_value).then_with(|| a.name.cmp(&b.name)));
    if let Some(lim) = q.limit { out.truncate(lim); }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effects::KeywordAbility;
    use crate::mana::ManaCost;
    use crate::objects::Characteristics;
    use crate::types::{ColorSet, PtValue};

    fn reg_with_samples() -> CardRegistry {
        let mut reg = CardRegistry::new();
        // a red flyer {2}{R}, 2/2
        let bird = Characteristics {
            mana_cost: Some(ManaCost::parse("{2}{R}").unwrap()),
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        };
        let n = reg.interner_mut().intern("Test Bird");
        reg.register(CardDefinition::new(n, bird));
        // a colorless land (mv 0)
        let land = Characteristics { types: TypeLine::LAND.into(), ..Default::default() };
        let n = reg.interner_mut().intern("Test Wood");
        reg.register(CardDefinition::new(n, land));
        // a blue instant {U}
        let bounce = Characteristics {
            mana_cost: Some(ManaCost::parse("{U}").unwrap()),
            colors: ColorSet::blue(),
            types: TypeLine::INSTANT.into(),
            ..Default::default()
        };
        let n = reg.interner_mut().intern("Test Bounce");
        reg.register(CardDefinition::new(n, bounce));
        reg
    }

    #[test]
    fn card_info_projects_fields() {
        let reg = reg_with_samples();
        let bird = card_info_by_name(&reg, "Test Bird").expect("registered");
        assert_eq!(bird.mana_value, 3);
        assert_eq!(bird.colors, vec!['R']);
        assert_eq!(bird.mana_cost.as_deref(), Some("{2}{R}"));
        assert!(bird.is_creature && !bird.is_land);
        assert_eq!(bird.power, Some(2));
        assert!(bird.keywords.contains(&"Flying".to_string()));
        assert!(bird.type_line.contains("Creature"));
        assert!(card_info_by_name(&reg, "Nonexistent").is_none());
    }

    #[test]
    fn query_filters_and_orders() {
        let reg = reg_with_samples();
        assert_eq!(query(&reg, &CardQuery::default()).len(), 3);

        let creatures = query(&reg, &CardQuery {
            types: Some(vec!["creature".into()]), ..Default::default() });
        assert_eq!(creatures.len(), 1);
        assert_eq!(creatures[0].name, "Test Bird");

        let red = query(&reg, &CardQuery { colors: Some(vec!['r']), ..Default::default() });
        assert_eq!(red.len(), 1, "color match is case-insensitive");

        let colorless = query(&reg, &CardQuery { colorless: Some(true), ..Default::default() });
        assert!(colorless.iter().any(|c| c.name == "Test Wood"));
        assert!(colorless.iter().all(|c| c.colors.is_empty()));

        let cheap = query(&reg, &CardQuery { cmc_max: Some(1), ..Default::default() });
        assert!(cheap.iter().all(|c| c.mana_value <= 1));

        let flyers = query(&reg, &CardQuery {
            keywords: Some(vec!["flying".into()]), ..Default::default() });
        assert_eq!(flyers.len(), 1);

        // sorted by mana value then name: land(0), instant(1), bird(3)
        let all = query(&reg, &CardQuery::default());
        assert_eq!(all.iter().map(|c| c.mana_value).collect::<Vec<_>>(), vec![0, 1, 3]);

        assert_eq!(query(&reg, &CardQuery { limit: Some(2), ..Default::default() }).len(), 2);
    }
}
