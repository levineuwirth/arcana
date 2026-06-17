//! Sphinx of Jwar Isle — `{4}{U}{U}` 5/5 Sphinx with Flying and Shroud.
//! "You may look at the top card of your library any time." (informational
//! static — GAP'd; no rules effect on the battlefield).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphinx of Jwar Isle");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Shroud],
        ..Default::default()
    };

    // GAP: "You may look at the top card of your library any time." — a purely
    // informational static, no battlefield rules effect.

    reg.register(CardDefinition::new(name, chars))
}
