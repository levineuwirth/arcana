//! Nova Pentacle — `{4}` artifact.
//! "{3}, {T}: The next time a source of your choice would deal damage
//! to you this turn, that damage is dealt to target creature of an
//! opponent's choice instead."
//! Wired with `Effect::RedirectDamage` (next damage to you -> the
//! target creature, until end of turn). Fidelity gaps: the redirect
//! applies to the next damage from ANY source (no "source of your
//! choice" picker), and the creature is chosen by the activator's
//! targeting rather than by an opponent.

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
    let name = reg.interner_mut().intern("Nova Pentacle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}, {T}: The next time a source of your choice would deal damage to you this turn, that damage is dealt to target creature of an opponent's choice instead.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: redirect_to_creature,
        }),
    )
}

fn redirect_to_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP (fidelity): "a source of your choice" — the redirect covers
    // the next damage from any source; "of an opponent's choice" — the
    // creature is chosen via normal targeting.
    vec![Effect::RedirectDamage {
        from: DamageTarget::Player(ctx.controller),
        to: DamageTarget::Object(*id),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
