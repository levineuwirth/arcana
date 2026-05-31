//! Visage of Dread // Dread Osseosaur — `{1}{B}` Artifact.
//!
//! Front face (Visage of Dread — Artifact):
//! - When this artifact enters, target opponent reveals their hand. You choose an
//!   artifact or creature card from it. That player discards that card.
//! - Craft with two creatures {5}{B} — exile this + two creatures to return transformed.
//!
//! Back face (Dread Osseosaur — Creature — Dinosaur Skeleton Horror, 5/5):
//! - Menace
//! - Whenever this creature enters or attacks, you may mill two cards.
//!
//! GAP: Craft (CR 702.165) is not modeled — the transform-via-exile-cost activated ability
//! is engine debt; the back face is declared so the transform target exists, but the Craft
//! activation itself is omitted.
//! GAP: ETB discard fidelity — "reveals their hand" and the "artifact or creature card"
//! restriction on the chosen card are not expressible; modeled as a targeted directed discard
//! (the discarding player's opponent — i.e. you — chooses the card).
//! GAP: "you may mill two" — the optional "may" on the mill is a resolution-time choice not
//! modeled; emitted as a straight mill 2.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Visage of Dread");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    // Back face — Dread Osseosaur (Creature — Dinosaur Skeleton Horror, 5/5)
    let back_name = reg.interner_mut().intern("Dread Osseosaur");
    let dinosaur_sub = reg.interner_mut().intern("Dinosaur");
    let skeleton_sub = reg.interner_mut().intern("Skeleton");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dinosaur_sub);
    back_subtypes.0.insert(skeleton_sub);
    back_subtypes.0.insert(horror_sub);

    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: back_subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back_face)
            // Front-face trigger (id 1): ETB target opponent discards a chosen card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            // Back-face trigger (id 2): enters -> may mill two.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: mill_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face trigger (id 3): attacks -> may mill two.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: mill_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1),
    )
}

fn etb_discard(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // GAP: hand-reveal + "artifact or creature card" restriction not expressible.
    // The discarding player's opponent (you) chooses the card via OpponentChooses.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::OpponentChooses,
    }]
}

fn mill_two(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill {
        player: trig.controller,
        count: 2,
    }]
}
