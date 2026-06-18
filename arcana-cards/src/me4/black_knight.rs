//! Black Knight — `{B}{B}` 2/2 Human Knight.
//!
//! Oracle:
//! * First strike → `KeywordAbility::FirstStrike` (in the keyword line).
//! * "Protection from white" — GAP: Protection is NOT part of the usable
//!   keyword surface for this card class (no `KeywordAbility::Protection`
//!   variant is demonstrated), so it is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Black Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        // GAP: "Protection from white" — no usable Protection keyword.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
