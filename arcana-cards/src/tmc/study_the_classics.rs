//! Study the Classics — `{2}{G}` sorcery. "Put a +1/+1 counter on
//! target creature, then double the number of +1/+1 counters on it.
//! You gain life equal to the number of +1/+1 counters on that
//! creature."
//!
//! The opening +1/+1 counter is expressible. The "double the number
//! of +1/+1 counters on it" doubling and the "gain life equal to the
//! number of +1/+1 counters on that creature" dynamic amount are NOT:
//! there is no effect to double counters on a single object, and no
//! script helper to read the +1/+1 counter count of a specific
//! permanent (only board-wide `count_matching` / power / toughness
//! exist). Emitting only the literal counter would silently drop the
//! dynamic doubling+lifegain, so those are GAPed below.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Study the Classics");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature, then double the number of +1/+1 counters on it. You gain life equal to the number of +1/+1 counters on that creature.".into(),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: cannot double the +1/+1 counters on a single object (no
    // counter-doubling effect) nor gain life equal to that object's
    // +1/+1 counter count (no script helper reads a specific
    // permanent's counter count). Only the initial counter is emitted.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
