//! Dread Wanderer — `{B}` 2/1 Zombie Jackal.
//! "This creature enters tapped."
//! "{2}{B}: Return this card from your graveyard to the battlefield. Activate
//!  only as a sorcery and only if you have one or fewer cards in hand."
//!
//! "Enters tapped" is a static with no primitive in this API surface — GAP'd.
//! The graveyard recursion ability is fully expressible: a graveyard-activated
//! ability gated at sorcery speed by an activation_condition (hand of one or
//! fewer) that returns this card from the graveyard to the battlefield.

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
    let name = reg.interner_mut().intern("Dread Wanderer");
    let zombie = reg.interner_mut().intern("Zombie");
    let jackal = reg.interner_mut().intern("Jackal");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(jackal);

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

    // GAP: static — "enters tapped" (no enters-tapped primitive here).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Return this card from your graveyard to the battlefield. Activate only as a sorcery and only if you have one or fewer cards in hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    activation_condition: Some(if_one_or_fewer_in_hand),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self_to_battlefield,
            }),
    )
}

fn if_one_or_fewer_in_hand(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "one or fewer cards in hand" == hand size <= 1 == NOT (>= 2).
    !conditions::hand_at_least(s, you, 2)
}

fn return_self_to_battlefield(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
