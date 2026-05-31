//! Unified Lunge — Sorcery (no mana cost). "Unified Lunge deals X
//! damage to each player, where X is the number of Heads on the
//! battlefield."
//!
//! X is the dynamic count of permanents with the "Heads" subtype on
//! the battlefield, computed via `script::count_matching` over a
//! subtype filter. The damage is dealt to each player, one
//! `Effect::DealDamage` per player wrapped in an `Effect::Sequence`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unified Lunge");
    let _heads = reg.interner_mut().intern("Heads");
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Unified Lunge deals X damage to each player, where X is the number of Heads on the battlefield.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &script::subtype_filter(reg, "Heads"),
        entry.controller,
    );
    Effect::Sequence(
        script::all_players(state)
            .into_iter()
            .map(|p| Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Player(p),
                amount: x,
            })
            .collect(),
    )
    .into_vec_single()
}

trait IntoVecSingle {
    fn into_vec_single(self) -> Vec<Effect>;
}

impl IntoVecSingle for Effect {
    fn into_vec_single(self) -> Vec<Effect> {
        vec![self]
    }
}
