//! Arcane Investigator — `{1}{U}` 2/1 Creature — Elf Wizard.
//! "Search the Room — {5}{U}: Roll a d20.
//!  1—9 | Draw a card.
//!  10—20 | Look at the top three cards of your library. Put one of them
//!  into your hand and the rest on the bottom of your library in any order."
//!
//! The ability branches on a d20 roll, which has no Effect primitive (no
//! die-roll / random-outcome variant). The whole effect is GAP'd.

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
    let name = reg.interner_mut().intern("Arcane Investigator");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Search the Room — {5}{U}: Roll a d20. 1—9: Draw a card. 10—20: Look at the top three cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: search_the_room,
        }),
    )
}

fn search_the_room(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Roll a d20" with banded outcomes — no die-roll Effect primitive.
    Vec::new()
}
