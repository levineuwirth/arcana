//! Sylvan Smite — `{1}{G}` instant. "Put a +1/+1 counter on target creature you
//! control if you weren't the starting player. Then that creature deals damage
//! equal to its power to target creature you don't control."
//!
//! GAP: conditional on "weren't the starting player" (no API to query starting
//! player at resolve time). GAP: "that creature deals damage equal to its power"
//! requires creature-to-creature damage sourced from a battlefield permanent, not
//! from the spell; no Fight variant that only deals in one direction. Best-effort:
//! emit the fight (mutual damage) between the two targets; the "+1/+1 counter if
//! not starting player" conditional is a gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sylvan Smite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control if you weren't the starting player. Then that creature deals damage equal to its power to target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
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
    // GAP: conditional "+1/+1 counter if not starting player" — no API for starting player query
    // GAP: one-directional creature-deals-power-damage (fight is mutual); using Fight as approximation
    let targets = &entry.targets.targets;
    if targets.len() < 2 { return Vec::new(); }
    let TargetChoice::Object(id_a) = &targets[0] else { return Vec::new(); };
    let TargetChoice::Object(id_b) = &targets[1] else { return Vec::new(); };
    vec![Effect::Fight { a: *id_a, b: *id_b }]
}
