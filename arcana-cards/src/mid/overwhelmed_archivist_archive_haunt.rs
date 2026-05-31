//! Overwhelmed Archivist // Archive Haunt — {2}{U} Creature — Human Wizard (3/2),
//! transforming DFC.
//!
//! Front (Overwhelmed Archivist):
//!   When this creature enters, draw a card, then discard a card.
//!   Disturb {3}{U}.
//! Back (Archive Haunt): Creature — Spirit Wizard (3/2)
//!   Flying
//!   Whenever this creature attacks, draw a card, then discard a card.
//!   If Archive Haunt would be put into a graveyard from anywhere, exile it instead.
//!
//! GAP: Disturb {3}{U} (cast from graveyard transformed for its disturb cost) is not
//! an expressible keyword/ability in this engine — no Disturb cast modifier. Recorded
//! in this comment; not emitted as a keyword.
//! GAP: back-face "if Archive Haunt would be put into a graveyard from anywhere, exile
//! it instead" is a replacement effect with no matching primitive here; not authored.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Overwhelmed Archivist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Archive Haunt — Creature — Spirit Wizard, 3/2, Flying.
    let back_name = reg.interner_mut().intern("Archive Haunt");
    let spirit = reg.interner_mut().intern("Spirit");
    let wizard_b = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit);
    back_subtypes.0.insert(wizard_b);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-only: ETB draw a card, then discard a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: draw_then_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-only: whenever this creature attacks, draw a card, then discard a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: draw_then_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn draw_then_discard(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
