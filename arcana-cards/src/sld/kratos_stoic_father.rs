//! Kratos, Stoic Father — `{2}{R}{W}` 4/4 Legendary God Warrior (R/W).
//!
//! Oracle:
//! * "Whenever you attack with one or more Gods and whenever a God dies, you
//!    get an experience counter." — wired as two triggers (a God you control
//!    attacks; a God dies). GAP: "you get an experience counter" — there is no
//!    primitive to add a counter to a PLAYER (AddCounters targets an object).
//! * "At the beginning of your end step, put a number of +1/+1 counters on
//!    target creature equal to the number of experience counters you have." —
//!    wired as an end-step trigger targeting a creature. GAP: a player's
//!    experience-counter count is not readable, so the amount is uncomputable.
//! * Partner—Father & son — Partner is not a `KeywordAbility` variant; GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kratos, Stoic Father");
    let god = reg.interner_mut().intern("God");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    subtypes.0.insert(warrior);

    let god_attack = script::subtype_filter(reg, "God").controlled_by(ControllerConstraint::You);
    let god_dies = script::subtype_filter(reg, "God");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Partner—Father & son — Partner has no KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks { filter: god_attack },
                intervening_if: None,
                effect: gain_experience,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: god_dies,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: gain_experience,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn gain_experience(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you get an experience counter" — no primitive adds a counter to a
    //      player.
    Vec::new()
}

fn end_step_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: amount = "number of experience counters you have" — a player's
    //      experience-counter count is not readable; the amount is uncomputable.
    Vec::new()
}
