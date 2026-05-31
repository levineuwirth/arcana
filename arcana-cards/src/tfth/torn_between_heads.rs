//! Torn Between Heads — Sorcery (no mana cost; an Unstable / dungeon-style
//! effect). "Tap up to two Heads. They don't untap during the Hydra's next
//! untap step. Torn Between Heads deals 5 damage to each player."
//!
//! Only the "deals 5 damage to each player" clause is expressible. The
//! "tap up to two Heads / they don't untap" clause references named
//! sub-permanents ("Heads") and a don't-untap rider that the engine has no
//! primitive for.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Torn Between Heads");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tap up to two Heads. They don't untap during the Hydra's next untap step. Torn Between Heads deals 5 damage to each player.".into(),
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
    // "deals 5 damage to each player" — one DealDamage per player.
    // GAP: "Tap up to two Heads. They don't untap during the Hydra's next
    // untap step." — no primitive for tapping named sub-permanents nor for a
    // skip-untap rider.
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 5,
        })
        .collect()
}
