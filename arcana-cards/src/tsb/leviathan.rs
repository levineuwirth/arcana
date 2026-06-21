//! Leviathan — `{5}{U}{U}{U}{U}` 10/10 Creature — Leviathan with Trample.
//!
//! * "This creature enters tapped and doesn't untap during your untap step." —
//!   GAP: enters-tapped + skip-untap static is not expressible.
//! * "At the beginning of your upkeep, you may sacrifice two Islands. If you
//!   do, untap this creature." — GAP: `OptionalPaymentKind` has only Mana/Life;
//!   a sacrifice-as-optional-cost is not expressible. Trigger structure emitted
//!   with a GAP'd body.
//! * "This creature can't attack unless you sacrifice two Islands." — GAP:
//!   attack-cost restriction is not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leviathan");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![KeywordAbility::Trample],
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
                effect: upkeep_maybe_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

// GAP: "you may sacrifice two Islands. If you do, untap this creature." —
// sacrifice-as-an-optional-cost is not expressible (OptionalPaymentKind is
// Mana/Life only).
fn upkeep_maybe_untap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
