//! Insectoid Exterminator — `{2}{B}` 2/2 Insect Mutant.
//! Flying.
//! Disappear — At the beginning of your end step, if a permanent left
//! the battlefield under your control this turn, scry 1.
//!
//! Flying is a base keyword (Scry / Disappear are mechanics, not keyword
//! variants). The end-step trigger has an intervening-if gate ("if a
//! permanent left the battlefield under your control this turn") with no
//! matching conditions:: predicate, so per the intervening-if discipline
//! (never fire an "if"-gated trigger unconditionally) the whole ability
//! is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Insectoid Exterminator");
    let insect = reg.interner_mut().intern("Insect");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "At the beginning of your end step, if a permanent left the
    //      battlefield under your control this turn, scry 1" — no
    //      conditions:: predicate for that intervening-if gate.
    reg.register(CardDefinition::new(name, chars))
}
