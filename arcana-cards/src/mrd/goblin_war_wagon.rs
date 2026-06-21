//! Goblin War Wagon — `{4}` 3/3 Artifact Creature — Juggernaut.
//!
//! Oracle:
//!  * This creature doesn't untap during your untap step. (static — GAP)
//!  * At the beginning of your upkeep, you may pay {2}. If you do, untap this
//!    creature.
//!
//! The "doesn't untap" line is a static untap restriction with no expressible
//! marker on this card class — GAP'd. The upkeep optional-payment untap is
//! wired via Effect::OptionalPayment.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Goblin War Wagon");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    // GAP: "This creature doesn't untap during your untap step" — static untap
    // restriction with no expressible marker on this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
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

fn upkeep_maybe_untap(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::Untap { target: trig.source }),
        else_effect: None,
    }]
}
