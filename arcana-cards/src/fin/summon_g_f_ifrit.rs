//! Summon: G.F. Ifrit — `{2}{R}` red Enchantment Creature — Saga Demon.
//! 3/2. 4 chapters.
//! I, II — You may discard a card. If you do, draw a card.
//! III, IV — Add {R}.
//! Chapter I/II "you may discard a card. If you do, draw a card" is wired via an
//! optional discard payment. Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Summon: G.F. Ifrit");
    let saga_sub = reg.interner_mut().intern("Saga");
    let demon_sub = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    subtypes.0.insert(demon_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
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
                effect: chapter_i_ii,
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
                effect: chapter_i_ii,
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
                effect: chapter_iii_iv,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(4),
                },
                intervening_if: None,
                effect: chapter_iii_iv,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
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

fn chapter_i_ii(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "You may discard a card. If you do, draw a card."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}

fn chapter_iii_iv(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}
