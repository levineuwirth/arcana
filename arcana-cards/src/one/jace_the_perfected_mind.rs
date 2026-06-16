//! Jace, the Perfected Mind — `{2}{U}{U/P}` legendary planeswalker,
//! printed starting loyalty 3 (Compleated; the engine handles the
//! life-paid loyalty reduction — CR for Compleated).
//!
//! Keywords: Mill, Compleated — neither is in the usable keyword surface,
//! so `keywords: vec![]`.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Until your next turn, up to one target creature gets -3/-0.
//!   — GAP'd (a duration P/T modification is not in the demonstrated
//!   Effect surface).
//! * `−2`: Target player mills three cards. Then if a graveyard has twenty
//!   or more cards in it, you draw three cards. Otherwise, you draw a card.
//!   — GAP'd (mill + conditional-on-graveyard-size branch not in the
//!   demonstrated surface).
//! * `−X`: Target player mills three times X cards. — OMITTED: a dynamic
//!   `−X` loyalty cost is not expressible (`remove_self_counter` is a fixed
//!   `u32`). GAP: dynamic-X loyalty cost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, the Perfected Mind");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U/P}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, up to one target creature \
                       gets -3/-0.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target player mills three cards. Then if a graveyard \
                       has twenty or more cards in it, you draw three cards. \
                       Otherwise, you draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            }),
        // −X ability omitted — dynamic-X loyalty cost is not expressible.
    )
}

/// `+1`
fn plus_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "until your next turn, gets -3/-0" is a duration P/T modification,
    // not in the demonstrated Effect surface.
    Vec::new()
}

/// `−2`
fn minus_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: mill three + branch on whether any graveyard has ≥20 cards
    // (mill and graveyard-size conditional draw not in the demonstrated
    // surface).
    Vec::new()
}
