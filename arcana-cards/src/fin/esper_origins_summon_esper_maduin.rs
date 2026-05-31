//! Esper Origins // Summon: Esper Maduin — `{1}{G}` Sorcery (front).
//!
//! Front face (Esper Origins — Sorcery):
//! "Surveil 2. You gain 2 life. If this spell was cast from a graveyard, exile
//! it, then put it onto the battlefield transformed under its owner's control
//! with a finality counter on it."
//! Flashback {3}{G}.
//!
//! Back face (Summon: Esper Maduin — Enchantment Creature — Saga Elemental):
//! I — Reveal the top card of your library. If it's a permanent card, put it
//!     into your hand.
//! II — Add {G}{G}.
//! III — Other creatures you control get +2/+2 and gain trample until end of turn.
//!
//! GAPs:
//! - "Flashback" keyword is not in the implemented KeywordAbility surface — the
//!   cast-from-graveyard-for-flashback-cost mechanic is not modeled.
//! - The front spell's "If this spell was cast from a graveyard, exile it then
//!   put it onto the battlefield transformed ... with a finality counter" — the
//!   cast-from-graveyard condition and "put onto battlefield transformed" are
//!   not expressible; only Surveil 2 + gain 2 life are modeled on the front.
//! - The Saga back face's chapters are authored as back-face-gated CounterAdded
//!   triggers; chapter II's "Add {G}{G}" and chapter III's dynamic team pump are
//!   modeled, chapter I uses DigTopN as the closest reveal-to-hand approximation.
//! - Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::layers::Duration;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf,
    TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet,
    SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esper Origins");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        // GAP: Flashback {3}{G} not modeled (keyword not in implemented surface)
        keywords: vec![],
        ..Default::default()
    };

    let front_ability = SpellAbilityDef {
        text: "Surveil 2. You gain 2 life. If this spell was cast from a \
               graveyard, exile it, then put it onto the battlefield \
               transformed under its owner's control with a finality counter \
               on it."
            .into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: front_resolve,
    };

    // Back face — Summon: Esper Maduin (Enchantment Creature — Saga Elemental)
    let back_name = reg.interner_mut().intern("Summon: Esper Maduin");
    let saga = reg.interner_mut().intern("Saga");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga);
    back_subtypes.0.insert(elemental);
    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes: back_subtypes,
        supertypes: SupertypeSet::default(),
        // P/T not given in spec; FFXIV Summon Sagas read P/T from print — using a
        // conservative placeholder consistent with the catalog's Saga-creature shape.
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    let back = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(front_ability)
            .with_transform_back(back)
            // Back-face Saga chapter triggers (gated to face 1 below).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
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
                id: 2,
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
                id: 3,
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
            })
            .with_trigger_face_gate(1, 1)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1),
    )
}

fn front_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If this spell was cast from a graveyard, exile it, then put it onto
    // the battlefield transformed ... with a finality counter" — cast-from-
    // graveyard condition and put-onto-battlefield-transformed not expressible.
    vec![
        Effect::Surveil { player: entry.controller, count: 2 },
        Effect::GainLife { player: entry.controller, amount: 2 },
    ]
}

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Reveal the top card of your library. If it's a permanent card, put it
    // into your hand." Closest: look at top 1, may put a permanent into hand.
    let permanent = ObjectFilter::new().with_types_any(TypeLine(
        TypeLine::CREATURE
            | TypeLine::ARTIFACT
            | TypeLine::ENCHANTMENT
            | TypeLine::LAND
            | TypeLine::PLANESWALKER,
    ));
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 1,
        filter: Some(permanent),
        rest: DigRest::BottomRandom,
    }]
}

fn chapter_ii(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Add {G}{G}."
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, trig.source); 2],
    }]
}

fn chapter_iii(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Other creatures you control get +2/+2 and gain trample until end of turn."
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let pumps: Vec<Effect> = script::ids_matching(state, &filter, trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .map(|id| Effect::Pump {
            target: id,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample],
        })
        .collect();
    vec![Effect::Sequence(pumps)]
}
