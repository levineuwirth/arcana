//! Chandra, Awakened Inferno gives each opponent an upkeep-damage emblem.
//! GAP: −X needs damage-dependent exile support.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Awakened Inferno");
    let chandra = reg.interner_mut().intern("Chandra");
    let _elemental = reg.interner_mut().intern("Elemental");
    let _emblem = reg
        .interner_mut()
        .intern("Chandra, Awakened Inferno emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Each opponent gets an emblem with \"At the beginning \
                       of your upkeep, this emblem deals 1 damage to you.\""
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_opponent_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Chandra, Awakened Inferno deals 3 damage to each \
                       non-Elemental creature."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_sweep,
            }),
    )
}

fn plus_two_opponent_emblem(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, ctx.controller)
        .into_iter()
        .map(|controller| Effect::CreateEmblem {
            controller,
            emblem: EmblemDefinition {
                name: reg
                    .interner()
                    .lookup("Chandra, Awakened Inferno emblem")
                    .expect("interned at register"),
                statics: vec![],
                abilities: vec![TriggeredAbilityDef {
                    id: 1,
                    trigger_condition: TriggerCondition::StepBegins {
                        step: Step::Upkeep,
                        whose: ControllerConstraint::You,
                    },
                    intervening_if: None,
                    effect: emblem_damage,
                    trigger_zones: vec![Zone::Command],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: vec![],
                }],
            },
        })
        .collect()
}

fn minus_three_sweep(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg
        .interner()
        .lookup("Elemental")
        .expect("Elemental interned at register");
    let filter = ObjectFilter::creature().without_subtype_sym(elemental);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 3,
        })
        .collect()
}

fn emblem_damage(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(trig.controller),
        amount: 1,
    }]
}
