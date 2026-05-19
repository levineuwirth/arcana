//! Essence Pulse — `{3}{B}` sorcery, "You gain 2 life. Each creature gets
//! -X/-X until end of turn, where X is the amount of life you gained this
//! turn."
//! GAP: tracking total life gained this turn is not accessible via script
//! helpers.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Essence Pulse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You gain 2 life. Each creature gets -X/-X until end of turn, where X is the amount of life you gained this turn.".into(),
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
    // GAP: total life gained this turn not accessible via script helpers
    // Emit gain-life only; pump-down requires the tracked life-gained value.
    vec![Effect::GainLife { player: entry.controller, amount: 2 }]
}
