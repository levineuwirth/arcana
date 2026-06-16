//! Sarkhan, the Dragonspeaker — `{3}{R}{R}` Legendary Planeswalker — Sarkhan, starting loyalty 4.
//!
//! +1: Until end of turn, Sarkhan becomes a legendary 4/4 red Dragon
//!   creature with flying, indestructible, and haste. IMPLEMENTED via PW
//!   animation — AddType(CREATURE) + SetBasePT(4/4) + SetColor(red) +
//!   GrantKeyword(Flying/Indestructible/Haste), all EndOfTurn. (Dragon
//!   subtype grant has no demonstrated Effect — GAP'd; the animation
//!   body is otherwise complete.)
//! −3: Sarkhan deals 4 damage to target creature. IMPLEMENTED.
//! −6: You get an emblem with "At the beginning of your draw step, draw
//!   two additional cards" and "At the beginning of your end step,
//!   discard your hand." IMPLEMENTED as a two-ability triggered emblem.
//!   ("Discard your hand" has no dynamic count; approximated as discard
//!   7, an upper bound.)

use arcana_core::effects::{DiscardChoice, Effect, EmblemDefinition, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
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

/// `+1` — Sarkhan becomes a 4/4 red creature with flying, indestructible, haste.
fn plus_one_animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "Dragon" creature-type grant has no demonstrated Effect
    // (no add-subtype primitive). Everything else is expressible.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: ctx.source,
            colors: ColorSet::red(),
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
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
