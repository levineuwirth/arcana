//! Famous Museum — `{3}` artifact.
//! "Whenever a creature you control dies, put an art counter on Famous Museum."
//! "{T}: Add {C}, plus an additional {C} for each art counter on Famous Museum."
//!
//! A counter-accumulation mana rock. The trigger (a creature you control
//! dying) places an "art" counter on the source. The mana ability adds
//! one {C} plus a dynamic {C} per art counter currently on the source,
//! read via `script::source_counter_count`. Both are fully faithful.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Famous Museum");
    // Intern the "art" counter name so the resolvers can look it up.
    let _ = reg.interner_mut().intern("art");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: add_art_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}, plus an additional {C} for each art counter on Famous Museum.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_scaled_mana,
            }),
    )
}

fn add_art_counter(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(art) = reg.interner().lookup("art") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(art),
        count: 1,
    }]
}

fn add_scaled_mana(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let count = reg
        .interner()
        .lookup("art")
        .map(|art| script::source_counter_count(state, ctx.source, CounterKind::Named(art)))
        .unwrap_or(0);
    // Base {C} plus an additional {C} per art counter.
    let total = (count + 1) as usize;
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); total],
    }]
}
