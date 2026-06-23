//! Aeromunculus — `{1}{G}{U}` 2/3 Homunculus Mutant.
//! Flying.
//! {2}{G}{U}: Adapt 1. (If this creature has no +1/+1 counters on it,
//! put a +1/+1 counter on it.)
//!
//! The keyword line (Flying) is a base characteristic. The activated
//! Adapt ability is modeled as a mana activation that puts a +1/+1
//! counter on this creature.
//!
//! GAP: the Adapt "if this creature has no +1/+1 counters on it" gate
//! is a fidelity nuance — there is no sanctioned `Condition` variant
//! that reads the source's counter count, so the counter is placed
//! unconditionally rather than only when it has none.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aeromunculus");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G}{U}: Adapt 1.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: adapt_one,
        }),
    )
}

fn adapt_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
