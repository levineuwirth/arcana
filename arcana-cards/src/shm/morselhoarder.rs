//! Morselhoarder — `{4}{R/G}{R/G}` 6/4 Creature — Elemental.
//! "This creature enters with two -1/-1 counters on it."
//! "Remove a -1/-1 counter from this creature: Add one mana of any color."
//!
//! The "enters with two -1/-1 counters" line is a static entering replacement
//! with no expressible primitive in this card class — GAP'd. The activated
//! mana ability's cost (remove a -1/-1 counter) IS expressible, but its effect
//! ("add one mana of any color") has no choose-a-color mana primitive, so the
//! effect body GAPs (emitting a fixed color would be materially wrong).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Morselhoarder");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    // GAP: "enters with two -1/-1 counters on it" — static entering replacement;
    // no enters-with primitive is available for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R/G}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove a -1/-1 counter from Morselhoarder: Add one mana of any color.".into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::MinusOneMinusOne, 1)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_any_color,
        }),
    )
}

fn add_any_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add one mana of any color" — no choose-a-color mana primitive is
    // available; emitting a fixed color would be materially wrong.
    Vec::new()
}
