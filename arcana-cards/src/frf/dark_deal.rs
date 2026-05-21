//! Dark Deal — `{2}{B}` sorcery. "Each player discards all the cards
//! in their hand, then draws that many cards minus one."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dark Deal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards all the cards in their hand, then draws that many cards minus one.".into(),
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
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        let n = script::hand_size(state, p);
        effects.push(Effect::Discard {
            player: p,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        });
        if n > 0 {
            effects.push(Effect::DrawCards {
                player: p,
                count: n - 1,
            });
        }
    }
    effects
}
