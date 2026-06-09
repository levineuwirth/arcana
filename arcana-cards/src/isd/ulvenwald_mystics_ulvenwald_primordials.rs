//! Ulvenwald Mystics // Ulvenwald Primordials — {2}{G}{G} 3/3 green Creature — Human Shaman Werewolf.
//!
//! Front face (Ulvenwald Mystics):
//!   At the beginning of each upkeep, if no spells were cast last turn, transform this creature.
//!
//! Back face (Ulvenwald Primordials — Creature — Werewolf):
//!   {G}: Regenerate this creature.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn,
//!   transform this creature.
//!
//! GAP: "if no spells were cast last turn" — last-turn spell count is not accessible
//!      via the script API (spells_cast_this_turn tracks the CURRENT turn, not last turn).
//!      Front-face transform trigger is GAP'd (trigger registered but fires unconditionally
//!      every upkeep — the verify pipeline will quarantine this as incorrect behavior).
//! GAP: "if a player cast two or more spells last turn" — same constraint as above.
//!      Back-face transform trigger is similarly not accurately modelable.
//! GAP: Back-face-only triggered ability ({G}: Regenerate and back-face upkeep trigger)
//!      not auto-installed on transform. Regenerate modeled as an ActivatedAbilityDef
//!      (shared across faces since abilities live on CardDefinition, not faces).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulvenwald Mystics");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ulvenwald Primordials");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face upkeep trigger — transform if no spells were cast last turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face upkeep trigger — transform if a player cast 2+ spells last turn.
            // GAP: This should only fire on the back face; face-gate not available for triggered abilities.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_two_or_more),
                effect: back_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {G}: Regenerate — modeled as activated ability (technically back-face only,
            // but abilities live on CardDefinition; GAP: fires on front face too).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: Regenerate Ulvenwald Primordials.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1), // back face only
                effect: do_regenerate,
            })
    )
}

fn iif_no_spells(state: &GameState, _s: ObjectId, _y: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn iif_two_or_more(state: &GameState, _s: ObjectId, _y: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(state)
}

fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn back_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn do_regenerate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
