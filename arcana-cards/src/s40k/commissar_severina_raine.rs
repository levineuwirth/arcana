//! Commissar Severina Raine — `{1}{W}{B}` 2/2 Legendary Human Soldier.
//! "Leading from the Front — Whenever Commissar Severina Raine attacks,
//! each opponent loses X life, where X is the number of other attacking
//! creatures."
//! "Summary Execution — {2}, Sacrifice another creature: You gain 2 life
//! and draw a card."
//! (The two named lines are ability words, not keywords — no keyword vec.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Commissar Severina Raine");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: leading_from_the_front,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Summary Execution — {2}, Sacrifice another creature: You gain 2 life and draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: summary_execution,
            }),
    )
}

fn leading_from_the_front(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // X = number of OTHER attacking creatures = all attackers minus self.
    let attacking = script::count_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        trig.controller,
    );
    let x = attacking.saturating_sub(1);
    if x == 0 {
        return Vec::new();
    }
    let opps = script::opponents(state, trig.controller);
    opps.into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: x })
        .collect()
}

fn summary_execution(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GainLife {
            player: ctx.controller,
            amount: 2,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
    ]
}
