//! Urza's Guilt — `{2}{U}{B}` sorcery. "Each player draws two cards, then
//! discards three cards, then loses 4 life."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Guilt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player draws two cards, then discards three cards, then loses 4 life.".into(),
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
    let mut effects: Vec<Effect> = Vec::new();
    for p in script::all_players(state) {
        effects.push(Effect::DrawCards { player: p, count: 2 });
    }
    for p in script::all_players(state) {
        effects.push(Effect::Discard {
            player: p,
            count: 3,
            choice: DiscardChoice::ControllerChooses,
        });
    }
    for p in script::all_players(state) {
        effects.push(Effect::LoseLife { player: p, amount: 4 });
    }
    let _ = entry;
    effects
}
