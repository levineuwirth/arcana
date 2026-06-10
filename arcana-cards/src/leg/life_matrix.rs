//! Life Matrix — `{4}` artifact.
//! "{4}, {T}: Put a matrix counter on target creature and that
//! creature gains \"Remove a matrix counter from this creature:
//! Regenerate this creature.\" Activate only during your upkeep."
//! The counter placement is wired with a `Named` counter kind.
//! GAP: granting an activated ability to the target is not
//! expressible (only triggered abilities can be granted), and the
//! "Activate only during your upkeep" timing window has no
//! `ActivationCost` field.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Life Matrix");
    let _matrix = reg.interner_mut().intern("matrix");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}, {T}: Put a matrix counter on target creature and that creature gains \"Remove a matrix counter from this creature: Regenerate this creature.\" Activate only during your upkeep.".into(),
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
            effect: add_matrix_counter,
        }),
    )
}

fn add_matrix_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(matrix) = reg.interner().lookup("matrix") else {
        return Vec::new();
    };
    // GAP: granting the target the activated ability "Remove a matrix
    // counter from this creature: Regenerate this creature" is not
    // expressible (only triggered abilities can be granted).
    // GAP: "Activate only during your upkeep" — no activation timing
    // window field exists.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(matrix),
        count: 1,
    }]
}
