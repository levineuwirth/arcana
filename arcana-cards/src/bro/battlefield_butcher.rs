//! Battlefield Butcher — `{2}{B}` 1/4 black Human Soldier.
//! "{5}, {T}: Each opponent loses 2 life. This ability costs {1} less to activate
//! for each creature card in your graveyard."
//! The dynamic cost reduction is wired via `ActivationCost::cost_reduction`
//! (`script::graveyard_matching` over creature cards in your graveyard).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battlefield Butcher");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}, {T}: Each opponent loses 2 life. This ability costs {1} less to activate for each creature card in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").unwrap(),
                    tap: true,
                    // "{1} less to activate for each creature card in your
                    // graveyard."
                    cost_reduction: Some(graveyard_creatures_reduction),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: each_opponent_loses_2,
            }),
    )
}

/// "{1} less to activate for each creature card in your graveyard."
/// `controller` is the activator; their graveyard is searched for cards
/// with the creature type.
fn graveyard_creatures_reduction(
    state: &GameState,
    _source: ObjectId,
    controller: PlayerId,
    _reg: &CardRegistry,
) -> u32 {
    script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        controller,
        controller,
    )
}

fn each_opponent_loses_2(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, ctx.controller);
    opponents
        .into_iter()
        .map(|opp| Effect::LoseLife { player: opp, amount: 2 })
        .collect()
}
