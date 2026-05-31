//! Druid's Deliverance — `{1}{G}` instant. "Prevent all combat damage
//! that would be dealt to you this turn. Populate. (Create a token
//! that's a copy of a creature token you control.)"
//!
//! Best-effort: the engine can prevent all damage dealt to you this
//! turn (source-filtered prevention with a permanent source filter and
//! a player target), but it cannot restrict that to *combat* damage
//! only, and Populate (copy a creature token you control) is not an
//! expressible effect. We emit the broad prevention and GAP the
//! combat-only restriction plus the Populate rider.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Druid's Deliverance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Prevent all combat damage that would be dealt to you this turn. Populate.".into(),
                target_requirements: vec![],
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
    // GAP: cannot restrict prevention to *combat* damage only (engine
    // prevention is source-filtered, not combat-specific). GAP: Populate
    // (create a token that's a copy of a creature token you control) is
    // not an expressible effect.
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::permanent(),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
