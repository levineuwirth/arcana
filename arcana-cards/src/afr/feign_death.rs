//! Feign Death — `{B}` instant. "Until end of turn, target creature
//! gains 'When this creature dies, return it to the battlefield
//! tapped under its owner's control with a +1/+1 counter on it.'"
//!
//! Delayed-trigger return-from-graveyard on death is modeled with
//! DelayedAction{ThisDies, ReturnToHand} as the closest catalog
//! action; the actual semantics (back to battlefield tapped + counter)
//! are not honestly representable.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feign Death");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Until end of turn, target creature gains \"When this creature dies, return it to the battlefield tapped under its owner's control with a +1/+1 counter on it.\"".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "return to battlefield tapped with +1/+1 counter" — closest
    // delayed action is ReturnToHand; emit it as a stand-in for the
    // dies-trigger half of the granted ability.
    vec![Effect::DelayedAction {
        source: *id,
        controller: entry.controller,
        when: DelayedWhen::ThisDies,
        action: DelayedAction::ReturnToHand,
    }]
}
