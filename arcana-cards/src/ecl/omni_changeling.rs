//! Omni-Changeling — `{3}{U}{U}` 0/0 blue Shapeshifter.
//!
//! * Changeling (keyword). Convoke is not in the usable keyword surface; GAP'd.
//! * "You may have this creature enter as a copy of any creature on the
//!   battlefield, except it has changeling." — an enters-as-a-copy replacement
//!   is not expressible with the available API; GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omni-Changeling");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: Convoke not expressible.
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    // GAP: "enter as a copy of any creature" — no enters-as-a-copy replacement.
    reg.register(CardDefinition::new(name, chars))
}
