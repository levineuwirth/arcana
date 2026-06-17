//! Twinflame Travelers — `{2}{U}{R}` 3/3 Elemental Sorcerer with Flying.
//! "If a triggered ability of another Elemental you control triggers,
//! it triggers an additional time."
//!
//! The trigger-doubling clause is a static replacement-style effect with
//! no expressible primitive, so it is GAP'd. Flying is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "If a triggered ability of another Elemental you control triggers,
//      it triggers an additional time." — a static replacement-style
//      doubling of other permanents' triggers; not expressible.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Twinflame Travelers");
    let elemental = reg.interner_mut().intern("Elemental");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
