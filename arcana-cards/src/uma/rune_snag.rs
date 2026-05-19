//! Rune Snag — `{1}{U}` instant, "Counter target spell unless its controller pays
//! {2} plus {2} for each other card named Rune Snag in any graveyard."
//!
//! GAP: variable-cost "counter unless" where threshold = 2 + 2 * (count of
//! Rune Snag copies in all graveyards) — no Effect for conditional counter with
//! opponent payment option based on graveyard card count. Best effort: Counter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rune Snag");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell unless its controller pays {2} plus {2} for each other card named Rune Snag in any graveyard.".into(),
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
    // GAP: "counter unless pays {2+2n}" where n = copies in graveyards (variable-cost counter)
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(stack_id) = target else { return Vec::new(); };
    vec![Effect::Counter { target: *stack_id }]
}
