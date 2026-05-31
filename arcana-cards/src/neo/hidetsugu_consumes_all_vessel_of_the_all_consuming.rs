//! Hidetsugu Consumes All // Vessel of the All-Consuming — {1}{B}{R} B/R transforming
//! Saga (CR 716 / 712).
//!
//! Front face (Hidetsugu Consumes All): Enchantment — Saga.
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I — Destroy each nonland permanent with mana value 1 or less.
//!   II — Exile all graveyards.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back face (Vessel of the All-Consuming): Enchantment Creature — Ogre Shaman, 8/8, Trample.
//!   Whenever this creature deals damage, put a +1/+1 counter on it.
//!   Whenever this creature deals damage to a player, if it has dealt 10 or more damage to
//!     that player this turn, they lose the game.
//!
//! # Notes / GAPs
//! - Chapter I "destroy each nonland permanent with mana value 1 or less" — enumerated via
//!   script::ids_matching over a permanent filter with max-cmc 1, applied with ForEach.
//! - Chapter II "Exile all graveyards" — there is no board-wide graveyard-exile primitive in
//!   the demonstrated surface (ExileFromGraveyard targets a single card, and script:: gives no
//!   graveyard-card id enumeration). GAP.
//! - Chapter III "Exile this Saga, then return it transformed" — modeled as self-transform.
//! - Back-face P/T is the printed 8/8 (Scryfall); Trample is on the face.
//! - GAP: the back face's two triggered abilities ("Whenever this creature deals damage, put a
//!   +1/+1 counter on it" and the 10-damage lose-the-game trigger) cannot be attached — CardFace
//!   carries only name/characteristics/spell_ability, with no slot for back-face triggers.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hidetsugu Consumes All");
    let saga_sub = reg.interner_mut().intern("Saga");
    let ogre = reg.interner_mut().intern("Ogre");
    let shaman = reg.interner_mut().intern("Shaman");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Vessel of the All-Consuming — Enchantment Creature — Ogre Shaman, 8/8, Trample.
    let back_name = reg.interner_mut().intern("Vessel of the All-Consuming");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(ogre);
    back_subtypes.0.insert(shaman);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::red(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(8)),
            keywords: vec![KeywordAbility::Trample],
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
            // First main phase: add a lore counter.
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
            // Chapter I — Destroy each nonland permanent with mana value 1 or less.
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
            // Chapter II — Exile all graveyards. (GAP — no board-wide graveyard-exile primitive.)
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
            // Chapter III — Exile this Saga, return it transformed under your control.
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

fn chapter_i(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Destroy each nonland permanent with mana value 1 or less.
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .with_max_cmc(1);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}

fn chapter_ii(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Exile all graveyards." No board-wide graveyard-exile primitive in the demonstrated
    // surface (ExileFromGraveyard is single-target and no graveyard-card enumeration helper exists).
    Vec::new()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control" — self-transform.
    vec![Effect::Transform {
        target: trig.source,
    }]
}
