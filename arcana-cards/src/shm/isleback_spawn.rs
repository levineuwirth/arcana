//! Isleback Spawn — `{5}{U}{U}` 4/8 Kraken.
//! Shroud.
//! This creature gets +4/+8 as long as a library has twenty or fewer cards.
//! (GAP: conditional static continuous P/T buff not expressible as a
//! triggered/activated ability.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Isleback Spawn");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    // GAP: "gets +4/+8 as long as a library has twenty or fewer cards" — a
    // conditional static continuous buff; no triggered/activated form.

    reg.register(CardDefinition::new(name, chars))
}
