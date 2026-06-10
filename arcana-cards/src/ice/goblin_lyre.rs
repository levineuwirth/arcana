//! Goblin Lyre — `{3}` artifact.
//! "Sacrifice this artifact: Flip a coin. If you win the flip, this
//! artifact deals damage to target opponent or planeswalker equal to
//! the number of creatures you control. If you lose the flip, this
//! artifact deals damage to you equal to the number of creatures that
//! opponent or that planeswalker's controller controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Lyre");
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
                text: "Sacrifice this artifact: Flip a coin. If you win the \
                       flip, this artifact deals damage to target opponent \
                       or planeswalker equal to the number of creatures you \
                       control. If you lose the flip, this artifact deals \
                       damage to you equal to the number of creatures that \
                       opponent or that planeswalker's controller controls."
                    .into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                // GAP: "target opponent or planeswalker" — any_target is
                // the closest filter (it also admits creatures and you).
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: flip_and_burn,
            },
        ),
    )
}

fn flip_and_burn(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let creature_filter =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let yours = script::count_matching(state, &creature_filter, ctx.controller);
    let (dmg_target, their_controller) = match target {
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
    let theirs =
        script::count_matching(state, &creature_filter, their_controller);
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::DealDamage {
            target: dmg_target,
            amount: yours,
            source: ctx.source,
        }),
        lose: Some(Box::new(Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: theirs,
            source: ctx.source,
        })),
    }]
}
