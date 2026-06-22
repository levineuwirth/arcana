//! Conduit Goblin — `{R}{W}` 2/2 Goblin Warrior.
//!
//! Oracle:
//! * "When this creature enters, you get {E}{E} (two energy counters)." —
//!   an ETB trigger via `Effect::GainEnergy`.
//! * "At the beginning of combat on your turn, you may pay {E}. If you do,
//!   another target creature you control gets +1/+0 and gains haste until
//!   end of turn." — the effect IS expressible (pump + grant haste), but the
//!   gate is an energy PAYMENT, and spending {E} as a cost is not a modeled
//!   cost (`OptionalPaymentKind` has only Mana/Life). Firing the pump
//!   unconditionally would be materially wrong, so this trigger's effect is
//!   GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conduit Goblin");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_energy,
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
                effect: combat_pay_energy_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // No target requirement: the pump is gated on an energy
                // payment that cannot be expressed, so the whole effect is
                // GAP'd (declaring a target with no payoff would be worse).
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_gain_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: 2,
    }]
}

fn combat_pay_energy_pump(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {E}. If you do, …" — energy is not a modeled
    // payment cost (OptionalPaymentKind lacks Energy); the pump+haste effect
    // cannot be gated on it, so the whole effect is GAP'd.
    Vec::new()
}
