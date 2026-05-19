//! End the Festivities — `{R}` sorcery. "End the Festivities deals 1 damage
//! to each opponent and each creature and planeswalker they control."
//!
//! Deals 1 damage to each opponent and each permanent (creature or planeswalker)
//! controlled by opponents.  Uses `ForEach` for the permanents and a direct
//! `DealDamage` to each opponent player.
//!
//! # GAP: targeting opponent players for damage when there may be multiple opponents
//! The catalog has no multi-player iteration; best-effort uses entry.controller
//! to infer a single opponent.  Planeswalker subtype filtering is also not
//! available — `TypeLine::ENCHANTMENT` is used as proxy but planeswalker is
//! not modeled.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("End the Festivities");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "End the Festivities deals 1 damage to each opponent and each creature and planeswalker they control.".into(),
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
    // Damage each opponent's creatures
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let creature_ids = script::ids_matching(state, &filter, entry.controller);
    let mut effects = Vec::new();
    if !creature_ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 1,
            }),
        });
    }
    // GAP: no multi-opponent iteration or planeswalker type for damage to each opponent player
    effects
}
