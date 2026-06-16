//! Necravolver — `{2}{B}` 2/2 Volver.
//! "Kicker {1}{G} and/or {W}. If kicked with its {1}{G} kicker, it enters
//! with two +1/+1 counters and trample. If kicked with its {W} kicker, it
//! enters with a +1/+1 counter and 'Whenever this creature deals damage, you
//! gain that much life.'"
//!
//! All non-bones text is GAP'd: Kicker is not a KeywordAbility variant and
//! there is no kicker-cost / was-kicked accessor to gate the
//! enters-with-counters/keyword/granted-ability riders. Only bones are
//! emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necravolver");
    let volver = reg.interner_mut().intern("Volver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(volver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Kicker {1}{G} and/or {W} — Kicker is not a KeywordAbility variant.
    // GAP: kicked-rider enters-with-counters / trample / granted lifegain
    // ability — no was-kicked accessor to gate them.
    reg.register(CardDefinition::new(name, chars))
}
