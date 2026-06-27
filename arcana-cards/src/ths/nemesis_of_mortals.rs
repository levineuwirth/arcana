//! Nemesis of Mortals — `{4}{G}{G}` 5/5 Snake.
//!
//! Oracle:
//! * This spell costs {1} less to cast for each creature card in your
//!   graveyard. (GAP: static cast-cost reduction is not expressible.)
//! * {7}{G}{G}: Monstrosity 5. This ability costs {1} less to activate for
//!   each creature card in your graveyard.
//!   Modeled as the Monstrosity 5 payload: put five +1/+1 counters on this
//!   creature. The activation cost-reduction per creature card in your
//!   graveyard is wired via `ActivationCost::cost_reduction`
//!   (`script::graveyard_matching`). GAP: no "becomes monstrous" flag /
//!   one-shot guard.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nemesis of Mortals");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{7}{G}{G}: Monstrosity 5.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{7}{G}{G}").expect("valid cost"),
                // "{1} less to activate for each creature card in your
                // graveyard."
                cost_reduction: Some(graveyard_creatures_reduction),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: monstrosity_five,
        }),
    )
}

/// "{1} less to activate for each creature card in your graveyard"
/// (Idiom A) — only the {7}{G}{G} Monstrosity ability is reduced; the
/// spell's own cast-cost reduction remains a GAP.
fn graveyard_creatures_reduction(
    state: &GameState,
    _source: ObjectId,
    controller: PlayerId,
    _reg: &CardRegistry,
) -> u32 {
    script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        controller,
        controller,
    )
}

fn monstrosity_five(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Monstrosity 5: put five +1/+1 counters on this creature.
    // GAP: the "becomes monstrous" flag (and its once-only guard) is unmodeled.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 5,
    }]
}
