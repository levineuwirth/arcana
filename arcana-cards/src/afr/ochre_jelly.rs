//! Ochre Jelly — `{X}{G}` 0/0 Ooze with Trample.
//!
//! * Trample (keyword).
//! * "This creature enters with X +1/+1 counters on it." — enters-with-X
//!   (X = the casting value) is an as-enters replacement, not expressible as a
//!   demonstrated Effect, GAP.
//! * "Split — When this creature dies, if it had two or more +1/+1 counters,
//!   create a token copy at the next end step with half that many counters." —
//!   delayed conditional token-copy with halved counters, not expressible, GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ochre Jelly");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "enters with X +1/+1 counters" (X = casting value) — as-enters X replacement.
    // GAP: Split — delayed conditional token-copy with halved counters on death.
    reg.register(CardDefinition::new(name, chars))
}
