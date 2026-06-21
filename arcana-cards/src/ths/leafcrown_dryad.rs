//! Leafcrown Dryad — `{1}{G}` 2/2 Enchantment Creature — Nymph Dryad
//! with Reach.
//!
//! * Reach (keyword).
//! * Bestow {3}{G} — not in the usable keyword surface (GAP).
//! * Enchanted creature gets +2/+2 and has reach. — a Bestow/Aura static
//!   buff; the aura-attach subsystem is not in this card class's
//!   documented surface (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leafcrown Dryad");
    let nymph = reg.interner_mut().intern("Nymph");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nymph);
    subtypes.0.insert(dryad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
