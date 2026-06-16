//! Mwonvuli Ooze — `{G}` Ooze.
//! Cumulative upkeep {2}.
//! Power and toughness are each equal to 1 plus twice the number of age
//! counters on it.
//!
//! Both lines are unexpressible: Cumulative upkeep is not a recognized
//! KeywordAbility (nor a triggered/activated shape with the demonstrated
//! API — it adds an age counter then a per-counter pay-or-sacrifice), and
//! the characteristic-defining P/T has no PtValue form beyond Fixed.
//! Base P/T is approximated as 1/1 (the "1 plus zero counters" base).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mwonvuli Ooze");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    // GAP: Cumulative upkeep {2} — no KeywordAbility variant / no expressible
    // "add an age counter, then sacrifice unless you pay {2} per age counter".
    // GAP: characteristic-defining "1 + twice the age counters" P/T — no
    // dynamic PtValue form; base 1/1 emitted.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
