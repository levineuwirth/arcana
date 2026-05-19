//! Tail Slash — `{2}{R}` instant. "Target creature you control deals damage equal to its
//! power to target creature you don't control."
//!
//! GAP: source-creature's-power as damage amount is not expressible; Effect::DealDamage
//! requires a fixed u32, and there is no catalog variant for "deals damage equal to
//! source creature's power". Fight is closest but has different semantics. Using
//! Effect::Fight as the best approximation (a fights b).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tail Slash");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature you don't control.".into(),
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
    // GAP: one-sided damage equal to source creature's power (not mutual fight)
    let mut targets = entry.targets.targets.iter();
    let Some(TargetChoice::Object(a)) = targets.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(b)) = targets.next() else { return Vec::new(); };
    vec![Effect::Fight { a: *a, b: *b }]
}
