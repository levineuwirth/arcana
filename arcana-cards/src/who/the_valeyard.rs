//! The Valeyard — `{2}{U}{B}{R}` 4/5 Legendary Creature — Time Lord Noble.
//!
//! Both printed lines are static abilities affecting the villainous-choice
//! and voting subsystems, neither of which is modeled:
//! * "If an opponent would face a villainous choice, they face that choice
//!   an additional time." — static replacement on villainous choices.  // GAP
//! * "While voting, you may vote an additional time." — static voting
//!   modifier.  // GAP
//!
//! No triggered or activated abilities to express; bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Valeyard");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
