//! Aether Spike — `{1}{U}` instant. "Choose target spell. You get
//! {E}{E} (two energy counters), then you may pay any amount of {E}.
//! Counter that spell unless its controller pays {1} for each {E}
//! paid this way."
//!
//! The energy gain is expressible via `Effect::GainEnergy`. The
//! variable counter — "pay any amount of {E}, counter unless its
//! controller pays {1} per {E} paid" — is not: spending energy as a
//! cost is not yet expressible, and `CounterUnlessPays` takes a fixed
//! `ManaCost`, not a tax derived from energy paid this way.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aether Spike");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target spell. You get {E}{E} (two energy counters), then you may pay any amount of {E}. Counter that spell unless its controller pays {1} for each {E} paid this way.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(ObjectFilter::default()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(_id) = target else { return Vec::new(); };
    // GAP: paying an arbitrary amount of {E} and counting unless the
    // controller pays {1} per {E} paid is not expressible — energy
    // cannot be spent as a cost, and CounterUnlessPays takes a fixed
    // ManaCost, not a tax derived from energy paid this way. Only the
    // energy gain is emitted; the variable counter is dropped.
    vec![Effect::GainEnergy { player: entry.controller, amount: 2 }]
}
