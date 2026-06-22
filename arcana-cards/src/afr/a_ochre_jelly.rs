//! A-Ochre Jelly — `{X}{G}` 0/0 Creature — Ooze.
//!
//! Oracle:
//! * Trample
//! * Ward {2}
//! * Ochre Jelly enters with X +1/+1 counters on it. — GAP (no accessor
//!   for a creature's cast X value at enters-the-battlefield)
//! * Split — When Ochre Jelly dies, if it had two or more +1/+1 counters
//!   on it, create a token that's a copy of it at the beginning of the
//!   next end step. The token enters with half that many +1/+1 counters,
//!   rounded down. — GAP
//!
//! Trample and Ward {2} are base keywords. The enters-with-X-counters
//! static and the Split dies trigger (delayed copy with half the
//! counters, gated on the now-destroyed object's counter count) are not
//! expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Ochre Jelly");
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
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: "Ochre Jelly enters with X +1/+1 counters on it." — no
    //      accessor for the cast X value at enters-the-battlefield.
    // GAP: "Split — When Ochre Jelly dies, if it had two or more +1/+1
    //      counters on it, create a token that's a copy of it at the
    //      beginning of the next end step, entering with half that many
    //      +1/+1 counters" — delayed copy with half-counters gated on
    //      the destroyed object's counter count is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
