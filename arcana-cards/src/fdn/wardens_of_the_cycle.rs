//! Wardens of the Cycle — `{1}{B}{G}{G}` 3/4 Elf Warlock.
//! "Morbid — At the beginning of your end step, if a creature died this
//! turn, choose one — • You gain 2 life. • You draw a card and you lose
//! 1 life."
//!
//! Morbid is reminder text for the intervening-if, not a usable
//! KeywordAbility. The end-step trigger is wired, but:
//!  - the intervening-if ("if a creature died this turn") has no general
//!    conditions:: predicate (only subtype-scoped death counts exist),
//!    so it is left None and GAP-noted.
//!  - "choose one" modal on a TRIGGERED ability is not expressible in the
//!    provided catalog (ModalSpec is a spell-ability shape only), so the
//!    effect body is GAP'd.

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
    let name = reg.interner_mut().intern("Wardens of the Cycle");
    let elf = reg.interner_mut().intern("Elf");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP: intervening-if "if a creature died this turn" — no
            // general "a creature died this turn" conditions predicate.
            intervening_if: None,
            effect: morbid_end_step,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn morbid_end_step(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one" modal on a triggered ability is not expressible
    // (ModalSpec is spell-only in the provided catalog).
    Vec::new()
}
