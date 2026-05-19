//! Frightful Delusion — `{2}{U}` instant.
//! "Counter target spell unless its controller pays {1}. That player discards
//! a card."
//! GAP: 'counter unless controller pays {1}' (conditional counter based on
//! opponent paying mana) — no Effect variant for 'counter unless pay'.

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frightful Delusion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell unless its controller pays {1}. That player discards a card.".into(),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(stack_id) = target else { return Vec::new(); };
    // Find the controller of the targeted spell to make them discard.
    let spell_controller = state
        .stack
        .iter()
        .find(|e| e.source == *stack_id)
        .map(|e| e.controller)
        .unwrap_or(entry.controller);
    // GAP: 'counter unless controller pays {1}' conditional counter
    vec![
        Effect::Counter { target: *stack_id },
        Effect::Discard { player: spell_controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}
