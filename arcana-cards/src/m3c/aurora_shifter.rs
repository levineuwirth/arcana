//! Aurora Shifter — `{1}{U}` 1/3 Shapeshifter.
//! "Whenever this creature deals combat damage to a player, you get that
//!   many {E}."
//! "At the beginning of combat on your turn, you may pay {E}{E}. When you
//!   do, this creature becomes a copy of another target creature you
//!   control, except it has this ability and '...'." — GAP.
//!
//! Trigger 1 is fully wired: combat damage to a player → gain that many
//! energy (dynamic amount from the damage event).
//!
//! Trigger 2 GAP: paying energy is not a modeled cost (OptionalPayment is
//! Mana/Life only; no {E} cost), and "becomes a copy of another target
//! creature you control, except it keeps these abilities" is a self-copy-
//! with-rider that no Effect expresses. The trigger fires but its effect
//! is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aurora Shifter");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: gain_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pay_and_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: n,
    }]
}

fn pay_and_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {E}{E}. When you do, this creature becomes a copy
    // of another target creature you control, except it has this ability
    // and '...'." — energy is not a modeled cost, and self-copy-with-
    // retained-ability is not an expressible Effect.
    Vec::new()
}
