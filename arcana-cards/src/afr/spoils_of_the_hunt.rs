//! Spoils of the Hunt — `{2}{G}` instant. "Target creature you control gets +1/+0 until end of
//! turn for each mana from a Treasure that was spent to cast this spell. Then that creature deals
//! damage equal to its power to target creature an opponent controls."
//!
//! GAP: Tracking mana-source type (Treasure) spent to cast a spell not in script helpers.
//! Emitting the Fight approximation (target creature deals damage to opponent's creature) only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spoils of the Hunt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 until end of turn for each mana from a Treasure that was spent to cast this spell. Then that creature deals damage equal to its power to target creature an opponent controls.".into(),
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
    // GAP: Treasure-mana counting for pump amount not in script helpers
    let mut targets = entry.targets.targets.iter();
    let first = targets.next();
    let second = targets.next();
    match (first, second) {
        (Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) => {
            vec![Effect::Fight { a: *a, b: *b }]
        }
        _ => Vec::new(),
    }
}
