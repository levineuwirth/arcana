//! Waxen Shapethief — `{3}{U}` 0/0 Shapeshifter with Flash and Cycling {2}.
//! "You may have this creature enter as a copy of an artifact or creature you
//!  control." (copy-on-entry replacement — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waxen Shapethief");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: "You may have this creature enter as a copy of an artifact or creature you
    //   control" — a copy-as-it-enters replacement effect (CR 707), not expressible as
    //   a triggered/activated ability and with no enter-as-a-copy Effect/field.
    reg.register(CardDefinition::new(name, chars))
}
