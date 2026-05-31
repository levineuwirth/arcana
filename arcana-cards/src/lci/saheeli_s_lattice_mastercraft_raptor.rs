//! Saheeli's Lattice // Mastercraft Raptor — transforming double-faced card.
//!
//! Front (Saheeli's Lattice, {1}{R} Artifact):
//!   When this artifact enters, you may discard a card. If you do, draw two cards.
//!   Craft with one or more Dinosaurs {4}{R} — transforms to the back face.
//! Back (Mastercraft Raptor, Artifact Creature — Dinosaur):
//!   Power equal to the total power of the exiled cards used to craft it.
//!
//! GAP: Craft ({4}{R}, exile this + exile Dinosaurs you control / Dinosaur cards
//! from graveyard: return transformed) is not an expressible activated-ability shape
//! (exile-creatures-as-cost + exile-self + transform-on-return). The transform back
//! face is declared so the catalog records both faces.
//! GAP: Mastercraft Raptor's power = total power of the exiled craft cards is dynamic
//! and depends on the (unmodeled) craft exile set — emitted as a placeholder Fixed P/T.
//! GAP: the ETB "you may discard" optionality is not an OptionalPayment kind (no
//! Discard payment); modeled best-effort as discard-then-draw.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::mana::ManaCost;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saheeli's Lattice");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Mastercraft Raptor");
    let dino_sub = reg.interner_mut().intern("Dinosaur");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dino_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            // GAP: power = total power of exiled craft cards (dynamic); placeholder.
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: arcana_core::targets::ObjectFilter::default(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: etb_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_loot(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "You may discard a card. If you do, draw two cards."
    // GAP: optionality of the discard is not modeled (no Discard OptionalPayment).
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 2,
        },
    ])]
}
