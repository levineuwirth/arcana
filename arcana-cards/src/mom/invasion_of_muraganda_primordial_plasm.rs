//! Invasion of Muraganda // Primordial Plasm — `{4}{G}` green Battle — Siege (front face).
//! Enters with defense counters. When this Siege enters, put a +1/+1 counter on target
//! creature you control. Then that creature fights up to one target creature you don't control.
//! Back face: Primordial Plasm, Creature — Ooze. At the beginning of combat on your turn,
//! another target creature gets +2/+2 and loses all abilities until end of turn.
//!
//! Defeat→back-face is auto-wired by the engine SBA. Front ETB (counter + fight)
//! is face-gated to the battle face (0); the back-face beginning-of-combat
//! ability is face-gated to the creature face (1).
//!
//! # GAPs
//! - "fights up to one target creature you don't control" — modeled as a fixed-1 fight when a
//!   second target is chosen; if none chosen the fight is skipped (counter still applied).
//! - Back-face "another target creature" — the engine has no exclude-source
//!   target restriction, so the trigger targets any creature (the "another"
//!   clause is not strictly enforced; the back face is a 0/0 creature itself).
//! - Siege protector-designation simplified (any opponent may attack it).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
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
    let name = reg.interner_mut().intern("Invasion of Muraganda");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face: Primordial Plasm — Creature — Ooze.
    let back_name = reg.interner_mut().intern("Primordial Plasm");
    let ooze_sub = reg.interner_mut().intern("Ooze");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(ooze_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            ..Default::default()
        },
        spell_ability: None,
    };
    // GAP: back-face combat trigger ("+2/+2 and loses all abilities") not wired through CardFace.

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 3,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counter_fight,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            })
            // Back (Primordial Plasm): at the beginning of combat on your turn,
            // another target creature gets +2/+2 and loses all abilities until
            // end of turn. Face-gated to the creature face (1).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_combat_buff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn back_combat_buff(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    vec![
        Effect::Pump {
            target: id,
            power: 2,
            toughness: 2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::LoseAllAbilities {
            target: id,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn etb_counter_fight(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(t1) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(mine) = t1 else { return Vec::new(); };
    let mine = *mine;
    let mut effects = vec![Effect::AddCounters {
        target: mine,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if let Some(TargetChoice::Object(theirs)) = trig.targets.targets.get(1) {
        effects.push(Effect::Fight { a: mine, b: *theirs });
    }
    effects
}
