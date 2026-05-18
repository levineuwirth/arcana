//! Rotting Mastodon — vanilla 2/8 black creature for `{4}{B}`. A
//! Zombie Elephant with no abilities, pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rotting Mastodon");
    let subtype_zombie = reg.interner_mut().intern("Zombie");
    let subtype_elephant = reg.interner_mut().intern("Elephant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_zombie);
    subtypes.0.insert(subtype_elephant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
