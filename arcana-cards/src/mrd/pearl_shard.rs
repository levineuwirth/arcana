//! Pearl Shard — `{3}` artifact (Mirrodin, 2003).
//! "{3}, {T} or {W}, {T}: Prevent the next 2 damage that would be
//! dealt to any target this turn." The or-cost is modeled as two
//! separately activatable abilities, one per printed cost.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pearl Shard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, {T}: Prevent the next 2 damage that would be \
                       dealt to any target this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}: Prevent the next 2 damage that would be \
                       dealt to any target this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_two,
            }),
    )
}

fn prevent_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(choice) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let target = match choice {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    vec![Effect::PreventDamage {
        target,
        amount: Some(2),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
