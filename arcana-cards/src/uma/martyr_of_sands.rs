//! Martyr of Sands — `{W}` 1/1 white Human Cleric.
//! "{1}, Reveal X white cards from your hand, Sacrifice this creature: You gain three times X life."
//! GAP: "Reveal X white cards from your hand" is not an ActivationCost field.
//! The X-variable life gain (3*X) cannot be computed without knowing X from hand reveals.
//! Modeling as mana + sacrifice cost; GAP on the reveal cost and dynamic life gain.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Martyr of Sands");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Reveal X white cards from your hand, Sacrifice this creature: You gain three times X life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_life_from_reveal,
            }),
    )
}

fn gain_life_from_reveal(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Reveal X white cards from your hand" — no ActivationCost field for hand reveals.
    // The life gain amount (3*X) is dynamic based on X cards revealed; cannot compute without
    // access to the reveal mechanic. Full effect not expressible.
    Vec::new()
}
