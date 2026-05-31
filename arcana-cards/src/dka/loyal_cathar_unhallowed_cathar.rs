//! Loyal Cathar // Unhallowed Cathar — `{W}{W}` Creature — Human Soldier 2/2.
//!
//! Front face — Loyal Cathar:
//!   Vigilance.
//!   When this creature dies, return it to the battlefield transformed under your
//!     control at the beginning of the next end step.
//!
//! Back face — Unhallowed Cathar — Creature — Zombie Soldier 2/2:
//!   This creature can't block.
//!
//! # GAP
//! - The dies-return is a delayed "at the beginning of the next end step" return.
//!   DelayedAction has no "return from graveyard to battlefield (transformed)"
//!   variant, so this returns immediately (no end-step delay) via
//!   ReturnFromGraveyardToBattlefield + Transform. (Same posture as Phytotitan.)
//! - Back-face static "this creature can't block" is a continuous self-restriction
//!   active only on the back face; there is no static can't-block characteristic
//!   field, only the targeted Effect::ForbidBlocking. Documented as a GAP.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Loyal Cathar");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let zombie = reg.interner_mut().intern("Zombie");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // Back face: Unhallowed Cathar — Zombie Soldier 2/2.
    let back_name = reg.interner_mut().intern("Unhallowed Cathar");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(zombie);
    back_subtypes.0.insert(soldier);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            // GAP: back-face static "this creature can't block" not modeled
            // (only targeted Effect::ForbidBlocking exists; no static field).
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: when this dies, return it transformed.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies_return_transformed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 belongs to the front face only.
            .with_trigger_face_gate(1, 0),
    )
}

fn on_dies_return_transformed(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dying = trig.dying_object().unwrap_or(trig.source);
    // GAP: "at the beginning of the next end step" delayed return from graveyard
    // not supported (DelayedAction has no graveyard-return). Returning immediately,
    // transformed.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: dying },
        Effect::Transform { target: dying },
    ]
}
