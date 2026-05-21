//! Smallpox — `{B}{B}` sorcery. "Each player loses 1 life, discards a
//! card, sacrifices a creature of their choice, then sacrifices a
//! land of their choice."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smallpox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player loses 1 life, discards a card, sacrifices a creature of their choice, then sacrifices a land of their choice.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let players = script::all_players(state);
    let mut effects = Vec::new();
    for p in &players {
        effects.push(Effect::LoseLife { player: *p, amount: 1 });
    }
    for p in &players {
        effects.push(Effect::Discard {
            player: *p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    for p in &players {
        effects.push(Effect::Sacrifice {
            player: *p,
            filter: ObjectFilter::creature(),
            count: 1,
        });
    }
    for p in &players {
        effects.push(Effect::Sacrifice {
            player: *p,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            count: 1,
        });
    }
    effects
}
