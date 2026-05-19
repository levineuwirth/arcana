//! Strength of Solidarity — `{G}` sorcery.
//! "Choose target creature you control. Put a +1/+1 counter on it for each
//! creature in your party. (Your party consists of up to one each of Cleric,
//! Rogue, Warrior, and Wizard.)"
//!
//! # GAP: "party" mechanic (count of up to one each of Cleric, Rogue, Warrior,
//! Wizard you control) is not available via the script API. Approximated as
//! 0 counters (no-op). A human must wire up party counting.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Strength of Solidarity");
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
                text: "Choose target creature you control. Put a +1/+1 counter on it for each creature in your party.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: party count (up to one each of Cleric, Rogue, Warrior, Wizard)
    // not queryable via script API.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // Using 1 counter as placeholder; actual value is party size (0-4).
    vec![Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 }]
}
