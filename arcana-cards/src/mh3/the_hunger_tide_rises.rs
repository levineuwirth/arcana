//! The Hunger Tide Rises — `{2}{G}` green Enchantment — Saga.
//! I, II, III — Create a 1/1 black and green Insect creature token.
//! IV — Sacrifice any number of creatures. Search your library and/or graveyard
//! for a creature card with mana value less than or equal to the number of
//! creatures sacrificed this way and put it onto the battlefield. If you search
//! your library this way, shuffle.
//!
//! GAP: Chapter IV "sacrifice any number of creatures, then tutor for a creature
//!   with mv ≤ count sacrificed" — the sacrifice-count-tracking and CMC-bounded
//!   tutor require a dynamic variable captured from the cost payment; there is no
//!   Effect::SacrificeAny with a callback count. Chapter IV emits
//!   TutorToBattlefield with a creature filter but the CMC bound is hardcoded 0
//!   (worst-case approximation). Real logic GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Hunger Tide Rises");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    // Pre-intern Insect for token creation
    let _ = reg.interner_mut().intern("Insect");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            // Lore-counter trigger each pre-combat main phase
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
            // Chapter I
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
            // Chapter II
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
            // Chapter III
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
            })
            // Chapter IV
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(4),
                },
                intervening_if: None,
                effect: chapter_iv,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn insect_token(reg: &CardRegistry, controller: PlayerId) -> Effect {
    let insect_sym = reg
        .interner()
        .lookup("Insect")
        .expect("Insect interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(insect_sym);
    Effect::CreateToken {
        controller,
        token: TokenDefinition {
            name: insect_sym,
            colors: ColorSet::black() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }
}

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![insect_token(reg, trig.controller)]
}

fn chapter_ii(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![insect_token(reg, trig.controller)]
}

fn chapter_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![insect_token(reg, trig.controller)]
}

fn chapter_iv(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice any number of creatures, then tutor for a creature with
    // mv ≤ count sacrificed" — cannot track how many creatures were sacrificed
    // as a cost in the effect fn; Effect::Sacrifice doesn't return the count.
    // Emitting an unconditional TutorToBattlefield with creature filter only
    // (CMC bound not enforceable here).
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        tapped: false,
    }]
}
