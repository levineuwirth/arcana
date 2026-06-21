//! Gutterbones — `{B}` 2/1 Skeleton Warrior.
//!
//! * This creature enters tapped. (no documented enters-tapped hook — GAP)
//! * `{1}{B}: Return this card from your graveyard to your hand.` Activate
//!   only during your turn and only if an opponent lost life this turn.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gutterbones");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP (static): "This creature enters tapped." — no documented
    // enters-tapped characteristic / replacement hook for the creature itself.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Return this card from your graveyard to your hand. Activate only during your turn and only if an opponent lost life this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    activation_condition: Some(can_recur),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self,
            }),
    )
}

fn can_recur(
    state: &GameState,
    _source: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    state.active_player() == you
        && conditions::an_opponent_lost_life_this_turn(state, you)
}

fn return_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
