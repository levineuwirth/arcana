//! Tester of the Tangential — `{1}{U}` 1/1 Djinn Wizard.
//! Increment (Whenever you cast a spell, if the mana you spent is greater than
//!   this creature's power or toughness, put a +1/+1 counter on it.)
//! At the beginning of combat on your turn, you may pay {X}. When you do, move
//!   X +1/+1 counters from this creature onto another target creature.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tester of the Tangential");
    let djinn = reg.interner_mut().intern("Djinn");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                // GAP: intervening-if "if the mana you spent is greater than
                // this creature's power or toughness" — mana-spent-on-spell is
                // not exposed to conditions::/script:: helpers.
                intervening_if: None,
                effect: increment_counter,
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
                effect: move_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn increment_counter(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: gated on the mana-spent intervening-if above (see id 1); firing
    // unconditionally would be a materially wrong card, so the effect is omitted.
    Vec::new()
}

fn move_counters(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may pay {X}. ... move X +1/+1 counters from this creature onto
    // another target creature" — variable-X payment (OptionalPayment is fixed
    // mana only) and counter-move-by-X have no demonstrated primitive.
    Vec::new()
}
