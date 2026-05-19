//! Hard-Hitting Question — `{G}` sorcery. "Target creature you control deals
//! damage equal to its power to target creature or planeswalker you don't control."
//!
//! Note: planeswalkers are not a separate TypeLine in the engine; targeting
//! creatures only. GAP: target creature or planeswalker (planeswalker type).
//! The effect is Fight-like but one-sided (only attacker deals damage).
//! Using Fight as closest available; actual one-sided damage is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hard-Hitting Question");
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
                text: "Target creature you control deals damage equal to its power to target creature or planeswalker you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
                modal: None,
                effect: resolve,
            }),
    )
    // GAP: one-sided damage (attacker deals power-damage to defender only); Fight is mutual
    // GAP: second target should be "creature or planeswalker you don't control"
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut it = entry.targets.targets.iter();
    let t1 = it.next();
    let t2 = it.next();
    match (t1, t2) {
        (Some(TargetChoice::Object(a)), Some(TargetChoice::Object(b))) => {
            vec![Effect::Fight { a: *a, b: *b }]
        }
        _ => Vec::new(),
    }
}
