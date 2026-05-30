//! Biolume Egg // Biolume Serpent — `{2}{U}` Serpent Egg creature 0/4 (front) with Defender.
//! Front: Defender. When this creature enters, scry 2.
//! When you sacrifice this creature, return it to the battlefield transformed under its
//! owner's control at the beginning of the next end step.
//! Back: Serpent — activated ability: Sacrifice two Islands: This creature can't be blocked
//! this turn.
//!
//! GAP: "When you sacrifice this creature, return it to the battlefield transformed" —
//! The Sacrificed trigger condition fires when the player sacrifices a permanent, but
//! the subsequent "exile + return transformed at end step" chain is not expressible with
//! a single DelayedAction. Approximated: fire Transform immediately on sacrifice trigger.
//! GAP: "Sacrifice two Islands" as activation cost — sacrifice_other filter supports
//! sacrificing permanents but sacrificing two Islands specifically is not modeled with count.
//! The back-face activated ability is omitted.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Biolume Egg");
    let serpent_sub = reg.interner_mut().intern("Serpent");
    let egg_sub = reg.interner_mut().intern("Egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent_sub);
    subtypes.0.insert(egg_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Defender],
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Biolume Serpent");
    let mut back_subtypes = SubtypeSet::default();
    let back_serpent_sub = reg.interner_mut().intern("Serpent");
    back_subtypes.0.insert(back_serpent_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Triggered ability 1: ETB — scry 2
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Triggered ability 2: When you sacrifice this creature (approximated via Sacrificed
            // trigger), apply transform.
            // GAP: The oracle says "return it to the battlefield transformed under its owner's
            // control at the beginning of the next end step". Full exile + return-transformed
            // sequence not modeled; only Transform is emitted here.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_sacrifice_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn etb_scry(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 2 }]
}

fn on_sacrifice_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Oracle says "return it to the battlefield transformed at next end step".
    // DelayedAction + exile + transform chain not expressible; emitting Transform only.
    vec![Effect::Transform { target: trig.source }]
}
