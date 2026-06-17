//! Myojin of Night's Reach and Grim Betrayal — `{5}{B}{B}{B}` 10/4 Legendary
//! Spirit.
//! "Myojin ... enters the battlefield with an indestructible counter on it if
//!  you cast it from your hand."
//! "Remove an indestructible counter from Myojin ...: Each opponent discards
//!  their hand. Put onto the battlefield under your control all creature cards
//!  in all graveyards that were put there from anywhere this turn."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Night's Reach and Grim Betrayal");
    let spirit = reg.interner_mut().intern("Spirit");
    let indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "enters with an indestructible counter on it if you cast it from
    // your hand" — a cast-zone-conditional ETB replacement; not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove an indestructible counter from Myojin of Night's Reach \
                   and Grim Betrayal: Each opponent discards their hand. Put onto \
                   the battlefield under your control all creature cards in all \
                   graveyards that were put there from anywhere this turn."
                .into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Named(indestructible), 1)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: night_reach,
        }),
    )
}

fn night_reach(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Each opponent discards their hand" (no all-hand-discard Effect;
    // discard_hand is only a cost field) and "put onto the battlefield all
    // creature cards in all graveyards that were put there from anywhere this
    // turn" (no this-turn-graveyard mass-reanimation Effect). Whole body GAP'd.
    Vec::new()
}
