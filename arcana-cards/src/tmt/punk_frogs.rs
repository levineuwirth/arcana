//! Punk Frogs — `{3}{G/U}{G/U}` 4/5 Frog Mutant Rebel with Ward {3}.
//! A green-blue hybrid creature with three creature subtypes and Ward {3},
//! taxing opponents who target it with spells or abilities.
//!
//! # Rules references
//!
//! * CR 702.20j — Ward. Whenever this permanent becomes the target of a
//!   spell or ability an opponent controls, counter it unless that player
//!   pays the ward cost. Implemented as `KeywordAbility::Ward(ManaCost)`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Punk Frogs");
    let frog = reg.interner_mut().intern("Frog");
    let mutant = reg.interner_mut().intern("Mutant");
    let rebel = reg.interner_mut().intern("Rebel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(mutant);
    subtypes.0.insert(rebel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{3}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
