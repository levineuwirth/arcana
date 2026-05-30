//! Invasion of Azgol // Ashen Reaper — `{B}{R}` Battle — Siege.
//! Front: Enters with 3 defense counters.
//!   When this Siege enters, target player sacrifices a creature or planeswalker
//!   of their choice and loses 1 life.
//! Back (Ashen Reaper, 2/2 Zombie Elemental): Menace.
//!   At the beginning of your end step, put a +1/+1 counter on this creature if
//!   a permanent was put into a graveyard from the battlefield this turn.
//!
//! GAP: Front ETB "target player sacrifices a creature or planeswalker of THEIR
//!      choice" — Effect::Sacrifice has the player pick; filter includes creatures
//!      and planeswalkers (best-effort). "of their choice" is modeled by Effect::Sacrifice
//!      which lets the player choose.
//! GAP: Back trigger condition "if a permanent was put into a graveyard from the
//!      battlefield this turn" — no per-turn zone-change counting in the engine;
//!      trigger fires unconditionally at end step.
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11).
//! GAP: back-face-only triggered ability not auto-installed on transform.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Azgol");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ashen Reaper");
    let zombie_sub = reg.interner_mut().intern("Zombie");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(zombie_sub);
    back_subtypes.0.insert(elemental_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Menace],
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 3,
            })
            .with_transform_back(back)
            // When this Siege enters, target player sacrifices a creature or planeswalker and loses 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Back face: at beginning of your end step, put a +1/+1 counter.
            // GAP: condition "if a permanent was put into a graveyard this turn" not modeled;
            //      fires unconditionally.
            // GAP: back-face-only triggered ability not auto-installed on transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_end_step_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn etb_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // Target player sacrifices a creature or planeswalker of their choice.
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER));
    vec![
        Effect::Sacrifice { player: *p, filter, count: 1 },
        Effect::LoseLife { player: *p, amount: 1 },
    ]
}

fn back_end_step_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only fire if a permanent went to graveyard from battlefield this turn.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
