//! Ghostway — `{2}{W}` instant. "Exile each creature you control. Return
//! those cards to the battlefield under their owner's control at the
//! beginning of the next end step."
//!
//! GAP: Delayed return from exile at the beginning of the next end step
//! is not expressible via the current Effect catalog (no delayed trigger
//! / future-zone-return effect variant).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ControllerConstraint, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ghostway");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile each creature you control. Return those cards to the battlefield under their owner's control at the beginning of the next end step.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, entry.controller);
    // GAP: delayed return from exile at next end step not supported; only exile is emitted
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
