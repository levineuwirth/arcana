//! Wall of Vapor — `{3}{U}` 0/1 Creature — Wall.
//! Defender.
//! "Prevent all damage that would be dealt to this creature by
//! creatures it's blocking."
//!
//! Defender is a base keyword. The damage-prevention clause is a STATIC
//! replacement effect whose source/target relationship ("creatures it's
//! blocking") is not expressible with the demonstrated `Effect`/static
//! surface — there is no static blocking-relationship-scoped prevention
//! primitive on this card class. GAP'd; the keyword is still emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Vapor");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static — "Prevent all damage that would be dealt to this
    // creature by creatures it's blocking." No static blocking-relationship
    // damage-prevention primitive is available for this card class.
    reg.register(CardDefinition::new(name, chars))
}
