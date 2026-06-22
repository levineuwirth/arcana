//! Krosan Cloudscraper — `{7}{G}{G}{G}` 13/13 Creature — Beast Mutant.
//!
//! * At the beginning of your upkeep, sacrifice this creature unless you pay
//!   {G}{G}.
//! * Morph {7}{G}{G}.
//!
//! The upkeep "sacrifice unless you pay {G}{G}" is wired with
//! [`TriggerCondition::StepBegins`] (your upkeep) plus an
//! [`Effect::OptionalPayment`] in the "punish unless paid" polarity (pay →
//! no-op; decline → sacrifice this creature). Morph has no usable
//! `KeywordAbility` variant (a face-down cast modifier outside the supported
//! surface) and is GAP'd.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krosan Cloudscraper");
    let beast = reg.interner_mut().intern("Beast");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(13)),
        toughness: Some(PtValue::Fixed(13)),
        // GAP: Morph {7}{G}{G} — face-down cast modifier, no usable keyword.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_pay_or_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// At your upkeep: sacrifice this creature unless you pay {G}{G}.
fn upkeep_pay_or_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{G}{G}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You),
            count: 1,
        })),
    }]
}
