//! The Huntsman's Redemption — `{2}{G}` green Enchantment — Saga.
//! I — Create a 3/3 green Beast creature token.
//! II — You may sacrifice a creature. If you do, search your library for a creature or basic land card, reveal it, put to hand.
//! III — Up to two target creatures each get +2/+2 and gain trample until end of turn.
//! NOTE: Chapter II's search filter "creature or basic land" is approximated as
//! creature-or-land (over-includes nonbasic lands) — a single ObjectFilter can't
//! OR a type-only with a type+supertype clause.
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Huntsman's Redemption");
    let saga_sub = reg.interner_mut().intern("Saga");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::ENCHANTMENT.into(), subtypes, supertypes: SupertypeSet::default(), ..Default::default() };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Lore, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::PreCombatMain, whose: ControllerConstraint::You }, intervening_if: None, effect: add_lore_counter, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 2, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(1) }, intervening_if: None, effect: chapter_i, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) }, intervening_if: None, effect: chapter_ii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new() })
            .with_triggered_ability(TriggeredAbilityDef { id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) }, intervening_if: None, effect: chapter_iii, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement { filter: TargetFilter::Creature, count: TargetCount::UpTo(2), controller: None }] }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("interned at register");
    let mut ts = SubtypeSet::default();
    ts.0.insert(beast);
    let token = TokenDefinition { name: beast, colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes: ts, power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(3)), keywords: vec![], abilities: vec![] };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // "you may sacrifice a creature. If you do, search your library for a
    // creature or basic land card, reveal it, put to hand." Optional Sacrifice
    // payment; the tutor `then` runs only after the sacrifice resolves.
    let search_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::LAND));
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Creature),
        then: Box::new(Effect::TutorToHand {
            player: trig.controller,
            filter: search_filter,
            reveal: true,
        }),
        else_effect: None,
    }]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    trig.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            Some(Effect::Pump { target: *id, power: 2, toughness: 2, duration: Duration::EndOfTurn, keywords: vec![KeywordAbility::Trample] })
        } else { None }
    }).collect()
}
