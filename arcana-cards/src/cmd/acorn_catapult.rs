//! Acorn Catapult — `{4}` artifact.
//! "{1}, {T}: This artifact deals 1 damage to any target. That permanent's
//! controller or that player creates a 1/1 green Squirrel creature token."
//! An any-target pinger whose victim is compensated with a Squirrel.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Acorn Catapult");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, {T}: This artifact deals 1 damage to any \
                       target. That permanent's controller or that player \
                       creates a 1/1 green Squirrel creature token."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_and_compensate,
            },
        ),
    )
}

fn ping_and_compensate(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let (dmg_target, token_player) = match target {
        TargetChoice::Object(id) => (
            DamageTarget::Object(*id),
            script::target_controller(state, *id, ctx.controller),
        ),
        TargetChoice::Player(p) => (DamageTarget::Player(*p), *p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => (
            DamageTarget::Object(*id),
            script::target_controller(state, *id, ctx.controller),
        ),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            (DamageTarget::Player(*p), *p)
        }
        _ => return Vec::new(),
    };
    let squirrel = reg.interner().lookup("Squirrel").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    vec![
        Effect::DealDamage {
            target: dmg_target,
            amount: 1,
            source: ctx.source,
        },
        Effect::CreateToken {
            controller: token_player,
            token: TokenDefinition {
                name: squirrel,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
