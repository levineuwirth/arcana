//! Sarkhan, the Dragonspeaker — `{3}{R}{R}` Legendary Planeswalker — Sarkhan, starting loyalty 4.
//!
//! +1: Until end of turn, Sarkhan becomes a legendary 4/4 red Dragon
//!   creature with flying, indestructible, and haste. GAP: this
//!   integrated self-animate ("becomes a creature" with printed P/T,
//!   color, type, and keywords) has no single demonstrated Effect.
//! −3: Sarkhan deals 4 damage to target creature.
//! −6: You get an emblem with "At the beginning of your draw step, draw
//!   two additional cards" and "At the beginning of your end step,
//!   discard your hand." Implemented as a two-ability triggered emblem.
//!   ("Discard your hand" has no dynamic count; approximated as discard
//!   7, an upper bound.)

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::turn::Step;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan, the Dragonspeaker");
    let sarkhan = reg.interner_mut().intern("Sarkhan");
    let _emblem = reg.interner_mut().intern("Sarkhan, the Dragonspeaker emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sarkhan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until end of turn, Sarkhan becomes a legendary 4/4 \
                       red Dragon creature with flying, indestructible, and \
                       haste.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Sarkhan deals 4 damage to target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"At the beginning of your \
                       draw step, draw two additional cards\" and \"At the \
                       beginning of your end step, discard your hand.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn plus_one_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 4/4 red Dragon creature with flying/indestructible/haste"
    // self-animation has no single demonstrated Effect.
    Vec::new()
}

fn minus_three_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Sarkhan, the Dragonspeaker emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![
                TriggeredAbilityDef {
                    id: 1,
                    trigger_condition: TriggerCondition::StepBegins {
                        step: Step::Draw,
                        whose: ControllerConstraint::You,
                    },
                    intervening_if: None,
                    effect: emblem_draw_two,
                    trigger_zones: vec![Zone::Command],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: Vec::new(),
                },
                TriggeredAbilityDef {
                    id: 2,
                    trigger_condition: TriggerCondition::StepBegins {
                        step: Step::End,
                        whose: ControllerConstraint::You,
                    },
                    intervening_if: None,
                    effect: emblem_discard_hand,
                    trigger_zones: vec![Zone::Command],
                    frequency: TriggerFrequency::EachTime,
                    target_requirements: Vec::new(),
                },
            ],
        },
    }]
}

fn emblem_draw_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 2 }]
}

fn emblem_discard_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Discard your hand" has no dynamic count; approximate with an upper
    // bound of 7 (the discard effect tops out at hand size).
    vec![Effect::Discard {
        player: trig.controller,
        count: 7,
        choice: DiscardChoice::ControllerChooses,
    }]
}
