//! Horrific Assault — `{G}` sorcery.
//! "Target creature you control deals damage equal to its power to target
//! creature or planeswalker you don't control. If you control an Eldrazi, you
//! gain 3 life."
//!
//! # GAP: fight targeting planeswalker (only creatures supported by Fight);
//!   conditional gain-life based on controlling an Eldrazi subtype

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horrific Assault");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature or planeswalker you don't control. If you control an Eldrazi, you gain 3 life.".into(),
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
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(a) = first else { return Vec::new(); };
    let Some(second) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(b) = second else { return Vec::new(); };
    // GAP: planeswalker targeting not expressible with TargetFilter::Creature
    // GAP: conditional "if you control an Eldrazi" gain 3 life
    vec![Effect::Fight { a: *a, b: *b }]
}
