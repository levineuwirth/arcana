//! Dark Deal — `{2}{B}` sorcery. "Each player discards all the cards
//! in their hand, then draws that many cards minus one."
//!
//! Per-player dynamic discard-all-then-draw-(N-1) coupling is not
//! expressible — `Discard{count}` is a fixed-amount, and the
//! "draws that many minus one" depends on the per-player hand size
//! BEFORE the discard. Modeled by reading each player's current hand
//! size, discarding that many, then drawing max(N-1, 0).

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player discards all the cards in their hand, then draws that many cards minus one.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        let h = script::hand_size(state, p);
        effects.push(Effect::Discard {
            player: p,
            count: h,
            choice: DiscardChoice::ControllerChooses,
        });
        let draw = h.saturating_sub(1);
        effects.push(Effect::DrawCards { player: p, count: draw });
    }
    effects
}
