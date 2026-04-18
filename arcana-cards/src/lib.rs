//! Card catalog for the Arcana engine.
//!
//! This crate is a thin catalog: every card is a `pub fn register(reg:
//! &mut CardRegistry) -> CardId` that constructs and registers a
//! [`arcana_core::registry::CardDefinition`]. The engine lives entirely
//! in [`arcana_core`]; this crate owns no types of its own.
//!
//! # Organization
//!
//! Modules are named by **set code** (Scryfall's three-letter codes:
//! `lea` = Limited Edition Alpha, `lrw` = Lorwyn, etc.). Each card
//! lives in its **canonical-printing** set — the earliest set that
//! printed the card, matching Scryfall's default scheme. Reprints do
//! not duplicate; `arcana-gen` will emit one module per canonical
//! printing and handle reprints via card-id aliasing.
//!
//! Example: Lightning Bolt was printed in LEA, so it lives at
//! [`lea::lightning_bolt`]. When Bolt reprints in M11, M12, or any
//! other set, the Scryfall id is aliased to the same `CardId` the
//! LEA module registered.
//!
//! # Why set-code
//!
//! `arcana-gen` consumes Scryfall bulk data which is set-tagged; the
//! generator writes one module per card keyed on canonical set. Flat
//! organization (`arcana-cards/src/lightning_bolt.rs`) would be
//! simpler for a hand-written catalog but would diverge from the
//! generator's output shape, creating friction at the hand-generated
//! boundary. Function-based organization (`burn/`, `removal/`) is
//! fuzzy — Cryptic Command is a counterspell *and* a bounce spell
//! *and* a tap spell — so it's rejected.

pub mod lea;

use arcana_core::registry::CardRegistry;
use arcana_core::types::CardId;

/// The Tier 1 seed set: five basic lands + Lightning Bolt + Grizzly
/// Bears. Minimum surface for mana production, creature combat, and
/// a targeted instant. `CardId`s returned for test convenience.
#[derive(Clone, Copy, Debug)]
pub struct SeedIds {
    pub plains: CardId,
    pub island: CardId,
    pub swamp: CardId,
    pub mountain: CardId,
    pub forest: CardId,
    pub grizzly_bears: CardId,
    pub lightning_bolt: CardId,
}

/// Register every Tier 1 card. Convenience for tests and tooling;
/// production code can register selectively per set/module.
pub fn register_seed(reg: &mut CardRegistry) -> SeedIds {
    SeedIds {
        plains: lea::plains::register(reg),
        island: lea::island::register(reg),
        swamp: lea::swamp::register(reg),
        mountain: lea::mountain::register(reg),
        forest: lea::forest::register(reg),
        grizzly_bears: lea::grizzly_bears::register(reg),
        lightning_bolt: lea::lightning_bolt::register(reg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_seed_produces_distinct_card_ids() {
        let mut reg = CardRegistry::new();
        let ids = register_seed(&mut reg);
        let as_slice = [
            ids.plains, ids.island, ids.swamp, ids.mountain, ids.forest,
            ids.grizzly_bears, ids.lightning_bolt,
        ];
        let unique: std::collections::HashSet<_> = as_slice.iter().collect();
        assert_eq!(unique.len(), as_slice.len(),
            "every card in the seed set must get a distinct CardId");
    }

    #[test]
    fn every_basic_land_has_one_mana_ability() {
        let mut reg = CardRegistry::new();
        let ids = register_seed(&mut reg);
        for id in [ids.plains, ids.island, ids.swamp, ids.mountain, ids.forest] {
            let def = reg.get(id).unwrap();
            assert!(def.base_characteristics.types.is_land(),
                "basic must be land");
            assert!(def.base_characteristics.supertypes.is_basic(),
                "basic must have Basic supertype");
            assert_eq!(def.activated_abilities.len(), 1,
                "basic must have exactly one activated ability");
            assert!(def.activated_abilities[0].is_mana_ability,
                "basic's ability must be a mana ability");
        }
    }

    #[test]
    fn lightning_bolt_has_any_target_requirement() {
        let mut reg = CardRegistry::new();
        let id = lea::lightning_bolt::register(&mut reg);
        let def = reg.get(id).unwrap();
        let sa = def.spell_ability.as_ref().expect("Bolt has a spell ability");
        assert_eq!(sa.target_requirements.len(), 1);
    }

    #[test]
    fn grizzly_bears_is_2_2_green_creature() {
        use arcana_core::types::PtValue;
        let mut reg = CardRegistry::new();
        let id = lea::grizzly_bears::register(&mut reg);
        let def = reg.get(id).unwrap();
        assert!(def.base_characteristics.types.is_creature());
        assert_eq!(def.base_characteristics.power, Some(PtValue::Fixed(2)));
        assert_eq!(def.base_characteristics.toughness, Some(PtValue::Fixed(2)));
        assert!(def.base_characteristics.colors.contains(arcana_core::types::Color::Green));
    }
}
