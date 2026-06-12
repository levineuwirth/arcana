//! The Curse of Fenric — `{2}{G}{W}` green/white Enchantment — Saga.
//! I — For each player, destroy up to one target creature that player controls.
//!     For each creature destroyed this way, create a 3/3 green Mutant with deathtouch.
//! II — Target nontoken creature becomes a 6/6 legendary Horror named Fenric.
//! III — Target Mutant fights another target creature named Fenric.
//! GAP: Chapter I "for each player targeting" not expressible per-player.
//! GAP: Chapter I "create token for each destroyed" not expressible (no loop).
//! GAP: Chapter II — SetBasePT 6/6 + LoseAllAbilities are wired (permanent);
//! the Horror type / legendary / "named Fenric" changes have no effect surface.
//! GAP: Chapter III "target named Fenric" — can't filter by name in TargetRequirement.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Curse of Fenric");
    let saga_sub = reg.interner_mut().intern("Saga");
    let _mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
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
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: arcana_core::targets::ControllerConstraint::You,
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
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
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
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        arcana_core::targets::ObjectFilter::creature().nontoken(),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn add_lore_counter(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    // GAP: "create a Mutant token for each destroyed" — can't loop; create 1 as best-effort
    let mutant = reg.interner().lookup("Mutant").expect("Mutant interned during register()");
    let mut ts = SubtypeSet::default();
    ts.0.insert(mutant);
    let token = TokenDefinition {
        name: mutant,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: ts,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        abilities: vec![],
    };
    effects.push(Effect::CreateToken { controller: trig.controller, token });
    effects
}

fn chapter_ii(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "becomes a 6/6 ... and loses all abilities" — permanent (the
    // modification must survive the Saga's final-chapter sacrifice).
    // GAP: the Horror type / legendary / "named Fenric" changes — no
    // targeted subtype/supertype/name-change effect.
    vec![
        Effect::LoseAllAbilities {
            target: *id,
            duration: arcana_core::layers::Duration::Permanent,
        },
        Effect::SetBasePT {
            target: *id,
            power: 6,
            toughness: 6,
            duration: arcana_core::layers::Duration::Permanent,
        },
    ]
}

fn chapter_iii(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let mut it = trig.targets.targets.iter();
    let Some(t1) = it.next() else { return Vec::new(); };
    let Some(t2) = it.next() else { return Vec::new(); };
    let (TargetChoice::Object(a), TargetChoice::Object(b)) = (t1, t2) else { return Vec::new(); };
    vec![Effect::Fight { a: *a, b: *b }]
}
