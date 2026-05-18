//! Tomakul Honor Guard — `{1}{G}` 3/1 Human Soldier with Ward {2}
//! (Dominaria United, common).
//!
//! # Rules text
//!
//! Ward {2} (Whenever this creature becomes the target of a spell or ability
//! an opponent controls, counter it unless that player pays {2}.)
//!
//! # Rules references
//!
//! * CR 702.20 — Ward. Whenever this permanent becomes the target of a spell
//!   or ability an opponent controls, counter that spell or ability unless its
//!   controller pays the ward cost. The ward cost here is the mana cost {2},
//!   which is expressible as `KeywordAbility::Ward(ManaCost::parse("{2}"))`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tomakul Honor Guard");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
