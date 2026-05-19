//! Incremental Growth — `{3}{G}{G}` sorcery. "Put a +1/+1 counter on target
//! creature, two +1/+1 counters on another target creature, and three +1/+1
//! counters on a third target creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Incremental Growth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature, two +1/+1 counters on another target creature, and three +1/+1 counters on a third target creature.".into(),
                target_requirements: vec![
                    TargetRequirement { filter: TargetFilter::Creature, count: TargetCount::Exactly(1), controller: None },
                    TargetRequirement { filter: TargetFilter::Creature, count: TargetCount::Exactly(1), controller: None },
                    TargetRequirement { filter: TargetFilter::Creature, count: TargetCount::Exactly(1), controller: None },
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
    let targets = &entry.targets.targets;
    let mut effects = Vec::new();
    let counts = [1u32, 2, 3];
    for (i, count) in counts.iter().enumerate() {
        if let Some(TargetChoice::Object(id)) = targets.get(i) {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: *count,
            });
        }
    }
    effects
}
