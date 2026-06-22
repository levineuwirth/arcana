//! Nessian Asp — `{4}{G}` 4/5 Snake with Reach.
//!
//! Reach
//! {6}{G}: Monstrosity 4. (If this creature isn't monstrous, put four
//! +1/+1 counters on it and it becomes monstrous.)
//!
//! The activated ability puts four +1/+1 counters on this creature via
//! `Effect::AddCounters`. GAP: there is no Monstrosity primitive — the
//! "becomes monstrous" flag and the "only if not already monstrous"
//! once-only gate are not modeled, so the counters can be added more than
//! once. The counter payload (the gameplay-relevant part) is faithful.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nessian Asp");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{6}{G}: Monstrosity 4.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{6}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: monstrosity_4,
        }),
    )
}

fn monstrosity_4(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "becomes monstrous" flag + "only if not already monstrous" gate
    // are not modeled. The +1/+1 counter payload is wired faithfully.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 4,
    }]
}
