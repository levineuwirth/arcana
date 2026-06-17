//! Moraug, Fury of Akoum — `{4}{R}{R}` 6/6 Legendary Minotaur Warrior.
//! "Each creature you control gets +1/+0 for each time it has attacked
//! this turn." (dynamic continuous anthem — GAP'd)
//! "Landfall — Whenever a land you control enters, if it's your main
//! phase, there's an additional combat phase after this phase. At the
//! beginning of that combat, untap all creatures you control."
//! (additional-combat-phase generation — GAP'd)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moraug, Fury of Akoum");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: "+1/+0 for each time it has attacked this turn" — per-creature times-attacked anthem not expressible.
    // GAP: Landfall additional combat phase — no Effect to create an extra combat phase / its begin-of-combat untap.
    reg.register(CardDefinition::new(name, chars))
}
