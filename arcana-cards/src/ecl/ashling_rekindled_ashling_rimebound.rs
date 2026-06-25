//! Ashling, Rekindled // Ashling, Rimebound
//!
//! Front: `{1}{R}` Legendary Creature — Elemental Sorcerer 1/3.
//!   Whenever this creature enters or transforms into Ashling, Rekindled, you may discard a card.
//!   If you do, draw a card.
//!   At the beginning of your first main phase, you may pay {U}. If you do, transform Ashling.
//!
//! Back: Legendary Creature — Elemental Wizard.
//!   Whenever this creature transforms into Ashling, Rimebound and at the beginning of your first
//!   main phase, add two mana of any one color. Spend this mana only to cast spells with mana value 4+.
//!   At the beginning of your first main phase, you may pay {R}. If you do, transform Ashling.
//!
//! GAP: back-face "transforms into Ashling, Rimebound [or first main phase], add two
//!   mana" trigger not modeled — the mana effect itself is inexpressible (see below).
//! GAP: "add two mana of any one color" — this is a triggered-ability effect, so
//!   the per-color mana-ability idiom can't apply (the player doesn't pick the color
//!   by choosing an ability), and there is no chosen-color-mana follow-up for a
//!   resolver. Left omitted.
//! GAP: "spend this mana only to cast spells with mana value 4 or greater" restriction not modeled.
//! GAP: Back-face triggered abilities not auto-installed on transform.
//! The front-face "you may discard a card. If you do, draw a card" loot trigger is wired
//! via an optional discard payment (discard a card, then draw a card).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashling, Rekindled");

    let elemental_sub = reg.interner_mut().intern("Elemental");
    let sorcerer_sub = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental_sub);
    subtypes.0.insert(sorcerer_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ashling, Rimebound");
    let back_elemental_sub = reg.interner_mut().intern("Elemental");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_elemental_sub);
    back_subtypes.0.insert(wizard_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: front face — ETB: you may discard a card, if you do draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 4: "or transforms into Ashling, Rekindled" (front face) half
            // of the loot trigger.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(0) },
                intervening_if: None,
                effect: etb_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: front face — beginning of your first main phase, may pay {U} to transform
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_main_phase_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 3: back face — beginning of your first main phase, may pay {R} to transform back
            // GAP: back-face-only; no face-gate on TriggeredAbilityDef.
            // GAP: back face mana ability not modeled here (triggered mana).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_main_phase_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn etb_loot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may discard a card. If you do, draw a card."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}

fn front_main_phase_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{U}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}

fn back_main_phase_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: back-face-only. Also GAP: back face should add two mana of any color before the optional.
    // GAP: "add two mana of any one color" is a triggered-ability effect — the per-color
    // mana-ability idiom can't apply and there is no chosen-color-mana follow-up; omitted.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
