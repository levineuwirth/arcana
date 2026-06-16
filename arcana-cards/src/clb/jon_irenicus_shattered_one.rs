//! Jon Irenicus, Shattered One — `{2}{U}{B}` 3/3 Legendary Elf Wizard.
//! At the beginning of your end step, target opponent gains control of up to
//! one target creature you control; put two +1/+1 counters on it, tap it,
//! goad it, and it gains "can't be sacrificed". Whenever a creature you own
//! but don't control attacks, you draw a card.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jon Irenicus, Shattered One");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // Goad is an effect-driven mechanic, not a base keyword on this card.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_donate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // "a creature you own but don't control attacks" — the
                // ownership/controllership split isn't expressible in the
                // CreatureAttacks filter, so this fires for any opponent's
                // attacking creature (closest available filter).
                // GAP: "you own but don't control" restriction.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_donate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut new_controller = None;
    let mut creature = None;
    for t in &trig.targets.targets {
        match t {
            TargetChoice::Player(p) => new_controller = Some(*p),
            TargetChoice::Object(id) => creature = Some(*id),
            _ => {}
        }
    }
    let (Some(p), Some(id)) = (new_controller, creature) else { return Vec::new(); };
    vec![
        Effect::ChangeControl { target: id, new_controller: p },
        Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 2 },
        Effect::Tap { target: id },
        // "Goaded for the rest of the game" — engine Goad is until-EOT; this is
        // the closest expressible duration.
        Effect::Goad { target: id, goader: trig.controller, duration: Duration::EndOfTurn },
        // GAP: grant "This creature can't be sacrificed" — no such ability grant.
    ]
}

fn draw_a_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
