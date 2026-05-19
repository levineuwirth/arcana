//! Tenderize — `{1}{G}` instant.
//! "Target creature you control deals damage equal to its power to target
//! creature an opponent controls."
//! This is a one-sided fight: only the controlled creature deals damage.
//! Approximated with Effect::Fight (mutual damage); true one-sided damage
//! is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tenderize");
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
                text: "Target creature you control deals damage equal to its power to target creature an opponent controls.".into(),
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
    let mut iter = entry.targets.targets.iter();
    let Some(t0) = iter.next() else { return Vec::new(); };
    let Some(t1) = iter.next() else { return Vec::new(); };
    let (TargetChoice::Object(a), TargetChoice::Object(b)) = (t0, t1) else { return Vec::new(); };
    // GAP: one-sided damage (only first creature deals damage to second);
    // Fight is mutual — used as best effort.
    vec![Effect::Fight { a: *a, b: *b }]
}
