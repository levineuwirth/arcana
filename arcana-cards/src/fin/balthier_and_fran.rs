//! Balthier and Fran — `{1}{R}{G}` 4/3 Legendary Human Rabbit with Reach.
//!
//! Oracle:
//! "Reach
//!  Vehicles you control get +1/+1 and have vigilance and reach.
//!  Whenever a Vehicle crewed by Balthier and Fran this turn attacks, if it's
//!  the first combat phase of the turn, you may pay {1}{R}{G}. If you do,
//!  after this phase, there is an additional combat phase."
//!
//! * Reach — base keyword.
//! * "Vehicles you control get +1/+1 and have vigilance and reach." — a pure
//!   static continuous anthem; no trigger/cost, so GAP'd.
//! * The attack trigger: the engine has `Effect::AdditionalCombatPhase` and
//!   `Effect::OptionalPayment`, so the "you may pay {1}{R}{G}; if you do,
//!   there is an additional combat phase" payoff IS expressible. We wire it on
//!   a `CreatureAttacks` watcher for a Vehicle you control. Two faithful
//!   limits, noted inline: (a) "crewed by Balthier and Fran this turn" cannot
//!   be filtered (no crew-source tracking) — we approximate with "a Vehicle
//!   you control attacks"; (b) the "if it's the first combat phase of the
//!   turn" intervening-if has no matching `conditions::` predicate, so it is
//!   left ungated.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Balthier and Fran");
    let human = reg.interner_mut().intern("Human");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rabbit);

    // "a Vehicle you control [attacks]".
    let vehicle_filter =
        script::subtype_filter(reg, "Vehicle").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP (static): "Vehicles you control get +1/+1 and have vigilance and
    // reach." Pure static continuous anthem — no trigger/cost to wire.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks { filter: vehicle_filter },
            // GAP (intervening-if): "if it's the first combat phase of the
            // turn" — no matching conditions:: predicate, left ungated.
            intervening_if: None,
            effect: extra_combat_payment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn extra_combat_payment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may pay {1}{R}{G}. If you do, after this phase, there is an
    // additional combat phase." ("crewed by Balthier and Fran this turn" is a
    // GAP — no crew-source filter; this fires for any Vehicle you control.)
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        then: Box::new(Effect::AdditionalCombatPhase),
        else_effect: None,
    }]
}
