//! Red Priest of Yawgmoth — `{1}{R}` 1/2 red Artifact Creature — Phyrexian Cleric.
//! "{T}, Sacrifice an artifact: Add an amount of {R} equal to the sacrificed
//! artifact's mana value."
//! GAP: "Sacrifice an artifact (not self)" not in ActivationCost. "Mana value of
//! the sacrificed artifact" is not accessible at effect resolution time.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Red Priest of Yawgmoth");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice an artifact: Add an amount of {R} equal to the sacrificed artifact's mana value.".into(),
                cost: ActivationCost {
                    tap: true,
                    // GAP: "Sacrifice an artifact (not self)" not in ActivationCost
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_per_mana_value,
            }),
    )
}

fn add_red_per_mana_value(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: The mana value of the sacrificed artifact is not accessible.
    // Cannot compute the amount of {R} to add dynamically.
    Vec::new()
}
