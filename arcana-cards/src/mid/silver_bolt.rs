//! Silver Bolt — `{1}` artifact (Innistrad: Midnight Hunt era).
//! "{3}, {T}, Sacrifice this artifact: It deals 3 damage to target
//! creature. If a Werewolf is dealt damage this way, destroy it." The
//! Werewolf rider is checked at resolution: if the target is a Werewolf
//! it is also destroyed (fidelity note: the printed rider keys on damage
//! actually being DEALT — a prevention shield would skip the destroy,
//! which this resolution-time check cannot see).

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
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silver Bolt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{3}, {T}, Sacrifice this artifact: It deals 3 damage \
                       to target creature. If a Werewolf is dealt damage \
                       this way, destroy it."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: silver_shot,
            },
        ),
    )
}

fn silver_shot(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount: 3,
        source: ctx.source,
    }];
    let werewolves = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Werewolf"),
        ctx.controller,
    );
    if werewolves.contains(id) {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    effects
}
