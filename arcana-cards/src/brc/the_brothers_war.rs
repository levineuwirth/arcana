//! The Brothers' War — `{3}{R}` red Enchantment — Saga.
//! I — Create two tapped Powerstone tokens.
//! II — Choose two target players. Until your next turn, each creature they control attacks the other chosen player each combat if able.
//! III — Deal X damage to any target and X damage to any other target, where X = number of artifacts you control.
//! GAP: Chapter II "attacks the other chosen player each combat if able" — targeted-attack-direction not in catalog.
//! GAP: Chapter III "X = number of artifacts you control" — second target for damage not in single-target damage.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Brothers' War");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                target_requirements: vec![
                    TargetRequirement::target_player(),
                    TargetRequirement::target_player(),
                ],
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
                    TargetRequirement::any_target(),
                    TargetRequirement::any_target(),
                ],
            }),
    )
}

fn add_lore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateCommodityToken { controller: trig.controller, kind: CommodityToken::Powerstone, count: 1 },
        Effect::CreateCommodityToken { controller: trig.controller, kind: CommodityToken::Powerstone, count: 1 },
    ]
}

fn chapter_ii(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each creature they control attacks the other chosen player" — targeted-attack-direction not in catalog
    Vec::new()
}

fn chapter_iii(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()).controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut effects = Vec::new();
    for (i, target) in trig.targets.targets.iter().enumerate() {
        if i >= 2 { break; }
        match target {
            TargetChoice::Object(id) => {
                effects.push(Effect::DealDamage {
                    target: DamageTarget::Object(*id),
                    amount: x,
                    source: trig.source,
                });
            }
            TargetChoice::Player(p) => {
                effects.push(Effect::DealDamage {
                    target: DamageTarget::Player(*p),
                    amount: x,
                    source: trig.source,
                });
            }
            _ => {}
        }
    }
    effects
}
