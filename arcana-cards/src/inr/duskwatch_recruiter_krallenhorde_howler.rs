//! Duskwatch Recruiter // Krallenhorde Howler — `{1}{G}` 2/2 green Human Warrior Werewolf.
//!
//! Front face (Duskwatch Recruiter):
//!   {2}{G}: Look at top 3 cards, may reveal a creature card and put it in hand,
//!   put the rest on the bottom in any order.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//!
//! Back face (Krallenhorde Howler):
//!   Creature spells you cast cost {1} less to cast. (GAP: cost-reduction static not modeled.)
//!   At the beginning of each upkeep, if a player cast two or more spells last turn, transform.
//!
//! GAP: "if no spells were cast last turn" / "if a player cast two or more spells last turn" —
//!   day/night transform conditions not modeled; triggers fire unconditionally.
//! GAP: Krallenhorde Howler cost-reduction static ability not modeled (continuous effect layer).
//! GAP: Back-face-only triggered ability (transform back) not auto-installed on transform.

use arcana_core::conditions;
use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duskwatch Recruiter");
    let human_sub = reg.interner_mut().intern("Human");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(warrior_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Krallenhorde Howler");
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
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face activated ability: {2}{G}: DigTopN 3, may take a creature card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Look at the top three cards of your library. You may reveal a creature card from among them and put it into your hand. Put the rest on the bottom of your library in any order.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: dig_top_three,
            })
            // Front-face upkeep trigger: transform if no spells were cast last turn.
            // Intervening-if modeled via conditions::no_spells_cast_last_turn.
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
            // Back-face upkeep trigger: transform back if a player cast 2+ spells last turn.
            // Intervening-if modeled via conditions::a_player_cast_two_or_more_last_turn.
            // GAP: back-face-only triggered ability not modeled correctly; emitted for structure.
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
    )
}

fn dig_top_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 3,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}

fn iif_no_spells(state: &GameState, _source: ObjectId, _you: PlayerId) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn iif_two_or_more(state: &GameState, _source: ObjectId, _you: PlayerId) -> bool {
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
    // GAP: should only fire if a player cast two or more spells last turn (day/night condition)
    // GAP: back-face-only triggered ability not auto-installed on transform
    vec![Effect::Transform { target: trig.source }]
}
