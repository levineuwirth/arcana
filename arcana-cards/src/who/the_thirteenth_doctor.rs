//! The Thirteenth Doctor — `{1}{G}{U}` 2/2 Legendary Creature — Time
//! Lord Doctor (green-blue).
//!
//! Oracle text:
//! * Paradox — Whenever you cast a spell from anywhere other than your
//!   hand, put a +1/+1 counter on target creature.
//! * Team TARDIS — At the beginning of your end step, untap each
//!   creature you control with a counter on it.
//!
//! Implemented: the bones; the Team TARDIS end-step trigger SHAPE is
//! wired.
//!
//! GAP: Paradox fires only on spells cast "from anywhere other than
//! your hand" — the engine's `SpellCast` condition exposes caster + a
//! spell filter but not the cast's source zone, so the trigger can't be
//! restricted to non-hand casts. Wiring it would over-fire on every
//! spell AND force a meaningless target choice, so the whole ability is
//! omitted.
//! GAP: Team TARDIS untaps "each creature you control WITH A COUNTER on
//! it" — the documented `ObjectFilter` refinements have no
//! has-a-counter predicate, so the set can't be scoped correctly; the
//! resolver returns no effects.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Thirteenth Doctor");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let doctor = reg.interner_mut().intern("Doctor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);
    subtypes.0.insert(doctor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: team_tardis_untap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn team_tardis_untap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "untap each creature you control with a counter on it" — no
    // has-a-counter ObjectFilter refinement.
    Vec::new()
}
