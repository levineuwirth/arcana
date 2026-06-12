//! Ulrich of the Krallenhorde // Ulrich, Uncontested Alpha
//! `{3}{R}{G}` Legendary Creature — Human Werewolf 4/4
//!
//! Front: Whenever this creature enters or transforms into Ulrich of the
//! Krallenhorde, target creature gets +4/+4 until end of turn.
//! At the beginning of each upkeep, if no spells were cast last turn,
//! transform Ulrich of the Krallenhorde.
//!
//! Back (Legendary Creature — Werewolf 6/6): Whenever this creature
//! transforms into Ulrich, Uncontested Alpha, you may have it fight
//! target non-Werewolf creature you don't control.
//! At the beginning of each upkeep, if a player cast two or more spells
//! last turn, transform Ulrich.
//!
//! GAP: "you may have it fight" — the optional fight is modeled via the
//!      up-to-one target count (declining = no target).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulrich of the Krallenhorde");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Back face: Ulrich, Uncontested Alpha
    let back_name = reg.interner_mut().intern("Ulrich, Uncontested Alpha");
    let werewolf_sub2 = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub2);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            mana_cost: None,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // "Whenever this creature enters or transforms into Ulrich of the
            // Krallenhorde, target creature gets +4/+4 until end of turn."
            // Enters half:
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: front_pump_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            // Transforms-into-front half:
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(0) },
                intervening_if: None,
                effect: front_pump_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            // Back: "Whenever this creature transforms into Ulrich, Uncontested
            // Alpha, you may have it fight target non-Werewolf creature you
            // don't control." ("you may" via up-to-one target.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: back_fight_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent)
                            .without_subtype_sym(werewolf_sub),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // Front: at the beginning of each upkeep, if no spells were cast
            // last turn, transform Ulrich.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_no_spells_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back: at the beginning of each upkeep, if a player cast two or
            // more spells last turn, transform Ulrich.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_player_cast_two_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 4 fires only on the front face; trigger 5 only on the back.
            .with_trigger_face_gate(4, 0)
            .with_trigger_face_gate(5, 1),
    )
}

fn front_pump_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 4,
        toughness: 4,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn back_fight_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Fight { a: trig.source, b: *id }]
}

fn if_no_spells_last_turn(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::no_spells_cast_last_turn(s)
}

fn if_player_cast_two_last_turn(
    s: &GameState,
    _src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(s)
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
