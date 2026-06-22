//! Hallowed Healer — `{2}{W}` 1/1 Human Cleric.
//!
//! * `{T}: Prevent the next 2 damage that would be dealt to any target this
//!   turn.`
//! * Threshold — `{T}: Prevent the next 4 damage that would be dealt to any
//!   target this turn. Activate only if there are seven or more cards in your
//!   graveyard.`
//!
//! Threshold is not an expressible keyword (`keywords: vec![]`); the
//! seven-cards gate is modeled as an `activation_condition` on the second
//! ability.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hallowed Healer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Prevent the next 2 damage that would be dealt to any \
                       target this turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_2,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Prevent the next 4 damage that would be dealt to any \
                       target this turn. Activate only if there are seven or more \
                       cards in your graveyard."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    activation_condition: Some(if_threshold),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_4,
            }),
    )
}

fn target_to_damage(ctx: &ActivationContext) -> Option<DamageTarget> {
    let target = ctx.targets.targets.first()?;
    Some(match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return None,
    })
}

fn prevent_2(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(dt) = target_to_damage(ctx) else { return Vec::new(); };
    vec![Effect::PreventDamage {
        target: dt,
        amount: Some(2),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn prevent_4(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(dt) = target_to_damage(ctx) else { return Vec::new(); };
    vec![Effect::PreventDamage {
        target: dt,
        amount: Some(4),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn if_threshold(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::graveyard_at_least(s, you, 7)
}
