//! Ian Chesterton — `{2}{W}` 2/3 Legendary Human Scientist.
//! "Science Teacher — Each Saga spell you cast has replicate ..." is a
//! static cast-modifier (GAP — not a triggered/activated ability, and
//! replicate is not an expressible mechanic). "Doctor's companion" is a
//! deck-construction rule (GAP — no gameplay ability). Only the bones
//! are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ian Chesterton");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Science Teacher — Each Saga spell you cast has replicate." —
    // a static cast-modifier; replicate is not an expressible mechanic.
    // GAP: "Doctor's companion" — a deck-construction rule, not a
    // gameplay ability.
    reg.register(CardDefinition::new(name, chars))
}
