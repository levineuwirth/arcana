//! Pox Plague — `{B}{B}{B}{B}{B}` sorcery. Each player loses half their
//! life, discards half their hand, sacrifices half their permanents.

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
    let name = reg.interner_mut().intern("Pox Plague");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player loses half their life, then discards half the cards in their hand, then sacrifices half the permanents they control of their choice. Round down each time.".into(),
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
        let life = script::life(state, p).max(0) as u32;
        let hand = script::hand_size(state, p);
        let perms = script::count_matching(state, &ObjectFilter::permanent(), p);
        effects.push(Effect::LoseLife { player: p, amount: life / 2 });
        effects.push(Effect::Discard {
            player: p,
            count: hand / 2,
            choice: DiscardChoice::ControllerChooses,
        });
        effects.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::permanent(),
            count: perms / 2,
        });
    }
    let _ = entry;
    effects
}
