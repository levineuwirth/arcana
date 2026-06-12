//! Free from Flesh — `{R}` instant. "Target creature gets +2/+2
//! until end of turn. Put two oil counters on it."
//!
//! The +2/+2 is expressed. The oil counters are wired as
//! `CounterKind::Named("oil")` (oil counters have no inherent rules
//! meaning — pure bookkeeping other cards read).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Free from Flesh");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("oil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets +2/+2 until end of turn. Put two oil counters on it.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(kind) = reg.interner().lookup("oil").map(CounterKind::Named) {
        effects.push(Effect::AddCounters { target: *id, kind, count: 2 });
    }
    effects
}
