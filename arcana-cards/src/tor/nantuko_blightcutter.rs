//! Nantuko Blightcutter — `{2}{G}` 2/2 Insect Druid.
//! "Protection from black."
//! "Threshold — This creature gets +1/+1 for each black permanent your
//!  opponents control as long as there are seven or more cards in your
//!  graveyard."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nantuko Blightcutter");
    let insect = reg.interner_mut().intern("Insect");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(druid);

    // GAP: keyword Protection (from black) — not in the implemented keyword
    // surface; no Protection variant.
    // GAP: static Threshold ability "+1/+1 for each black permanent your
    // opponents control as long as 7+ cards are in your graveyard" — a
    // graveyard-gated dynamic continuous P/T boost; not expressible as a
    // triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
