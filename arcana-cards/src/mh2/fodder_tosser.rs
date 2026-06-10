//! Fodder Tosser — `{3}` artifact.
//! "{T}, Discard a card: This artifact deals 2 damage to target player or
//! planeswalker." Tap + discard-a-card cost; the 'player or planeswalker'
//! target is approximated with `any_target` (which also admits creatures
//! — a documented over-broad gap), and the resolver handles all three
//! target shapes.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fodder Tosser");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Discard a card: This artifact deals 2 damage \
                       to target player or planeswalker."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                // GAP: 'target player or planeswalker' — any_target also
                // admits creatures; no player-or-planeswalker-only
                // TargetFilter exists.
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: toss_fodder,
            },
        ),
    )
}

fn toss_fodder(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
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
    vec![Effect::DealDamage { target: dt, amount: 2, source: ctx.source }]
}
