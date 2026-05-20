//! Eerie Interlude — `{2}{W}` instant. "Exile any number of target
//! creatures you control. Return those cards to the battlefield under
//! their owner's control at the beginning of the next end step."
//!
//! Per target: exile now + DelayedAction at next end step returning
//! it (battlefield blink). DelayedAction returns to hand, not the
//! battlefield, so the return-to-battlefield part is a GAP; the exile
//! is still emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eerie Interlude");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile any number of target creatures you control. Return those cards to the battlefield under their owner's control at the beginning of the next end step.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature()),
                count: TargetCount::Any,
                controller: None,
            }],
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
    // GAP: the at-next-end-step return is to the battlefield;
    // DelayedAction only supports ReturnToHand. Exiles are emitted.
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ExilePermanent { target: *id }),
            _ => None,
        })
        .collect()
}
