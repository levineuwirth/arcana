//! Incremental Blight — `{3}{B}{B}` sorcery, "Put a -1/-1 counter on
//! target creature, two -1/-1 counters on another target creature, and
//! three -1/-1 counters on a third target creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Incremental Blight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a -1/-1 counter on target creature, two -1/-1 counters on another target creature, and three -1/-1 counters on a third target creature.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids: Vec<_> = entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(*id),
            _ => None,
        })
        .collect();
    if ids.len() < 3 {
        return Vec::new();
    }
    vec![
        Effect::AddCounters { target: ids[0], kind: CounterKind::MinusOneMinusOne, count: 1 },
        Effect::AddCounters { target: ids[1], kind: CounterKind::MinusOneMinusOne, count: 2 },
        Effect::AddCounters { target: ids[2], kind: CounterKind::MinusOneMinusOne, count: 3 },
    ]
}
