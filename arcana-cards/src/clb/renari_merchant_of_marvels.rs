//! Renari, Merchant of Marvels — `{3}{U}` 2/4 Legendary Dragon Artificer.
//! Both lines are static / format-only: "You may cast Dragon spells and artifact
//! spells as though they had flash" is a continuous casting-permission static, and
//! "Choose a Background" is a commander deck-construction keyword. Neither is a
//! triggered/activated ability, so this card is bones-only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Renari, Merchant of Marvels");
    let dragon = reg.interner_mut().intern("Dragon");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "You may cast Dragon spells and artifact spells as though they
    //      had flash" — a continuous casting-permission static.
    // GAP: "Choose a Background" — commander deck-construction keyword.
    reg.register(CardDefinition::new(name, chars))
}
