//! Jade Monolith — `{4}` artifact.
//! "{1}: The next time a source of your choice would deal damage to target
//! creature this turn, that source deals that damage to you instead."
//! Wired via `Effect::RedirectDamage` (next damage to the target creature is
//! redirected to you); the "source of your choice" restriction is a GAP —
//! the redirect applies to the next damage from ANY source.

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
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jade Monolith");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}: The next time a source of your choice would deal damage to target creature this turn, that source deals that damage to you instead.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: redirect_to_you,
        }),
    )
}

fn redirect_to_you(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "a source of your choice" — the redirect applies to the next
    // damage from ANY source, not a chosen one.
    vec![Effect::RedirectDamage {
        from: DamageTarget::Object(*id),
        to: DamageTarget::Player(ctx.controller),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
