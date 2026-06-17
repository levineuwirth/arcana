//! Hish of the Snake Cult — `{2}{B}{G}{U}` 2/5 Legendary Snake.
//! "Nagas and Serpents you control are Snakes." (static type-adding)
//! "Snakes you control have daunt, deathtouch, and poisonous 2." (static
//! ability-granting)
//!
//! Both lines are continuous statics that modify OTHER creatures; this
//! card class has no static-ability registration, so both are GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hish of the Snake Cult");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static "Nagas and Serpents you control are Snakes" — no static
    // type-adding ability in this card class.
    // GAP: static "Snakes you control have daunt, deathtouch, and poisonous
    // 2" — no static keyword-granting ability in this card class (daunt and
    // poisonous are also not in the keyword surface).

    reg.register(CardDefinition::new(name, chars))
}
