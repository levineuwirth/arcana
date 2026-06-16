//! Korlessa, Scale Singer — `{G}{U}` 1/4 Legendary Dragon Bard.
//! You may look at the top card of your library any time. (static — GAP)
//! You may cast Dragon spells from the top of your library. (static — GAP)
//!
//! Both lines are pure static play-permission abilities with no
//! triggered/activated form expressible in this card class — emitted as a
//! vanilla legendary creature with the statics GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Korlessa, Scale Singer");
    let dragon = reg.interner_mut().intern("Dragon");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(bard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
