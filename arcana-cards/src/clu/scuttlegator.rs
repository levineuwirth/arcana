//! Scuttlegator — `{4}{G/U}{G/U}` 6/6 Crab Turtle Crocodile with Defender.
//! `{6}{G/U}{G/U}: Adapt 3.`
//! "As long as this creature has a +1/+1 counter on it, it can attack as
//! though it didn't have defender."
//!
//! Defender is a base keyword. Adapt (conditional "if no +1/+1 counters,
//! put three") has no engine primitive — the activated ability carries the
//! correct cost but the effect body is a GAP. The defender-bypass static is
//! also a GAP (no "attack despite defender while counter present" primitive).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scuttlegator");
    let crab = reg.interner_mut().intern("Crab");
    let turtle = reg.interner_mut().intern("Turtle");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crab);
    subtypes.0.insert(turtle);
    subtypes.0.insert(crocodile);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G/U}{G/U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{G/U}{G/U}: Adapt 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{G/U}{G/U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_three,
            }),
    )
    // GAP: static "As long as this creature has a +1/+1 counter on it, it can
    // attack as though it didn't have defender" — no defender-bypass primitive.
}

// GAP: Adapt 3 — "if this creature has no +1/+1 counters on it, put three +1/+1
// counters on it" has no conditional-on-zero-counters primitive; emitting an
// unconditional AddCounters would be materially wrong, so the body is empty.
fn adapt_three(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
