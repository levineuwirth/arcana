//! Aberrant Researcher // Perfected Form —
//! `{3}{U}` blue Human Insect creature 3/2 with Flying (front).
//!
//! Front: Flying. At the beginning of your upkeep, mill a card. If an
//! instant or sorcery card was milled this way, transform this creature.
//!
//! Back: Perfected Form — Insect Horror, Flying.
//!
//! # GAP
//! - "If an instant or sorcery card was milled this way, transform" — the
//!   conditional on what type of card was milled is not accessible in the
//!   trigger effect handler (no milled-card inspector in script API).
//!   Modeled as unconditional mill + transform (best effort; omits the
//!   instant/sorcery condition guard).
//! - Back face's Flying keyword is registered on the back Characteristics.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aberrant Researcher");
    let human_sub = reg.interner_mut().intern("Human");
    let insect_sub = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(insect_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Perfected Form");
    let insect_sub2 = reg.interner_mut().intern("Insect");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(insect_sub2);
    back_subtypes.0.insert(horror_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn upkeep_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Mill a card. If an instant or sorcery card was milled this way, transform."
    // GAP: The "if an instant or sorcery was milled" conditional is not checkable
    // from the effect handler (no milled-card inspector). Modeling as unconditional
    // mill + transform (best effort).
    vec![
        Effect::Mill { player: trig.controller, count: 1 },
        Effect::Transform { target: trig.source },
    ]
}
