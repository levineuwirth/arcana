//! Behold the Unspeakable // Vision of the Unspeakable — {3}{U}{U} Enchantment — Saga (front)
//! transforming into Enchantment Creature — Spirit (back).
//!
//! (As this Saga enters and after your draw step, add a lore counter.)
//! I — Creatures you don't control get -2/-0 until your next turn.
//! II — If you have one or fewer cards in hand, draw four cards. Otherwise, scry 2, then draw two cards.
//! III — Exile this Saga, then return it to the battlefield transformed under your control.
//! ---
//! Vision of the Unspeakable: Flying, trample. Gets +1/+1 for each card in your hand.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::registry::EntersWithSpec;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Behold the Unspeakable");
    let saga_sub = reg.interner_mut().intern("Saga");
    let back_name = reg.interner_mut().intern("Vision of the Unspeakable");
    let spirit_sub = reg.interner_mut().intern("Spirit");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);

    // GAP: back face static "gets +1/+1 for each card in your hand" (dynamic continuous P/T) not
    // expressible as a static; printed base P/T is */* — emitting Flying + Trample faithfully.
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_lore_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_i,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_ii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: chapter_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

// I — Creatures you don't control get -2/-0 until your next turn.
// GAP: "until your next turn" duration not expressible — using EndOfTurn as the closest duration.
fn chapter_i(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: -2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect()
}

// II — If you have one or fewer cards in hand, draw four; otherwise scry 2, then draw two.
fn chapter_ii(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let hand = script::hand_size(state, trig.controller);
    if hand <= 1 {
        vec![Effect::DrawCards {
            player: trig.controller,
            count: 4,
        }]
    } else {
        vec![
            Effect::Scry {
                player: trig.controller,
                count: 2,
            },
            Effect::DrawCards {
                player: trig.controller,
                count: 2,
            },
        ]
    }
}

// III — Exile this Saga, then return it to the battlefield transformed under your control.
fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
