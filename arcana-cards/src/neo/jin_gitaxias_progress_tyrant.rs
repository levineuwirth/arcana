//! Jin-Gitaxias, Progress Tyrant — `{5}{U}{U}` 5/5 Legendary Phyrexian
//! Praetor.
//!
//! Oracle:
//! * Whenever you cast an artifact, instant, or sorcery spell, copy that spell.
//!   You may choose new targets for the copy. This ability triggers only once
//!   each turn.
//! * Whenever an opponent casts an artifact, instant, or sorcery spell, counter
//!   that spell. This ability triggers only once each turn.
//!
//! Both triggers are expressible as filtered `SpellCast` triggers with
//! `OncePerTurn` frequency. Both EFFECTS are GAP'd: there is no
//! `PendingTrigger` accessor exposing the triggering spell's stack object id,
//! so "that spell" can't be referenced; additionally "counter that spell" has
//! no counter-spell `Effect` variant in the demonstrated surface. The triggers
//! are kept wired with GAP bodies.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jin-Gitaxias, Progress Tyrant");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let praetor = reg.interner_mut().intern("Praetor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(praetor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let ais_filter = || {
        ObjectFilter::new().with_types_any(TypeLine(
            TypeLine::ARTIFACT | TypeLine::INSTANT | TypeLine::SORCERY,
        ))
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ais_filter()),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: copy_that_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ais_filter()),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: counter_that_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_that_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy that spell" — no accessor exposes the triggering spell's stack
    // object id, so the spell to copy can't be referenced.
    Vec::new()
}

fn counter_that_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "counter that spell" — no triggering-spell accessor, and no
    // counter-spell Effect variant in the demonstrated surface.
    Vec::new()
}
