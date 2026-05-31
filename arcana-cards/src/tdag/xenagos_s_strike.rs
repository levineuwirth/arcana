//! Xenagos's Strike — sorcery. "Xenagos's Strike deals 4 damage to
//! each player." A no-target each-player burn spell.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xenagos's Strike");
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Xenagos's Strike deals 4 damage to each player.".into(),
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
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 4,
        })
        .collect()
}
