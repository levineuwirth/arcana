//! Cognivore — `{6}{U}{U}` */* Creature — Lhurgoyf.
//!
//! Oracle:
//! * Flying.
//! * "Cognivore's power and toughness are each equal to the number of
//!   instant cards in all graveyards." — a characteristic-defining
//!   ability; the base P/T are `*`. There is no demonstrated effect or
//!   static primitive that continuously sets P/T equal to a graveyard
//!   count, so the CDA itself is a GAP; we emit the `*`/`*` bones.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP static (CDA): P/T equal to the number of instant cards in all
// graveyards — no continuous self-P/T-from-count primitive available.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cognivore");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
