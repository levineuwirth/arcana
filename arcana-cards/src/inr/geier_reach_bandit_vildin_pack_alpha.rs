//! Geier Reach Bandit // Vildin-Pack Alpha — `{2}{R}` Human Rogue Werewolf 3/2.
//! Front face: Haste. At the beginning of each upkeep, if no spells were cast
//! last turn, transform this creature.
//! Back face (Vildin-Pack Alpha): Whenever a Werewolf you control enters, you
//! may transform it. At the beginning of each upkeep, if a player cast two or
//! more spells last turn, transform this creature.
//!
//! The werewolf transform conditions ("no spells cast last turn" / "two or more
//! spells last turn") are modeled as intervening-if predicates
//! (conditions::no_spells_cast_last_turn / conditions::a_player_cast_two_or_more_last_turn).
//! The front upkeep trigger is gated to face 0; the back upkeep transform and the
//! back "Werewolf you control enters" trigger are gated to face 1
//! (with_trigger_face_gate). The "you may transform it" is modeled as an
//! unconditional transform of the entering creature (beneficial may).

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Geier Reach Bandit");
    let human_sub = reg.interner_mut().intern("Human");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(rogue_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Vildin-Pack Alpha");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    // "Whenever a Werewolf you control enters" — built before reg.register so the
    // immutable interner lookup doesn't overlap reg's mutable borrow.
    let werewolf_enters_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(werewolf_back_sub);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: at beginning of each upkeep, if no spells were cast last
            // turn, transform. Intervening-if via conditions::no_spells_cast_last_turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: at beginning of each upkeep, if a player cast two or more
            // spells last turn, transform. Intervening-if via
            // conditions::a_player_cast_two_or_more_last_turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_two_or_more),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face (Vildin-Pack Alpha): whenever a Werewolf you control enters,
            // you may transform it (modeled as an unconditional transform of the
            // entering creature).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: werewolf_enters_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: transform_entering_werewolf,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 fires only on the front face; triggers 2 & 3 only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1),
    )
}

fn iif_no_spells(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn iif_two_or_more(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(state)
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

/// Transform the Werewolf that just entered (the "it" in "you may transform it").
fn transform_entering_werewolf(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    match trig.entering_object() {
        Some(id) => vec![Effect::Transform { target: id }],
        None => Vec::new(),
    }
}
