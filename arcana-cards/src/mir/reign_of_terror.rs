//! Reign of Terror — `{3}{B}{B}` sorcery. Destroy all green creatures
//! or all white creatures; lose 2 life per creature that died.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reign of Terror");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all green creatures or all white creatures. They can't be regenerated. You lose 2 life for each creature that died this way.".into(),
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
    // GAP: 'green creatures OR white creatures' (player-chosen mode) —
    // no resolution-time modal-choice primitive. Default: hit green.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_colors(ColorSet::green()),
        entry.controller,
    );
    let n = ids.len() as u32;
    let mut effects = vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    effects.push(Effect::LoseLife { player: entry.controller, amount: 2 * n });
    effects
}
