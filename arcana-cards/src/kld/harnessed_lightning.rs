//! Harnessed Lightning — `{1}{R}` instant. "Choose target creature.
//! You get {E}{E}{E} (three energy counters), then you may pay any
//! amount of {E}. Harnessed Lightning deals that much damage to that
//! creature."
//!
//! The energy GAIN is expressible (`Effect::GainEnergy`). The variable
//! damage equals the amount of {E} the player chooses to pay, but
//! spending energy as a cost is not yet expressible in the engine, so
//! the pay-energy clause and the resulting variable damage are GAPped.

use arcana_core::effects::Effect;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harnessed Lightning");
    let chars = arcana_core::objects::Characteristics {
        name,
        mana_cost: Some(arcana_core::mana::ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature. You get {E}{E}{E} (three energy counters), then you may pay any amount of {E}. Harnessed Lightning deals that much damage to that creature.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // The energy gain is faithful.
    vec![Effect::GainEnergy { player: entry.controller, amount: 3 }]
    // GAP: "you may pay any amount of {E}. Harnessed Lightning deals
    // that much damage to that creature." Spending energy as a cost is
    // not expressible, and the damage amount is the variable amount of
    // energy paid — there is no engine primitive for pay-energy-as-cost
    // nor for damage equal to energy spent, so this clause is omitted.
}
