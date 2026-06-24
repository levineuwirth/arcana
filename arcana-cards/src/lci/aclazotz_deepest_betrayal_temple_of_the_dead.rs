//! Aclazotz, Deepest Betrayal // Temple of the Dead — `{3}{B}{B}` Legendary Creature — Bat God
//! 4/4 with Flying, Lifelink (front).
//!
//! Front face (Aclazotz, Deepest Betrayal):
//!   Flying, lifelink.
//!   Whenever Aclazotz attacks, each opponent discards a card. For each opponent who can't, you
//!   draw a card.
//!   Whenever an opponent discards a land card, create a 1/1 black Bat creature token with flying.
//!   When Aclazotz dies, return it to the battlefield tapped and transformed under its owner's
//!   control.
//!
//! Back face (Temple of the Dead) — Land:
//!   (Transforms from Aclazotz, Deepest Betrayal.)
//!   {T}: Add {B}.
//!   {2}{B}, {T}: Transform this land. Activate only if a player has one or fewer cards in hand
//!   and only as a sorcery.
//!
//! GAP: "For each opponent who can't discard" draw — can't check if discard succeeded;
//!      unconditional opponent discard only modeled.
//! GAP: "Whenever an opponent discards a land card" — discard-type tracking not in engine;
//!      trigger not modeled.
//! GAP: On-death "return tapped and transformed" — ReturnFromGraveyardToBattlefield doesn't
//!      support tapped+transformed; modeled as untapped transform (closest approximation).
use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aclazotz, Deepest Betrayal");
    let bat_sub = reg.interner_mut().intern("Bat");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat_sub);
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Back face: Temple of the Dead — Land
    let back_name = reg.interner_mut().intern("Temple of the Dead");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Whenever Aclazotz attacks, each opponent discards a card.
            // GAP: "for each opponent who can't, you draw a card" — discard-success tracking
            //      not in engine; draw not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "Whenever an opponent discards a land card, create a 1/1 black Bat token"
            //      — discard-type (land) tracking not in engine; trigger not modeled.
            // On-death trigger: return to battlefield transformed.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_death_return_transformed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face (Temple of the Dead — Land): {T}: Add {B}.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {B}.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: add_black,
            })
            // Back face: {2}{B}, {T} — transform this land. "Activate only if a player
            // has one or fewer cards in hand and only as a sorcery." The sorcery-speed
            // restriction is is_instant_speed:false; the hand-size precondition is the
            // activation_condition gate (checks every player).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, {T}: Transform this land. Activate only if a player has \
                       one or fewer cards in hand and only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    tap: true,
                    activation_condition: Some(if_a_player_has_one_or_fewer_in_hand),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: back_transform,
            }),
    )
}

/// Whenever Aclazotz attacks, each opponent discards a card.
fn attack_discard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    effects
}

/// When Aclazotz dies, return it to the battlefield transformed.
/// GAP: should enter tapped; ReturnFromGraveyardToBattlefield doesn't support tapped flag.
/// GAP: Transform effect after ETB not auto-chained; modeled as return then transform
///      (best-effort, will need manual follow-up).
fn on_death_return_transformed(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // We return the card from graveyard to battlefield. The engine doesn't support
    // returning tapped+transformed directly; best-effort: return then transform.
    // GAP: the transform effect on a just-returned permanent may not be sequenced correctly.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}

/// "Activate only if a player has one or fewer cards in hand" — ANY player
/// (one or fewer == hand size <= 1 == NOT (>= 2)).
fn if_a_player_has_one_or_fewer_in_hand(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::all_players(s)
        .into_iter()
        .any(|p| !conditions::hand_at_least(s, p, 2))
}

/// Back-face land mana ability: {T}: Add {B}.
fn add_black(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn back_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
