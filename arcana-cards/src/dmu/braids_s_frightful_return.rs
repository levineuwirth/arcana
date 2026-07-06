//! Braids's Frightful Return — `{2}{B}` Enchantment — Saga
//!
//! Read ahead — GAP: not modeled; using standard saga starting at chapter I.
//!
//! I — You may sacrifice a creature. If you do, each opponent discards a card.
//!     (Wired as an OptionalPayment: pay = sacrifice a creature → each opponent
//!     discards 1; decline does nothing.)
//! II — Return target creature card from your graveyard to your hand.
//! III — Target opponent may sacrifice a nonland, nontoken permanent. If they
//!        don't, they lose 2 life and you draw a card.
//!        (Wired as an OptionalPayment whose chooser is the targeted opponent:
//!        pay = sacrifice a nonland permanent → nothing more; decline = they lose
//!        2 life and you draw. Fidelity note: SacrificeFilter::NonLand can't add
//!        the "nontoken" constraint, so a token could satisfy the payment.)

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Braids's Frightful Return");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Lore, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::PreCombatMain, whose: arcana_core::targets::ControllerConstraint::You },
                intervening_if: None, effect: add_lore_counter,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(1) },
                intervening_if: None, effect: chapter_i,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(2) },
                intervening_if: None, effect: chapter_ii,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: arcana_core::targets::ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4, trigger_condition: TriggerCondition::CounterAdded { on: TriggerSelf::Source, kind: Some(CounterKind::Lore), chapter: Some(3) },
                intervening_if: None, effect: chapter_iii,
                trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_opponent()],
            }),
    )
}

fn add_lore_counter(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(state: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    // "You may sacrifice a creature. If you do, each opponent discards a card."
    let discards: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::Discard { player: p, count: 1, choice: DiscardChoice::ControllerChooses })
        .collect();
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Creature),
        then: Box::new(Effect::Sequence(discards)),
        else_effect: None,
    }]
}

fn chapter_ii(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}

fn chapter_iii(_s: &GameState, trig: &PendingTrigger, _r: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else { return Vec::new(); };
    // "Target opponent may sacrifice a nonland, nontoken permanent. If they
    // don't, they lose 2 life and you draw a card." The opponent chooses: pay
    // (sacrifice a nonland permanent) avoids the penalty; decline triggers it.
    // (Nontoken can't be added to SacrificeFilter::NonLand — minor over-inclusion.)
    vec![Effect::OptionalPayment {
        chooser: *p,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::NonLand),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::Sequence(vec![
            Effect::LoseLife { player: *p, amount: 2 },
            Effect::DrawCards { player: trig.controller, count: 1 },
        ]))),
    }]
}
