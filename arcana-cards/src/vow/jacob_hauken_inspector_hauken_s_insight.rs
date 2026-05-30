//! Jacob Hauken, Inspector // Hauken's Insight
//!
//! Front (Jacob Hauken, Inspector, {1}{U}, Legendary 0/2 Human Advisor):
//!   {T}: Draw a card, then exile a card from your hand face down.
//!        You may look at that card for as long as it remains exiled.
//!        You may pay {4}{U}{U}. If you do, transform Jacob Hauken.
//!   (GAP: "exile a card face down" is not expressible — modeled as draw only.)
//!   (GAP: "look at that card for as long as exiled" is not expressible.)
//!   The {4}{U}{U} payment to transform is modeled via OptionalPayment.
//!
//! Back (Hauken's Insight, Legendary Enchantment):
//!   At the beginning of your upkeep, exile the top card of your library face down.
//!        You may look at that card for as long as it remains exiled.
//!   (GAP: face-down exile + look-at not expressible; modeled as Mill 1.)
//!   Once during each of your turns, you may play a land or cast a spell from among the
//!   cards exiled with this permanent without paying its mana cost.
//!   (GAP: casting from exile zone not modeled.)

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jacob Hauken, Inspector");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hauken's Insight");

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}: Draw a card, then optionally pay {4}{U}{U} to transform
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then exile a card from your hand face down. You may pay {4}{U}{U}. If you do, transform Jacob Hauken.".to_string(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: tap_ability,
            })
            // Back-face: at the beginning of your upkeep, exile top card face down
            // (GAP: face-down exile not expressible; modeled as Mill 1)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_exile_top,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: back-face "play a land or cast a spell from exile without paying" not modeled.
    )
}

fn tap_ability(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Draw a card (GAP: exile a card from hand face down not expressible).
    // Then optionally pay {4}{U}{U} to transform.
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::OptionalPayment {
            chooser: ctx.controller,
            cost: OptionalPaymentKind::Mana(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
            then: Box::new(Effect::Transform { target: ctx.source }),
            else_effect: None,
        },
    ]
}

fn upkeep_exile_top(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile top card face down not expressible; modeling as Mill 1.
    vec![Effect::Mill { player: trig.controller, count: 1 }]
}
