//! Graaz, Unstoppable Juggernaut — `{8}` 7/5 Legendary Artifact Creature —
//! Juggernaut.
//! "Juggernauts you control attack each combat if able.
//!  Juggernauts you control can't be blocked by Walls.
//!  Other creatures you control have base power and toughness 5/3 and are
//!  Juggernauts in addition to their other creature types."
//!
//! All three lines are static continuous abilities; none is a triggered or
//! activated ability, so this card carries bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Graaz, Unstoppable Juggernaut");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static — "Juggernauts you control attack each combat if able."
    // GAP: static — "Juggernauts you control can't be blocked by Walls."
    // GAP: static — "Other creatures you control have base power and
    //       toughness 5/3 and are Juggernauts in addition to their other
    //       creature types." (board-wide base-PT-set + subtype-add static).
    reg.register(CardDefinition::new(name, chars))
}
