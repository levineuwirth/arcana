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
pub mod lrw;
pub mod isd;
pub mod po2;
pub mod aer;
pub mod mh2;
pub mod rav;
pub mod m11;
pub mod hou;
pub mod m15;
pub mod zen;
pub mod ons;
pub mod tor;
pub mod eld;
pub mod znr;
pub mod apc;
pub mod ktk;
pub mod akh;
pub mod eve;
pub mod m14;
pub mod mrd;

pub mod fdn;

pub mod m13;

pub mod s10e;

pub mod xln;

pub mod mmq;

pub mod gs1;

pub mod rix;

pub mod bok;

pub mod rna;

pub mod por;

pub mod m20;

pub mod dgm;

pub mod ice;

pub mod m12;

pub mod s7ed;

pub mod ptk;

pub mod jmp;

pub mod tsb;

pub mod war;

pub mod me4;

pub mod thb;

pub mod bbd;

pub mod tpr;

pub mod s9ed;

pub mod me3;

pub mod dft;

pub mod s99;

pub mod bng;

pub mod dtk;

pub mod tdc;

pub mod nph;

pub mod soi;

pub mod tsp;

pub mod tle;

pub mod m10;

pub mod ths;

pub mod leg;

pub mod som;

pub mod s8ed;

pub mod s00;

pub mod frf;

pub mod grn;

pub mod mir;

pub mod shm;

pub mod ala;

pub mod ust;

pub mod arb;

pub mod mm3;

pub mod s6ed;

pub mod w17;

pub mod chr;

pub mod cns;

pub mod p02;

pub mod oana;

pub mod m21;

pub mod mom;

pub mod m19;

pub mod vis;

pub mod gtc;

pub mod cmm;

pub mod kld;

pub mod stx;

pub mod ori;

pub mod cn2;

pub mod roe;

pub mod gpt;

pub mod a25;

/// Staging area for arcana-gen card generations. See the module
/// docs — this is intermediate storage, not a stable public API.
pub mod generated;

use arcana_core::registry::CardRegistry;
use arcana_core::types::CardId;

/// The Tier 1–3 seed set. Tier 1 = five basic lands + Lightning Bolt
/// + Grizzly Bears (mana, combat, targeted instant). Tier 2 adds
/// Counterspell (stack targeting), Murder (destroy), Elvish
/// Visionary (ETB-draw trigger), Glorious Anthem (layer-7c static),
/// Disintegrate (X-cost damage). Tier 3 adds Walking Ballista
/// (X-in-P/T via `EntersWithSpec::CountersFromX` + counter-removal-
/// as-activation-cost). The keyword-stress pack adds Serra Angel
/// (Flying + Vigilance), Giant Spider (Reach), and Typhoid Rats
/// (Deathtouch) so the already-wired evergreen combat keywords get
/// exercised via real seed cards rather than only synthetic combat
/// tests. Abrade (modal), Chandra, Pyromaster (loyalty), and Burst
/// Lightning (Kicker) anchor the Phase 2 mechanics each in a real
/// printed card. Preordain (Scry 2, then draw a card) anchors
/// sequential multi-effect resolution — the engine must park the
/// draw while the scry's `OrderCards` prompt is open and resume it
/// when the placements come in. `CardId`s returned for test
/// convenience.
#[derive(Clone, Copy, Debug)]
pub struct SeedIds {
    pub plains: CardId,
    pub island: CardId,
    pub swamp: CardId,
    pub mountain: CardId,
    pub forest: CardId,
    pub grizzly_bears: CardId,
    pub lightning_bolt: CardId,
    pub counterspell: CardId,
    pub murder: CardId,
    pub elvish_visionary: CardId,
    pub glorious_anthem: CardId,
    pub disintegrate: CardId,
    pub walking_ballista: CardId,
    pub snapcaster_mage: CardId,
    pub murktide_regent: CardId,
    pub chord_of_calling: CardId,
    pub serra_angel: CardId,
    pub giant_spider: CardId,
    pub typhoid_rats: CardId,
    pub abrade: CardId,
    pub chandra_pyromaster: CardId,
    pub burst_lightning: CardId,
    pub tranquil_thicket: CardId,
    pub fiery_temper: CardId,
    pub bonecrusher_giant: CardId,
    pub tangled_florahedron: CardId,
    pub fire_ice: CardId,
    pub monastery_swiftspear: CardId,
    pub ahn_crop_crasher: CardId,
    pub slippery_bogle: CardId,
    pub servo_exhibition: CardId,
    pub young_pyromancer: CardId,
    pub bonesplitter: CardId,
    pub preordain: CardId,
}

/// Register every seed card. Convenience for tests and tooling;
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
        counterspell: lea::counterspell::register(reg),
        murder: isd::murder::register(reg),
        elvish_visionary: lrw::elvish_visionary::register(reg),
        glorious_anthem: po2::glorious_anthem::register(reg),
        disintegrate: lea::disintegrate::register(reg),
        walking_ballista: aer::walking_ballista::register(reg),
        snapcaster_mage: isd::snapcaster_mage::register(reg),
        murktide_regent: mh2::murktide_regent::register(reg),
        chord_of_calling: rav::chord_of_calling::register(reg),
        serra_angel: lea::serra_angel::register(reg),
        giant_spider: lea::giant_spider::register(reg),
        typhoid_rats: m11::typhoid_rats::register(reg),
        abrade: hou::abrade::register(reg),
        chandra_pyromaster: m15::chandra_pyromaster::register(reg),
        burst_lightning: zen::burst_lightning::register(reg),
        tranquil_thicket: ons::tranquil_thicket::register(reg),
        fiery_temper: tor::fiery_temper::register(reg),
        bonecrusher_giant: eld::bonecrusher_giant::register(reg),
        tangled_florahedron: znr::tangled_florahedron::register(reg),
        fire_ice: apc::fire_ice::register(reg),
        monastery_swiftspear: ktk::monastery_swiftspear::register(reg),
        ahn_crop_crasher: akh::ahn_crop_crasher::register(reg),
        slippery_bogle: eve::slippery_bogle::register(reg),
        servo_exhibition: aer::servo_exhibition::register(reg),
        young_pyromancer: m14::young_pyromancer::register(reg),
        bonesplitter: mrd::bonesplitter::register(reg),
        preordain: m11::preordain::register(reg),
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
            ids.counterspell, ids.murder, ids.elvish_visionary,
            ids.glorious_anthem, ids.disintegrate, ids.walking_ballista,
            ids.snapcaster_mage, ids.murktide_regent, ids.chord_of_calling,
            ids.serra_angel, ids.giant_spider, ids.typhoid_rats,
            ids.abrade, ids.chandra_pyromaster, ids.burst_lightning,
            ids.tranquil_thicket, ids.fiery_temper, ids.bonecrusher_giant,
            ids.tangled_florahedron, ids.fire_ice, ids.monastery_swiftspear,
            ids.ahn_crop_crasher, ids.slippery_bogle,
            ids.servo_exhibition, ids.young_pyromancer,
            ids.bonesplitter, ids.preordain,
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
