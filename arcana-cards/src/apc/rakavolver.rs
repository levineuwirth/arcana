//! Rakavolver — `{2}{R}` 2/2 red Volver.
//!
//! Rules text:
//! * Kicker {1}{W} and/or {U}. (Kicker is not in the usable KeywordAbility
//!   surface — emitted as `keywords: vec![]` and GAP'd.)
//! * If this creature was kicked with its {1}{W} kicker, it enters with two
//!   +1/+1 counters on it and with "Whenever this creature deals damage, you
//!   gain that much life." (kicker-conditional ETB rider — GAP)
//! * If this creature was kicked with its {U} kicker, it enters with a +1/+1
//!   counter on it and with flying. (kicker-conditional ETB rider — GAP)
//!
//! There is no Kicker cost field nor a "was kicked with X" condition in the
//! demonstrated API, so both as-it-enters riders (which gate entirely on which
//! kicker was paid) are unexpressible and GAP'd. No ability is expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rakavolver");
    let volver = reg.interner_mut().intern("Volver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(volver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker is not in the usable KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "If kicked with {1}{W}, enters with two +1/+1 counters and a
    //       lifegain-on-damage triggered ability." — kicker-conditional ETB
    //       rider; no Kicker cost / "was kicked" condition in the API.
    // GAP: "If kicked with {U}, enters with a +1/+1 counter and flying." —
    //       same kicker-conditional ETB rider limitation.

    reg.register(CardDefinition::new(name, chars))
}
