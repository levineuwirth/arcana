//! Minion of the Wastes — `{3}{B}{B}{B}` */* Minion with Trample.
//! "As this creature enters, pay any amount of life. Minion of the
//! Wastes's power and toughness are each equal to the life paid as it
//! entered."
//!
//! Keyword line: Trample. The "as it enters, pay any amount of life"
//! clause is a variable life-payment replacement at entry, with no
//! expressible cost field — GAP. Its P/T are `*` defined by the life
//! paid as it entered — a self-CDA whose value is the chosen entry-payment
//! amount. self_pt_cda reads only state scalars and there is no tracked
//! "life paid as it entered" quantity to read, so the value is genuinely
//! inexpressible — GAP (`PtValue::Star` transcribes the printed `*/*`).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Minion of the Wastes");
    let minion = reg.interner_mut().intern("Minion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // P/T are `*` — defined by the life paid as it entered (un-tracked).
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        // GAP: replacement — "As this creature enters, pay any amount of life" (variable life payment at entry; no expressible cost).
        // GAP: static — P/T equal to life paid as it entered (CDA from un-tracked entry payment).
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
