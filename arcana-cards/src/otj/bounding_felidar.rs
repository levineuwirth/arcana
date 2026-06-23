//! Bounding Felidar — `{5}{W}` 4/7 white Cat Beast Mount.
//! "Whenever this creature attacks while saddled, put a +1/+1 counter on each
//! other creature you control. You gain 1 life for each of those creatures.
//! Saddle 2 (Tap any number of other creatures you control with total power
//! 2 or more: This Mount becomes saddled until end of turn. Saddle only as a
//! sorcery.)"
//!
//! Saddle is not a `KeywordAbility` variant the engine models, so the keyword
//! line is empty and the "becomes saddled" cost is GAP'd. The attack payoff
//! is wired on `SelfAttacks`; the "while saddled" gate is GAP'd (no saddled
//! state to read), so the counter + lifegain fire on every attack — the
//! richest faithful behavior available.

use arcana_core::effects::Effect;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bounding Felidar");
    let cat = reg.interner_mut().intern("Cat");
    let beast = reg.interner_mut().intern("Beast");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(beast);
    subtypes.0.insert(mount);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        // GAP: Saddle 2 — Saddle is not a modeled KeywordAbility, and its
        // "tap any number of other creatures with total power >= 2" cost has
        // no activation-cost shape.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "while saddled" gate is unmodeled (no saddled state);
                // closest expressible trigger is plain SelfAttacks.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_buff_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_buff_and_gain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each other creature you control"
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids: Vec<_> = script::ids_matching(state, &filter, trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();
    let n = ids.len() as u32;
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::AddCounters {
                target: NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
        Effect::GainLife {
            player: trig.controller,
            amount: n,
        },
    ]
}
