//! Afflicted Deserter // Werewolf Ransacker (transforming DFC, layout "transform")
//!
//! Front face: Afflicted Deserter — {3}{R} Creature — Human Werewolf, 3/2.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform this creature.
//! Back face: Werewolf Ransacker — Creature — Werewolf, 5/4 (printed stats).
//!   At the beginning of each upkeep, if a player cast two or more spells last turn,
//!     transform this creature.
//!
//! GAP: the destroy half of "Whenever this creature transforms into Werewolf Ransacker,
//! you may destroy target artifact" is wired; the conditional rider "If that artifact is
//! put into a graveyard this way, this creature deals 3 damage to that artifact's
//! controller" is not expressible (no destroyed-this-way linkage) and is omitted.

use arcana_core::conditions;
use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Afflicted Deserter");
    let human = reg.interner_mut().intern("Human");
    let werewolf = reg.interner_mut().intern("Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Werewolf Ransacker");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face werewolf transform: at the beginning of each upkeep,
            // if no spells were cast last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
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
            // Back-face werewolf transform: at the beginning of each upkeep,
            // if a player cast two or more spells last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
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
            // "Whenever this creature transforms into Werewolf Ransacker, you may
            // destroy target artifact." ("you may" via up-to-one target.)
            // GAP: the "deals 3 damage to that artifact's controller if it died
            // this way" rider is not expressible; only the destroy half is wired.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_smash_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // Trigger 1 fires only on the front face; trigger 2 only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn if_no_spells_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(s)
}

fn if_player_cast_two_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(s)
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform {
        target: trig.source,
    }]
}

fn on_transform_smash_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "If that artifact is put into a graveyard this way, this creature
    // deals 3 damage to that artifact's controller" — conditional rider omitted.
    vec![Effect::DestroyPermanent { target: *id }]
}
