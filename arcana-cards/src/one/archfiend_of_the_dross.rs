//! Archfiend of the Dross — `{2}{B}{B}` 6/6 Phyrexian Demon with Flying.
//!
//! Flying
//! This creature enters with four oil counters on it.
//! At the beginning of your upkeep, remove an oil counter from this
//! creature. Then if it has no oil counters on it, you lose the game.
//!   (the "you lose the game" rider is GAP'd — no lose-game Effect.)
//! Whenever a creature an opponent controls dies, its controller
//! loses 2 life.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archfiend of the Dross");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let demon = reg.interner_mut().intern("Demon");
    let _oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enter_with_oil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_remove_oil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: opponent_creature_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn oil_kind(reg: &CardRegistry) -> Option<CounterKind> {
    reg.interner().lookup("oil").map(CounterKind::Named)
}

fn enter_with_oil(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = oil_kind(reg) else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 4,
    }]
}

fn upkeep_remove_oil(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = oil_kind(reg) else { return Vec::new(); };
    // GAP: "then if it has no oil counters on it, you lose the game" —
    // no lose-the-game Effect in the demonstrated catalog.
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}

fn opponent_creature_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dead: ObjectId = trig.dying_object().unwrap_or(trig.source);
    let controller = script::target_controller(state, dead, trig.controller);
    vec![Effect::LoseLife {
        player: controller,
        amount: 2,
    }]
}
