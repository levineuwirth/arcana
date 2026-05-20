//! Death Wish — `{1}{B}{B}` sorcery. "You may put a card you own
//! from outside the game into your hand. You lose half your life,
//! rounded up. Exile Death Wish."
//!
//! Only the "lose half your life, rounded up" clause is expressed
//! (computed from current life). The wish (outside-the-game fetch)
//! and self-exile have no catalog primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death Wish");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You may put a card you own from outside the game into your hand. You lose half your life, rounded up. Exile Death Wish.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let life = script::life(state, entry.controller).max(0) as u32;
    let half_up = (life + 1) / 2;
    // GAP: "put a card from outside the game into your hand" and
    // self-exile — no wishboard / self-exile primitive.
    vec![Effect::LoseLife { player: entry.controller, amount: half_up }]
}
