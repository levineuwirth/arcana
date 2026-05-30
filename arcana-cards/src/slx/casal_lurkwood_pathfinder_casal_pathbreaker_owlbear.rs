//! Casal, Lurkwood Pathfinder // Casal, Pathbreaker Owlbear — `{3}{G}` green
//! Legendary Creature — Tiefling Druid (3/3) / Legendary Creature — Bird Bear.
//!
//! Front face (Casal, Lurkwood Pathfinder):
//!   Vigilance.
//!   When Casal enters, search your library for a Forest card, put it onto the
//!   battlefield tapped, then shuffle.
//!   Whenever Casal attacks, you may pay {1}{G}. If you do, transform her.
//!
//! Back face (Casal, Pathbreaker Owlbear):
//!   Vigilance, trample.
//!   When this creature transforms into Casal, Pathbreaker Owlbear, other legendary
//!   creatures you control get +2/+2 and gain trample until end of turn.
//!   At the beginning of your upkeep, transform Casal.
//!
//! GAP: Back-face-only triggered ability "when this creature transforms into Casal,
//!      Pathbreaker Owlbear — other legendary creatures get +2/+2 and trample" not
//!      auto-installed on transform. Not modeled.
//! GAP: Back-face-only upkeep transform (back to front) not auto-installed on transform.
//! The attack trigger with optional {1}{G} pay to transform is modeled on the front face.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Casal, Lurkwood Pathfinder");
    let tiefling_sub = reg.interner_mut().intern("Tiefling");
    let druid_sub = reg.interner_mut().intern("Druid");
    let forest_sub = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling_sub);
    subtypes.0.insert(druid_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Casal, Pathbreaker Owlbear");
    let bird_sub = reg.interner_mut().intern("Bird");
    let bear_sub = reg.interner_mut().intern("Bear");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(bird_sub);
    back_subtypes.0.insert(bear_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    let _ = forest_sub; // pre-interned for tutor filter at resolve time

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: search library for a Forest card, put it onto battlefield tapped.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_forest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Attack trigger: you may pay {1}{G}. If you do, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_optional_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
        // GAP: back-face-only triggered ability not modeled:
        //   "When this creature transforms into Casal, Pathbreaker Owlbear,
        //    other legendary creatures you control get +2/+2 and gain trample until end of turn."
        // GAP: back-face-only triggered ability not modeled:
        //   "At the beginning of your upkeep, transform Casal." (back to front)
    )
}

fn etb_tutor_forest(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest_name = reg.interner().lookup("Forest");
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: arcana_core::targets::ObjectFilter {
            name: forest_name,
            ..arcana_core::targets::ObjectFilter::default()
        },
        tapped: true,
    }]
}

fn attack_optional_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{G}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
