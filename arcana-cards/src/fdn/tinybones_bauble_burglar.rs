//! Tinybones, Bauble Burglar — `{1}{B}` 1/3 Legendary Creature — Skeleton Rogue.
//! Whenever an opponent discards a card, exile it from their graveyard with a
//!   stash counter on it.
//! During your turn, you may play cards you don't own with stash counters on
//!   them from exile, and mana of any type can be spent to cast those spells.
//! {3}{B}, {T}: Each opponent discards a card. Activate only as a sorcery.
//!
//! Decomposition:
//! 1. CardDiscarded (opponent) trigger. GAP the body: there is no accessor for
//!    "the discarded card" object and no Effect that exiles a just-discarded
//!    card from a graveyard with a named counter on it. The trigger condition
//!    is still recorded.
//! 2. "During your turn, you may play cards you don't own with stash counters
//!    …" — pure static play-permission. No Effect; GAP (documented only).
//! 3. {3}{B}, {T}: Each opponent discards a card → mana + tap activation,
//!    Sequence of one Discard per opponent. ("Activate only as a sorcery" =
//!    is_instant_speed: false.)

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tinybones, Bauble Burglar");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: exile_discarded_with_stash,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}, {T}: Each opponent discards a card. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: each_opponent_discards,
            }),
    )
}

fn exile_discarded_with_stash(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile it from their graveyard with a stash counter on it." No
    // accessor for the just-discarded card object, and no Effect exiles a
    // specific graveyard card with a named counter. Effect omitted.
    Vec::new()
}

fn each_opponent_discards(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
