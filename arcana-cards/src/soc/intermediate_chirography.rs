//! Intermediate Chirography
//!
//! Enchantment — Class, {1}{B}.
//! (Gain the next level as a sorcery to add its ability.)
//! Level 1 (ETB): When this Class enters, create a 2/1 white and black Inkling creature token
//! with flying.
//! {1}{B}: Level 2
//! Whenever you lose life for the first time each turn, put a +1/+1 counter on target creature
//! you control.
//! {2}{B}: Level 3
//! At the beginning of each end step, if a modified creature died under your control this turn,
//! create a 2/1 white and black Inkling creature token with flying. (Equipment, Auras you
//! control, and counters are modifications.)
//!
//! GAP: Level 2 per-level ability "Whenever you lose life for the first time each turn, put a
//! +1/+1 counter on target creature you control" — not a P/T or keyword anthem; continuous-effect
//! engine subsystem deferred. Level-up activation modeled; static ability not installed.
//! GAP: Level 3 per-level ability "At the beginning of each end step, if a modified creature
//! died under your control this turn, create a 2/1 white and black Inkling creature token with
//! flying" — not a P/T or keyword anthem; "modified creature" predicate not expressible;
//! continuous-effect engine subsystem deferred. Level-up activation modeled; static not installed.
//! Level 1 ETB token creation is modeled via a triggered ability (ETB).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationZone, CardDefinition, CardRegistry,
    EntersWithSpec,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Intermediate Chirography");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    // Pre-intern Inkling subtype for token creation
    let _inkling_sub = reg.interner_mut().intern("Inkling");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1 ETB: create a 2/1 white and black Inkling creature token with flying.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_inkling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2 activation: {1}{B}, requires level >= 1.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
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
            // Level 3 activation: {2}{B}, requires level >= 2.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
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

fn make_inkling_token(controller: arcana_core::types::PlayerId, reg: &CardRegistry) -> Effect {
    let inkling = reg.interner().lookup("Inkling").expect("Inkling interned during register()");
    let mut inkling_subtypes = SubtypeSet::default();
    inkling_subtypes.0.insert(inkling);
    Effect::CreateToken {
        controller,
        token: TokenDefinition {
            name: inkling,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: inkling_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }
}

fn etb_create_inkling(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![make_inkling_token(trig.controller, reg)]
}

fn level_up_to_2(
    _state: &arcana_core::state::GameState,
    ctx: &arcana_core::registry::ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 2 per-level granted ability "Whenever you lose life for the first time each
    // turn, put a +1/+1 counter on target creature you control" — not a P/T or keyword anthem;
    // continuous-effect engine subsystem deferred.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

fn level_up_to_3(
    _state: &arcana_core::state::GameState,
    ctx: &arcana_core::registry::ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 3 per-level granted ability "At the beginning of each end step, if a modified
    // creature died under your control this turn, create a 2/1 white and black Inkling creature
    // token with flying" — not a P/T or keyword anthem; continuous-effect engine subsystem
    // deferred.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
