//! Auriok Siege Sled — `{6}` 3/5 Artifact Creature — Juggernaut.
//!
//! * `{1}`: Target artifact creature blocks this creature this turn if
//!   able. (Effect GAP'd — no source-scoped "must block this creature"
//!   requirement effect.)
//! * `{1}`: Target artifact creature can't block this creature this
//!   turn. (Effect GAP'd — `ForbidBlocking` is a board-wide "can't
//!   block at all", not a per-source "can't block this creature".)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Auriok Siege Sled");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let artifact_creature = || TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::new()
                .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Target artifact creature blocks this creature this turn if able.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![artifact_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: must_block,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Target artifact creature can't block this creature this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![artifact_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cant_block,
            }),
    )
}

fn must_block(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "blocks this creature this turn if able" — no source-scoped
    // must-block-this-creature requirement effect.
    Vec::new()
}

fn cant_block(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can't block this creature this turn" — ForbidBlocking is a
    // board-wide can't-block-at-all, not per-source.
    Vec::new()
}
