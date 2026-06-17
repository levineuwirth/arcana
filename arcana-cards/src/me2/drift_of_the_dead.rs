//! Drift of the Dead — `{3}{B}` */* Wall with Defender.
//!
//! "Drift of the Dead's power and toughness are each equal to the number
//! of snow lands you control." — a characteristic-defining ability. The
//! printed P/T is `*/*` (`PtValue::Star`); GAP: the CDA that sets the
//! value from your snow-land count is a pure static with no expressible
//! primitive in this surface, so the `*` resolves to its default.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drift of the Dead");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: CDA "P/T each equal to the number of snow lands you control".
    reg.register(CardDefinition::new(name, chars))
}
