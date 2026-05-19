//! Duel for Dominance — `{1}{G}` instant, "Coven — Choose target creature you
//! control and target creature you don't control. If you control three or more
//! creatures with different powers, put a +1/+1 counter on the chosen creature
//! you control. Then the chosen creatures fight each other."
//!
//! # GAP
//! GAP: Coven condition (check for three or more creatures with different
//! powers) not in catalog.
//! GAP: conditional AddCounters based on Coven not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duel for Dominance");
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
                text: "Coven — Choose target creature you control and target creature you don't control. If you control three or more creatures with different powers, put a +1/+1 counter on the chosen creature you control. Then the chosen creatures fight each other.".into(),
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
    let mut targets = entry.targets.targets.iter();
    let Some(t0) = targets.next() else { return Vec::new(); };
    let Some(t1) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(id0) = t0 else { return Vec::new(); };
    let TargetChoice::Object(id1) = t1 else { return Vec::new(); };
    // GAP: Coven condition check (three or more creatures with different powers) not in catalog
    // GAP: conditional AddCounters based on Coven not expressible
    vec![Effect::Fight { a: *id0, b: *id1 }]
}
