//! Borrowing 100,000 Arrows — `{2}{U}` sorcery. "Draw a card for each tapped
//! creature target opponent controls."
//!
//! GAP: cannot inspect the chosen target opponent's tapped creatures at
//! resolution because count_matching takes the resolver's controller, not an
//! arbitrary player. Best effort: count tapped creatures controlled by
//! Opponent (any opponent) — strictly the same in 2-player games.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Borrowing 100,000 Arrows");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card for each tapped creature target opponent controls.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let n = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::Opponent)
            .tapped_only(),
        entry.controller,
    );
    vec![Effect::DrawCards {
        player: entry.controller,
        count: n,
    }]
}
