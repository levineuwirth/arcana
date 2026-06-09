//! Sage of Ancient Lore // Werewolf of Ancient Hunger — `{4}{G}` green
//! Legendary Creature — Human Shaman Werewolf (*/*) / Legendary Creature — Werewolf.
//!
//! Front face (Sage of Ancient Lore): */* (power/toughness = cards in your hand)
//!   Vigilance. (GAP: Vigilance listed as keyword but not on front face in oracle)
//!   Sage of Ancient Lore's power and toughness are each equal to the number of
//!   cards in your hand.
//!   When this creature enters, draw a card.
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//!
//! Back face (Werewolf of Ancient Hunger):
//!   Vigilance, trample.
//!   Power and toughness equal to total cards in all players' hands.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn,
//!   transform this creature.
//!
//! GAP: */* P/T — dynamic power/toughness based on hand size is a characteristic
//!      replacement effect, not expressible in Characteristics; using PtValue::Fixed(0)
//!      as placeholder (the actual * value is engine debt).
//! GAP: Back-face P/T equal to total cards in all players' hands — same gap.
//! Front-face werewolf transform condition ("if no spells were cast last turn") is
//!      modeled via `conditions::no_spells_cast_last_turn` on the upkeep trigger's
//!      `intervening_if`.
//! GAP: Back-face-only triggered ability (upkeep back-transform) not auto-installed
//!      on transform.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sage of Ancient Lore");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: */* P/T — dynamic hand size not expressible; placeholder Fixed(0).
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Werewolf of Ancient Hunger");
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
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // GAP: P/T = total cards in all players' hands — dynamic; placeholder Fixed(0).
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Front: at beginning of each upkeep, if no spells were cast last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells_last_turn),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
        // GAP: back-face-only triggered ability not modeled:
        //   "At the beginning of each upkeep, if a player cast two or more spells
        //    last turn, transform this creature." (back to front)
    )
}

fn etb_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn iif_no_spells_last_turn(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Front-to-back transform; gated by intervening_if (no spells cast last turn).
    vec![Effect::Transform { target: trig.source }]
}
