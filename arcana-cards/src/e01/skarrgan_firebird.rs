//! Skarrgan Firebird — `{4}{R}{R}` 3/3 Phoenix with Bloodthirst 3 and
//! Flying.
//!
//! * Bloodthirst 3 — enters with three +1/+1 counters if an opponent
//!   was dealt damage this turn (engine-wired from the keyword).
//! * Flying.
//! * "{R}{R}{R}: Return this card from your graveyard to your hand.
//!   Activate only if an opponent was dealt damage this turn." —
//!   graveyard-zone activated ability gated on an opponent having lost
//!   life this turn (the faithful damage proxy).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skarrgan Firebird");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Bloodthirst(3), KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}{R}{R}: Return this card from your graveyard to your hand. Activate only if an opponent was dealt damage this turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}{R}{R}").expect("valid cost"),
                activation_condition: Some(if_opponent_damaged),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: return_self_from_graveyard,
        }),
    )
}

fn if_opponent_damaged(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    // Closest faithful proxy for "an opponent was dealt damage this turn":
    // damage to a player causes life loss this turn.
    conditions::an_opponent_lost_life_this_turn(s, you)
}

fn return_self_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToHand { target: ctx.source }]
}
