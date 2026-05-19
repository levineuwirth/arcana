//! Windswift Slice — `{2}{G}` instant, "Target creature you control deals
//! damage equal to its power to target creature you don't control. Create
//! a number of 1/1 green Elf Warrior creature tokens equal to the amount
//! of excess damage dealt this way."
//!
//! GAP: excess damage tracking (tokens equal to excess) not in catalog.
//! Emitting Fight between the two targets as best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Windswift Slice");
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
                text: "Target creature you control deals damage equal to its power to target creature you don't control. Create a number of 1/1 green Elf Warrior creature tokens equal to the amount of excess damage dealt this way.".into(),
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
    // GAP: excess damage tracking for token creation not in catalog
    let mut targets = entry.targets.targets.iter();
    let Some(t0) = targets.next() else { return Vec::new(); };
    let Some(t1) = targets.next() else { return Vec::new(); };
    let (TargetChoice::Object(id0), TargetChoice::Object(id1)) = (t0, t1) else { return Vec::new(); };
    vec![Effect::Fight { a: *id0, b: *id1 }]
}
