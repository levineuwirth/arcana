//! Drake with Set's Mechanic — `{2}{U}` 2/2 Drake with Flying.
//! "Poison Tolerance +2 (It takes two additional poison counters for you to
//! lose the game to poison.)"
//!
//! Flying is a keyword. "Poison Tolerance +2" is a static modification of
//! the poison loss threshold (a state-based-action rule alteration) with no
//! Effect / KeywordAbility surface, so it is GAP'd. Bones + Flying faithful.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drake with Set's Mechanic");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Poison Tolerance +2" — static raise of the poison loss threshold;
    // no Effect / KeywordAbility surface.
    reg.register(CardDefinition::new(name, chars))
}
