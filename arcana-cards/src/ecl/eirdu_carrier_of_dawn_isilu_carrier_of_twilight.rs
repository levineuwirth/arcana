//! Eirdu, Carrier of Dawn // Isilu, Carrier of Twilight
//!
//! Front (Eirdu): Legendary Creature — Elemental God {3}{W}{W}, 5/5
//! Flying, lifelink.
//! Creature spells you cast have convoke. (GAP: Convoke not modeled as a static ability.)
//! At the beginning of your first main phase, you may pay {B}. If you do, transform Eirdu.
//!
//! Back (Isilu): Legendary Creature — Elemental God
//! Flying, lifelink.
//! Each other nontoken creature you control has persist. (GAP: back-face-only static ability not modeled.)
//! At the beginning of your first main phase, you may pay {W}. If you do, transform Isilu.
//! GAP: back-face-only triggered ability not modeled (transform back to front trigger).
//! GAP: Convoke (front-face static grant) not modeled — no static ability API.
//! GAP: Persist grant to other creatures (back-face static) not modeled.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eirdu, Carrier of Dawn");

    let elemental = reg.interner_mut().intern("Elemental");
    let god = reg.interner_mut().intern("God");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(elemental);
    front_subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Isilu, Carrier of Twilight");
    let elemental2 = reg.interner_mut().intern("Elemental");
    let god2 = reg.interner_mut().intern("God");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elemental2);
    back_subtypes.0.insert(god2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: at beginning of your first main phase, you may pay {B}. If so, transform (front -> back).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: transform_eirdu,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face-only triggered ability not modeled (transform Isilu back to front via {W}).
    )
}

fn transform_eirdu(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::OptionalPayment {
            chooser: trig.controller,
            cost: OptionalPaymentKind::Mana(ManaCost::parse("{B}").expect("valid cost")),
            then: Box::new(Effect::Transform { target: trig.source }),
            else_effect: None,
        },
    ]
}
