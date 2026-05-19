//! Spite of Mogis — `{R}` sorcery. "Spite of Mogis deals damage to target
//! creature equal to the number of instant and sorcery cards in your graveyard.
//! Scry 1."
//
// GAP: "damage equal to number of instant and sorcery cards in your graveyard"
// requires a dynamic amount based on graveyard contents, which DealDamage
// requires a fixed u32. Scry 1 is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spite of Mogis");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Spite of Mogis deals damage to target creature equal to the number of instant and sorcery cards in your graveyard. Scry 1.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    vec![
        // GAP: damage equal to count of instant/sorcery cards in graveyard (dynamic amount) not expressible
        Effect::Scry { player: entry.controller, count: 1 },
    ]
}
