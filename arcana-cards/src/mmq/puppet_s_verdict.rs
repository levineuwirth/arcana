//! Puppet's Verdict — `{1}{R}{R}` instant. "Flip a coin. If you win the flip,
//! destroy all creatures with power 2 or less. If you lose the flip, destroy
//! all creatures with power 3 or greater."
//!
//! # GAP: coin-flip (randomized conditional) not in Effect catalog
//! Best-effort: emit the board-wipe for power ≤ 2 side only (the winner side).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Puppet's Verdict");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flip a coin. If you win the flip, destroy all creatures with power 2 or less. If you lose the flip, destroy all creatures with power 3 or greater.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: coin-flip conditional not in Effect catalog
    // Best-effort: destroy creatures with power ≤ 2 (win side)
    let targets = script::ids_matching(
        state,
        &ObjectFilter::creature().with_max_power(2),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
