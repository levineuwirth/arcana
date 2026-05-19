//! Planetary Annihilation — `{3}{R}{R}` sorcery. "Each player chooses six
//! lands they control, then sacrifices the rest. Planetary Annihilation deals
//! 6 damage to each creature."
//!
//! # GAP: "each player chooses N lands they control, sacrifice the rest" —
//!   no Sacrifice or player-choice-of-N-permanents Effect variant
//! Best-effort: deal 6 damage to each creature (expressible).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Planetary Annihilation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player chooses six lands they control, then sacrifices the rest. Planetary Annihilation deals 6 damage to each creature.".into(),
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
    // GAP: each player chooses N lands then sacrifices the rest (no Sacrifice / choose-to-keep Effect)
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    creatures.into_iter().map(|id| Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(id),
        amount: 6,
    }).collect()
}
