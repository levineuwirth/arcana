//! Mycoid Shepherd — `{1}{G}{G}{W}` 5/4 Fungus creature. "Whenever
//! this creature or another creature you control with power 5 or
//! greater dies, you may gain 5 life." Modeled as a single
//! `ZoneChange` trigger filtered to creatures you control with
//! power ≥ 5 leaving the battlefield for a graveyard — since this
//! creature itself is a 5/4, it matches the filter when it dies, so
//! the "this creature or another creature you control" disjunction
//! collapses cleanly into one filter.
//!
//! GAP: the "you may" optionality is not modeled — the engine
//! resolves the gain unconditionally.

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
    let name = reg.interner_mut().intern("Mycoid Shepherd");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_min_power(5),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: gain_five_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_five_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 5 }]
}
