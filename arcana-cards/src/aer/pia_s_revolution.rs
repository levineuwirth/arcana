//! Pia's Revolution — `{2}{R}` enchantment.
//! "Whenever a nontoken artifact is put into your graveyard from the
//! battlefield, return that card to your hand unless target opponent
//! has this enchantment deal 3 damage to them."
//!
//! Wired on a battlefield→graveyard ZoneChange over your nontoken
//! artifacts. The opponent's choice is modeled as an OptionalPayment of
//! 3 LIFE (the closest cost shape to "has this deal 3 damage to them" —
//! documented fidelity GAP); on decline the artifact returns to hand.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pia's Revolution");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "into YOUR graveyard" is an ownership constraint;
                // controlled_by(You) is the closest filter (control ≈ own).
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .nontoken()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: revolution_tax,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            },
        ),
    )
}

/// "…return that card to your hand unless target opponent has this
/// enchantment deal 3 damage to them."
fn revolution_tax(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(card) = trig.dying_object() else {
        return Vec::new();
    };
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(opp) = target else {
        return Vec::new();
    };
    // GAP: fidelity — the opponent "has this enchantment deal 3 damage to
    // them"; modeled as paying 3 life (skips damage prevention/triggers).
    vec![Effect::OptionalPayment {
        chooser: *opp,
        cost: OptionalPaymentKind::Life(3),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::ReturnFromGraveyardToHand {
            target: card,
        })),
    }]
}
