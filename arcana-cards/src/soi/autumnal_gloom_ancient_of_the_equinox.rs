//! Autumnal Gloom // Ancient of the Equinox — `{2}{G}` transforming DFC.
//! Front (Autumnal Gloom — Enchantment, G):
//!   {B}: Mill a card.
//!   Delirium — At the beginning of your end step, if there are four or more card
//!     types among cards in your graveyard, transform Autumnal Gloom.
//! Back (Ancient of the Equinox — Creature — Treefolk, G, 5/5):
//!   Trample, hexproof.
//!
//! GAPs:
//! - Delirium intervening-if: "four or more card types among cards in your
//!   graveyard" has no `conditions::` predicate (card-type-count over the
//!   graveyard is not exposed). The transform trigger fires every end step
//!   instead of being gated; the delirium count cannot be expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Autumnal Gloom");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ancient of the Equinox");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(treefolk);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Hexproof],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front (face 0): {B}: Mill a card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: Mill a card.".to_string(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: mill_one,
            })
            // Front (face 0): Delirium end-step transform.
            // GAP: the "four or more card types in graveyard" gate is not
            //   expressible; trigger fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn mill_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill {
        player: ctx.controller,
        count: 1,
    }]
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
