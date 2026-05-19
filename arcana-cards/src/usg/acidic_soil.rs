//! Acidic Soil — `{2}{R}` sorcery, "Acidic Soil deals damage to each player equal to
//! the number of lands they control."
//!
//! GAP: per-player damage where amount differs by player (each player's own land count)
//! is not expressible with ForEach over players using script helpers.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Acidic Soil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Acidic Soil deals damage to each player equal to the number of lands they control.".into(),
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
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let my_lands = script::count_matching(state, &land_filter.clone().controlled_by(ControllerConstraint::You), entry.controller);
    let opp_lands = script::count_matching(state, &land_filter.controlled_by(ControllerConstraint::Opponent), entry.controller);
    // GAP: no API to enumerate all player IDs; using entry.controller for self and approximating opponent
    vec![
        Effect::DealDamage { source: entry.source, target: DamageTarget::Player(entry.controller), amount: my_lands },
        // GAP: opponent player ID not derivable; opponent damage approximated at 0
    ]
}
