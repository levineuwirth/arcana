//! Mirrorshell Crab — `{5}{U}{U}` 5/7 Artifact Creature — Crab.
//! Ward {3}.
//! Channel — {2}{U}, Discard this card: Counter target spell or ability
//! unless its controller pays {3}. (GAP: Channel is not in the usable keyword
//! surface and counter-unless-pay is not expressible.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mirrorshell Crab");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crab);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{3}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: Channel — {2}{U}, Discard this card: counter target spell or
    // ability unless its controller pays {3}. The Channel discard-from-hand
    // counter-unless-pay ability is not expressible with the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
