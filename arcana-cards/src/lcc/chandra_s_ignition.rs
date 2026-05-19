//! Chandra's Ignition — `{3}{R}{R}` sorcery. "Target creature you
//! control deals damage equal to its power to each other creature and
//! each opponent."
//!
//! Uses script::power_of to get the power, then ForEach over all other
//! creatures for creature damage and LoseLife for each opponent.
//! GAP: DealDamage to each opponent individually (no ForEach over
//! players / no player list); best-effort omits opponent damage.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra's Ignition");
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
                text: "Target creature you control deals damage equal to its power to each other creature and each opponent.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(source_id) = target else { return Vec::new(); };
    let power = script::power_of(state, *source_id).max(0) as u32;
    if power == 0 {
        return Vec::new();
    }
    // All creatures on battlefield (the ForEach template hits each one)
    let all_creatures = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // Remove the source creature itself
    let others: Vec<_> = all_creatures.into_iter().filter(|id| id != source_id).collect();
    let mut effects = Vec::new();
    if !others.is_empty() {
        effects.push(Effect::ForEach {
            targets: others,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: power,
            }),
        });
    }
    // GAP: deal damage to each opponent (no ForEach over players)
    effects
}
