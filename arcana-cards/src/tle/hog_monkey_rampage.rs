//! Hog-Monkey Rampage — `{1}{R/G}` instant. "Choose target creature you control and
//! target creature an opponent controls. Put a +1/+1 counter on the creature you
//! control if it has power 4 or greater. Then those creatures fight each other."
//!
//! # GAP: conditional counter based on power comparison at resolution time
//! The engine has no Effect::Conditional predicate that tests a permanent's power.
//! The fight is expressible; the conditional counter is not.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hog-Monkey Rampage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control and target creature an opponent controls. Put a +1/+1 counter on the creature you control if it has power 4 or greater. Then those creatures fight each other.".into(),
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
    let targets = &entry.targets.targets;
    let (Some(t0), Some(t1)) = (targets.get(0), targets.get(1)) else { return Vec::new(); };
    let (TargetChoice::Object(id0), TargetChoice::Object(id1)) = (t0, t1) else { return Vec::new(); };
    // GAP: conditional +1/+1 counter on id0 if its power >= 4 (power-check predicate not available)
    vec![Effect::Fight { a: *id0, b: *id1 }]
}
