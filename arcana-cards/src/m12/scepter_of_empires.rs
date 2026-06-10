//! Scepter of Empires — `{3}` artifact (Magic 2012 Empires cycle).
//! "{T}: This artifact deals 1 damage to target player or
//! planeswalker. It deals 3 damage instead if you control artifacts
//! named Crown of Empires and Throne of Empires." The amount is
//! computed at resolution by checking for the two named artifacts.
//!
//! GAP: the planeswalker half of "target player or planeswalker" is
//! not expressible (no planeswalker TargetFilter in this catalog);
//! the target is declared as a player only.

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
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scepter of Empires");
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
                text: "{T}: This artifact deals 1 damage to target player \
                       or planeswalker. It deals 3 damage instead if you \
                       control artifacts named Crown of Empires and Throne \
                       of Empires."
                    .into(),
                cost: ActivationCost::tap_only(),
                // GAP: planeswalker targets not expressible — player only.
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: scepter_damage,
            },
        ),
    )
}

fn scepter_damage(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let crown = reg.interner().lookup("Crown of Empires");
    let throne = reg.interner().lookup("Throne of Empires");
    let has_crown = script::count_matching(
        state,
        &ObjectFilter {
            name: crown,
            types: Some(TypeLine::ARTIFACT.into()),
            ..ObjectFilter::default()
        }
        .controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) > 0;
    let has_throne = script::count_matching(
        state,
        &ObjectFilter {
            name: throne,
            types: Some(TypeLine::ARTIFACT.into()),
            ..ObjectFilter::default()
        }
        .controlled_by(ControllerConstraint::You),
        ctx.controller,
    ) > 0;
    let amount = if has_crown && has_throne { 3 } else { 1 };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*p),
        amount,
        source: ctx.source,
    }]
}
