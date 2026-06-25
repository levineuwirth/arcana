//! Goblin Diplomats — `{1}{R}` 2/1 Goblin.
//! `{T}:` Each creature attacks this turn if able.
//! Wired as a board-wide must-attack (CR 508.1a) over every creature with
//! EndOfTurn duration.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Diplomats");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Each creature attacks this turn if able.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: force_attack,
            }),
    )
}

fn force_attack(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Each creature attacks this turn if able" — every creature, no controller
    // constraint, for this turn only.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_must_attack(
            ctx.source,
            ObjectFilter::creature(),
            Duration::EndOfTurn,
        ),
    }]
}
