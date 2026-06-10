//! Goblin Cannon — `{4}` artifact.
//! "{2}: This artifact deals 1 damage to any target. Sacrifice this
//! artifact." A mana-only activation with an any-target ping; the
//! resolution-time self-sacrifice is modeled as `DestroyPermanent` on
//! the source (the closest primitive — GAP noted).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Cannon");
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
                text: "{2}: This artifact deals 1 damage to any target. \
                       Sacrifice this artifact."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: fire_and_blow_up,
            },
        ),
    )
}

fn fire_and_blow_up(
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
    // GAP: "Sacrifice this artifact" on resolution — no targeted
    // self-sacrifice effect; modeled as DestroyPermanent on the
    // source (closest primitive; destroy vs sacrifice differs under
    // regeneration/indestructible).
    vec![
        Effect::DealDamage { target, amount: 1, source: ctx.source },
        Effect::DestroyPermanent { target: ctx.source },
    ]
}
