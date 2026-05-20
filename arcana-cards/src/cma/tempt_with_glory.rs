//! Tempt with Glory — `{5}{W}` sorcery. "Tempting offer — Put a +1/+1
//! counter on each creature you control. Each opponent may put a +1/+1
//! counter on each creature they control. For each opponent who does,
//! put a +1/+1 counter on each creature you control."
//!
//! Only the first clause (a +1/+1 counter on each creature you
//! control) is expressible; the tempting-offer opponent choice and the
//! per-accepting-opponent rider are not.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tempt with Glory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Tempting offer — Put a +1/+1 counter on each creature \
                   you control. Each opponent may put a +1/+1 counter on \
                   each creature they control. For each opponent who does, \
                   put a +1/+1 counter on each creature you control."
                .into(),
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
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    // GAP: tempting-offer opponent choice and per-accepting-opponent
    // extra counters are not expressible; only the first clause stands.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
