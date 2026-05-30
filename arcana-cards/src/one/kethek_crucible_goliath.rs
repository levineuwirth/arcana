//! Kethek, Crucible Goliath — `{2}{B}{R}` 4/4 legendary black/red
//! Phyrexian Beast. "At the beginning of your end step, you may
//! sacrifice another creature. If you do, reveal cards from the top of
//! your library until you reveal a nonlegendary creature card with
//! lesser mana value, put it onto the battlefield, then put the rest on
//! the bottom of your library in a random order."
//!
//! GAP: The "you may sacrifice another creature" gate — OptionalPaymentKind
//! supports only Mana and Life costs; sacrifice-other is not expressible.
//! The reveal is emitted unconditionally.
//! GAP: "with lesser mana value" — the mana-value comparison against the
//! sacrificed creature is dynamic and not expressible via ObjectFilter at
//! resolution time; the nonlegendary creature filter is emitted without
//! the lesser-mana-value constraint.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kethek, Crucible Goliath");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_reveal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_reveal(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice another creature. If you do," — sacrifice-other
    // optional gate not expressible; reveal fires unconditionally.
    // GAP: "with lesser mana value" — dynamic mana-value comparison not
    // expressible in ObjectFilter at resolution time.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::creature()
            .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}
