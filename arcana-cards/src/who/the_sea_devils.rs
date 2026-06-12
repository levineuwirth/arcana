//! The Sea Devils — `{2}{G}` Enchantment — Saga (green).
//! I, II — Create a 2/2 green Alien Salamander creature token with islandwalk.
//! III — Until end of turn, whenever a Salamander deals combat damage to a player, it deals
//!   that much damage to target creature that player controls.
//!
//! GAP-NARROW: Chapter III "target creature that player controls" — delayed triggers carry
//!   no targets; deterministic pick: the lowest-id creature that player controls.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{Effect, FloatingUntil, KeywordAbility, TokenDefinition};
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Sea Devils");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    // Pre-intern token subtypes at registration time.
    let _alien_sub = reg.interner_mut().intern("Alien");
    let _salamander_sub = reg.interner_mut().intern("Salamander");
    let _island_sub = reg.interner_mut().intern("Island");

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
            // Add lore counter at beginning of each precombat main phase.
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
            // Chapter I: Create a 2/2 green Alien Salamander token with islandwalk.
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
            // Chapter II: Create a 2/2 green Alien Salamander token with islandwalk.
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
            // Chapter III: GAP — delayed "until end of turn, whenever..." trigger not modeled.
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

fn make_salamander_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // These strings were interned at registration time, so lookup will succeed.
    let alien_id = reg.interner().lookup("Alien");
    let salamander_id = reg.interner().lookup("Salamander");
    let island_id = reg.interner().lookup("Island");

    let mut token_subtypes = SubtypeSet::default();
    if let Some(id) = alien_id { token_subtypes.0.insert(id); }
    if let Some(id) = salamander_id { token_subtypes.0.insert(id); }

    let island_kw = match island_id {
        Some(id) => vec![KeywordAbility::Landwalk(id)],
        None => vec![],
    };

    // Token name is the primary subtype "Salamander".
    let token_name = salamander_id.or(alien_id).unwrap_or(0);

    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: island_kw,
            abilities: Vec::new(),
        },
    }]
}

fn chapter_i(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    make_salamander_token(state, trig, reg)
}

fn chapter_ii(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    make_salamander_token(state, trig, reg)
}

/// "Until end of turn, whenever a Salamander deals combat damage to a player, it deals
/// that much damage to target creature that player controls."
fn chapter_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Salamander" was interned at registration time.
    let Some(salamander) = reg.interner().lookup("Salamander") else {
        return Vec::new();
    };
    vec![Effect::ScheduleFloatingTrigger {
        source: trig.source,
        controller: trig.controller,
        condition: TriggerCondition::DamageDealt {
            source_filter: ObjectFilter::creature().with_subtype_sym(salamander),
            target_filter: TargetFilter::Player,
            combat_only: true,
        },
        effect: lash_back,
        until: FloatingUntil::EndOfTurn,
    }]
}

fn lash_back(
    state: &GameState,
    pt: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::DamageDealt {
        source,
        target: DamageTarget::Player(player),
        amount,
        ..
    } = &pt.trigger_event
    else {
        return Vec::new();
    };
    if *amount == 0 {
        return Vec::new();
    }
    // GAP-NARROW: "target creature that player controls" — delayed
    // triggers carry no targets; deterministic lowest-id pick.
    let Some(victim) = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::Player(*player)),
        pt.controller,
    )
    .into_iter()
    .min() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: *source,
        target: DamageTarget::Object(victim),
        amount: *amount,
    }]
}
