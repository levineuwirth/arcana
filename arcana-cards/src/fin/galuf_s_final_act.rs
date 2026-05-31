//! Galuf's Final Act — `{1}{G}` instant. "Until end of turn, target
//! creature gets +1/+0 and gains 'When this creature dies, put a
//! number of +1/+1 counters equal to its power on up to one target
//! creature.'"
//!
//! The +1/+0 pump is faithful. The granted death-triggered ability
//! (which itself creates a new triggered ability with a dynamic,
//! power-derived counter count on a fresh target) cannot be granted
//! via `Effect::Pump`'s keyword list — there is no primitive for
//! granting an arbitrary ad-hoc triggered ability to a creature.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galuf's Final Act");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Until end of turn, target creature gets +1/+0 and gains \"When this creature dies, put a number of +1/+1 counters equal to its power on up to one target creature.\"".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: cannot grant an ad-hoc death-triggered ability (dies → put
    // +1/+1 counters equal to its power on up to one target creature)
    // to a creature; only the +1/+0 pump is expressible.
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
