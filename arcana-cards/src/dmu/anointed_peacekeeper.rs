//! Anointed Peacekeeper — `{2}{W}` 3/3 Human Cleric with Vigilance.
//!
//! * Vigilance — base keyword.
//! * "As this creature enters, look at an opponent's hand, then choose any card
//!   name." — an as-enters look-and-name replacement choice with no expressible
//!   primitive, GAP'd.
//! * "Spells your opponents cast with the chosen name cost {2} more to cast."
//!   and "Activated abilities of sources with the chosen name cost {2} more to
//!   activate unless they're mana abilities." — name-keyed cost-increase
//!   statics with no expressible primitive, GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anointed Peacekeeper");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // GAP: "As this creature enters, look at an opponent's hand, then choose any
    // card name" (replacement look-and-name choice) and the two name-keyed
    // cost-increase statics — none have an expressible primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
