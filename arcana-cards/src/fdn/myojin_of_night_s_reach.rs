//! Myojin of Night's Reach — `{5}{B}{B}{B}` 5/2 Legendary Spirit.
//! Enters with a divinity counter if cast from hand (GAP). Has indestructible
//! while it has a divinity counter (GAP — conditional static keyword).
//! "Remove a divinity counter from Myojin of Night's Reach: Each opponent
//! discards their hand."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Night's Reach");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let divinity = reg.interner_mut().intern("divinity");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "enters with a divinity counter if you cast it from your hand" — no
    // cast-from-hand ETB condition / EntersWith spec in scope.
    // GAP: "has indestructible as long as it has a divinity counter" — a
    // conditional static keyword grant is not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove a divinity counter from Myojin of Night's Reach: Each opponent discards their hand.".into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Named(divinity), 1)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: each_opponent_discards_hand,
        }),
    )
}

fn each_opponent_discards_hand(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Each opponent discards their hand" — discard each opponent's entire hand
    // (computed at resolution from their current hand size).
    let mut effects = Vec::new();
    for opp in script::opponents(state, ctx.controller) {
        let n = script::hand_size(state, opp);
        if n > 0 {
            effects.push(Effect::Discard {
                player: opp,
                count: n,
                choice: DiscardChoice::ControllerChooses,
            });
        }
    }
    effects
}
