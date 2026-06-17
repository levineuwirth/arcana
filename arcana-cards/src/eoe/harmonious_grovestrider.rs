//! Harmonious Grovestrider — `{3}{G}{G}` */* Beast with Ward {2}.
//! "Harmonious Grovestrider's power and toughness are each equal to the
//! number of lands you control" is a characteristic-defining ability;
//! the `*/*` is recorded via `PtValue::Star`, but the count-lands CDA
//! body has no triggered/activated/keyword representation here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harmonious Grovestrider");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: CDA "power and toughness each equal to the number of lands you
    // control" — `*/*` recorded via PtValue::Star, but no ability hook in
    // the demonstrated API computes the live land count for the CDA.

    reg.register(CardDefinition::new(name, chars))
}
