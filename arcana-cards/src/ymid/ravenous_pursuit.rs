//! Ravenous Pursuit — `{1}{G}` sorcery. "Target creature you control deals
//! damage equal to its power to target creature you don't control. Choose a
//! creature card in your hand. It perpetually gets +X/+X, where X is the
//! amount of excess damage dealt this way."
//!
//! GAP: "Deals damage equal to its power" requires inspecting the creature's
//! power at resolve time (dynamic amount). "Perpetually gets +X/+X based on
//! excess damage" is also not expressible. Best-effort: Fight approximates
//! the mutual-damage intent but is not exactly the same.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ravenous Pursuit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature you don't control. Choose a creature card in your hand. It perpetually gets +X/+X, where X is the amount of excess damage dealt this way.".into(),
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
    // GAP: One-directional damage (not mutual fight), dynamic damage amount from power,
    // and perpetual bonus from excess damage are not expressible.
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(t1) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(a) = t0 else { return Vec::new(); };
    let TargetChoice::Object(b) = t1 else { return Vec::new(); };
    vec![Effect::Fight { a: *a, b: *b }]
}
