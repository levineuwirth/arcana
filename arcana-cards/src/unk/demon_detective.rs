//! Demon Detective — `{1}{B}` 4/4 Demon Detective with Flying.
//! "You can't cast Demon Detective unless all of your commanders have been
//! revealed."
//!
//! Flying is a base keyword. The cast-restriction is a static casting
//! modifier with no expressible primitive — GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Demon Detective");
    let demon = reg.interner_mut().intern("Demon");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "can't cast unless all of your commanders have been revealed" — no cast-restriction primitive.
    reg.register(CardDefinition::new(name, chars))
}
