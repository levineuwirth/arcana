//! Muldrotha, the Gravetide — `{3}{B}{G}{U}` 6/6 Legendary Elemental Avatar.
//! "During each of your turns, you may play a land and cast a permanent spell of
//!  each permanent type from your graveyard." (static permission — GAP)
//!
//! The graveyard-casting permission is a continuous play-permission static with
//! no expressible primitive, so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Muldrotha, the Gravetide");
    let elemental = reg.interner_mut().intern("Elemental");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(avatar);

    // GAP: "During each of your turns, you may play a land and cast a permanent
    // spell of each permanent type from your graveyard." — a play-from-graveyard
    // permission static with no expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
