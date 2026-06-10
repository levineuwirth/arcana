//! Ebony Horse — `{3}` artifact (Arabian Nights).
//! "{2}, {T}: Untap target attacking creature you control. Prevent all
//! combat damage that would be dealt to and dealt by that creature
//! this turn."
//!
//! Modeled: untap the target + prevent all damage TO it this turn
//! (over-broad: covers noncombat damage too — GAP). The damage DEALT
//! BY the creature cannot be prevented (no by-specific-object source
//! filter — GAP). The "attacking" target restriction is likewise not
//! expressible (GAP).

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
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ebony Horse");
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
                text: "{2}, {T}: Untap target attacking creature you \
                       control. Prevent all combat damage that would be \
                       dealt to and dealt by that creature this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: "attacking" restriction on the target is not
                // expressible; modeled as target creature you control.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: untap_and_shield,
            },
        ),
    )
}

fn untap_and_shield(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: only damage dealt TO the creature is prevented (and from any
    // source, not just combat); "prevent all combat damage dealt BY that
    // creature" needs a source filter pinned to a specific object, which
    // ObjectFilter cannot express.
    vec![
        Effect::Untap { target: *id },
        Effect::PreventDamage {
            target: DamageTarget::Object(*id),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}
