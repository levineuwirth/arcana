//! Tyrant's Familiar — `{5}{R}{R}` 5/5 Dragon with Flying and Haste.
//! "Flying, haste.
//!  Lieutenant — As long as you control your commander, this creature gets
//!   +2/+2 and has 'Whenever this creature attacks, it deals 7 damage to target
//!   creature defending player controls.'"
//!
//! Flying and Haste are base keywords. Lieutenant is not a supported keyword,
//! and its conditional ("as long as you control your commander") static pump +
//! granted attack-trigger has no expressible hook — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyrant's Familiar");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "Lieutenant — as long as you control your commander, gets +2/+2 and has
    // '<attack trigger>'" — Lieutenant keyword + commander-conditional static not expressible.
    reg.register(CardDefinition::new(name, chars))
}
