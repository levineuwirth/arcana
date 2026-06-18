//! Myojin of Grim Betrayal — `{5}{B}{B}{B}` 5/2 Legendary Spirit.
//! Enters with an indestructible counter if cast from hand.
//! Remove an indestructible counter: put onto the battlefield under your
//! control all creature cards put into any graveyard this turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Grim Betrayal");
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
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "enters with an indestructible counter on it if you cast it from
    // your hand" — a cast-zone-conditioned enters-with is not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove an indestructible counter from Myojin of Grim Betrayal: Put onto the battlefield under your control all creature cards in all graveyards that were put there from anywhere this turn.".into(),
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
            effect: mass_reanimate,
        }),
    )
}

fn mass_reanimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Put onto the battlefield under your control all creature cards in
    // all graveyards that were put there from anywhere this turn." No script
    // helper enumerates graveyard cards that entered the graveyard this turn,
    // and Reanimate is a single non-targeted pick — there is no mass
    // this-turn-only reanimation primitive.
    Vec::new()
}
