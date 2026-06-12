//! Night of the Flying Merfolk
//!
//! {2}{U} Enchantment — Saga
//! Bedtime story variant: GAP: The "Bedtime story" reminder says lore counters
//! go at the beginning of your END STEP rather than main phase, and the Saga
//! does NOT enter with a counter. This is non-standard Saga behavior; the engine
//! Saga primitives use PreCombatMain + ETB counter. We approximate with the
//! standard engine Saga wiring (ETB counter + PreCombatMain trigger). The
//! precise Bedtime story timing difference is a GAP.
//!
//! I — Create two 1/1 blue Merfolk creature tokens.
//! II — Put a flying counter on each tapped creature you control.
//!      Wired as `CounterKind::Named("flying")` plus a permanent Flying
//!      grant (CR 122.1g) per creature.
//! III — Draw a card for each creature you control that dealt combat damage to
//!       a player this turn. GAP: per-creature combat-damage-dealt-this-turn
//!       tracking not in script API; emitting Vec::new().

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Night of the Flying Merfolk");
    let saga_sub = reg.interner_mut().intern("Saga");
    let _merfolk_sub = reg.interner_mut().intern("Merfolk");
    // Interned for chapter II's lookup of the named counter kind.
    reg.interner_mut().intern("flying");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            // CR 716.3 — add lore counter at beginning of controller's first main phase
            // GAP: Bedtime story says this should trigger at end step instead
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
            // Chapter I — Create two 1/1 blue Merfolk creature tokens
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
            // Chapter II — Put a flying counter on each tapped creature you control
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
            // Chapter III — Draw a card for each creature you control that dealt
            // combat damage to a player this turn.
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

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let merfolk_id = reg.interner().lookup("Merfolk").expect("Merfolk interned at register");
    let mut merfolk_subtypes = SubtypeSet::default();
    merfolk_subtypes.0.insert(merfolk_id);
    let make_token = || TokenDefinition {
        name: merfolk_id,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: merfolk_subtypes.clone(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: make_token(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: make_token(),
        },
    ]
}

fn chapter_ii(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Flying counter on each tapped creature you control, plus the keyword
    // it grants (CR 122.1g), modeled as a permanent grant; narrowed GAP:
    // removing the counter later would not revoke the keyword.
    let kind = reg.interner().lookup("flying").map(CounterKind::Named);
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .tapped_only();
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .flat_map(|id| {
            let mut per = Vec::new();
            if let Some(kind) = kind {
                per.push(Effect::AddCounters { target: id, kind, count: 1 });
            }
            per.push(Effect::GrantKeyword {
                target: id,
                keyword: KeywordAbility::Flying,
                duration: Duration::Permanent,
            });
            per
        })
        .collect()
}

fn chapter_iii(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "draw a card for each creature you control that dealt combat damage
    // to a player this turn" — per-creature combat-damage-dealt-this-turn
    // tracking not available in the script API.
    Vec::new()
}
