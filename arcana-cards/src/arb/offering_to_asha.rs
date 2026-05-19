//! Offering to Asha — `{2}{W}{U}` instant, "Counter target spell unless its
//! controller pays {4}. You gain 4 life."
//!
//! GAP: "counter unless pays {4}" — no Effect variant for conditional counter with
//! opponent payment option. Best effort: Counter + GainLife.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Offering to Asha");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell unless its controller pays {4}. You gain 4 life.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
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
    // GAP: "counter unless its controller pays {4}" — conditional counter with payment
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(stack_id) = target else { return Vec::new(); };
    vec![
        Effect::Counter { target: *stack_id },
        Effect::GainLife { player: entry.controller, amount: 4 },
    ]
}
