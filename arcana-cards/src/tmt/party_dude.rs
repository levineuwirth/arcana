//! Party Dude — `{G}` green Enchantment — Class.
//! Level 1: When this Class enters, each player creates a Food token.
//! Level 2 ({1}{G}): Whenever an artifact an opponent controls is put into a graveyard
//!   from the battlefield, draw a card.
//! Level 3 ({4}{G}): Whenever one or more of your opponents are attacked,
//!   up to one target attacking creature gets +X/+X until end of turn, where X is the
//!   number of cards in your hand.
//!
//! # GAPs
//! - Level 2 trigger: "artifact an opponent controls put into graveyard from battlefield"
//!   fires on ZoneChange. Modeled faithfully.
//! - Level 3 trigger: "whenever one or more of your opponents are attacked" — no exact
//!   trigger condition for opponent-is-attacked; approximated with CreatureAttacks
//!   filtering creatures you control (fires when you attack). Semantics differ slightly.
//! - Level 3: +X/+X where X is hand size — modeled dynamically via script::hand_size.
//! - Keyword "Food" on scryfall: represented by Level 1 ETB creating Food tokens.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Party Dude");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1 ETB: each player creates a Food token
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2 trigger: opponent artifact dies → draw a card
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: artifact_dies_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 3 trigger: when you attack → up to one target attacking creature gets +X/+X
            // GAP: "whenever one or more of your opponents are attacked" not modeled exactly;
            // approximated as "whenever a creature you control attacks".
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: attacking_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            })
            // Level 2 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // Level 3 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{G}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            }),
    )
}

fn etb_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Each player creates a Food token.
    // GAP: "each player" — we create one for each player including the controller.
    // For now: controller gets one Food; opponents also each get one.
    // Using CreateCommodityToken for the controller.
    // Full "each player" needs script::all_players but CreateCommodityToken only takes one player.
    // Model as controller creates Food (partial — doesn't give each opponent Food).
    // GAP: each opponent also creates a Food token not modeled.
    vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Food,
            count: 1,
        }
    ]
}

fn artifact_dies_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

fn attacking_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = script::hand_size(state, trig.controller) as i32;
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 }]
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 }]
}
