//! Avabruck Caretaker // Hollowhenge Huntmaster
//!
//! Front: Creature — Human Werewolf {4}{G}{G}, 4/4
//! Hexproof.
//! At the beginning of combat on your turn, put two +1/+1 counters on another target creature
//! you control.
//! Daybound (GAP: day/night cycle not modeled; transform trigger GAP-ed.)
//!
//! Back: Creature — Werewolf
//! Hexproof.
//! Other permanents you control have hexproof. (GAP: static ability grant not modeled.)
//! At the beginning of combat on your turn, put two +1/+1 counters on each creature you control.
//! Nightbound (GAP: night/day cycle not modeled.)
//! GAP: back-face-only triggered ability (each creature) not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter, TargetRequirement, TargetChoice, TargetCount};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Avabruck Caretaker");

    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hollowhenge Huntmaster");
    let werewolf2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![KeywordAbility::Hexproof],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: At the beginning of combat on your turn, put two +1/+1 counters on
            // another target creature you control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_combat_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // GAP: Daybound/Nightbound transform condition not modeled.
            // GAP: back-face-only triggered ability (combat trigger for each creature) not modeled.
            // GAP: back-face static "other permanents have hexproof" not modeled.
    )
}

fn front_combat_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
    ]
}
