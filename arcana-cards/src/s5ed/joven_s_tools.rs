//! Joven's Tools — `{6}` artifact (Homelands, 1995).
//! "{4}, {T}: Target creature can't be blocked this turn except by Walls."
//!
//! GAP: the "except by Walls" carve-out is not expressible —
//! `Effect::CantBeBlocked` is unconditional. The wired effect is strictly
//! stronger than printed (plain unblockable); flagged for routing.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Joven's Tools");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Target creature can't be blocked this turn \
                       except by Walls."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: unblockable_except_walls,
            },
        ),
    )
}

fn unblockable_except_walls(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "except by Walls" — CantBeBlocked has no blocker carve-out;
    // this is plain unblockable (strictly stronger than printed).
    vec![Effect::CantBeBlocked { target: *id, duration: Duration::EndOfTurn }]
}
