//! Tempt with Glory — `{5}{W}` sorcery.
//! "Tempting offer — Put a +1/+1 counter on each creature you control. Each opponent may
//!  put a +1/+1 counter on each creature they control. For each opponent who does, put a
//!  +1/+1 counter on each creature you control."
//!
//! GAP: "Tempting offer" group decision by opponents with conditional additional counters
//! for each accepting opponent is not expressible with catalog Effect variants.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::objects::NULL_OBJECT_ID;

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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tempting offer — Put a +1/+1 counter on each creature you control. Each opponent may put a +1/+1 counter on each creature they control. For each opponent who does, put a +1/+1 counter on each creature you control.".into(),
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
    let filter = ObjectFilter::creature().controlled_by(arcana_core::targets::ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, entry.controller);
    // GAP: tempting offer group decision with conditional bonus counters not expressible
    // Best effort: put one +1/+1 counter on each creature you control
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}
