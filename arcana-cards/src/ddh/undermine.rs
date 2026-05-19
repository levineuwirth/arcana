//! Undermine — `{U}{U}{B}` instant. "Counter target spell. Its
//! controller loses 3 life."
//!
//! GAP: SpellControllerLoseLife (looking up the controller of a targeted
//! stack object to deal the life loss to them specifically) requires a
//! state accessor not shown in the catalog. The counter effect is
//! expressed; the life loss is applied to the caster (entry.controller)
//! as an approximation — the verify pipeline should flag this.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undermine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Its controller loses 3 life.".into(),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(stack_id) = target else { return Vec::new(); };
    // GAP: SpellControllerLoseLife — cannot read targeted spell's controller from stack;
    // life loss player approximated as entry.controller (incorrect for real play).
    vec![
        Effect::Counter { target: *stack_id },
        Effect::LoseLife { player: entry.controller, amount: 3 },
    ]
}
