//! Pincer Spider — `{2}{G}` 2/3 Spider with Reach.
//! Kicker {3}.
//! Reach.
//! If this creature was kicked, it enters with a +1/+1 counter on it.
//!
//! Reach is a base characteristic. Kicker is not part of the usable keyword
//! surface for this card class (the additional-cost cast mechanic and the
//! "was kicked" ETB condition are unmodeled), so both the keyword and the
//! kicked-rider ETB counter are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pincer Spider");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    // GAP: Kicker {3} — not part of the usable keyword surface; the additional
    // cost is unmodeled.
    // GAP: "If this creature was kicked, it enters with a +1/+1 counter" — the
    // "was kicked" ETB condition cannot be queried.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
