//! Aerial Volley — `{G}` instant. "Aerial Volley deals 3 damage
//! divided as you choose among one, two, or three target creatures
//! with flying."
//!
//! Divided-as-you-choose damage and a "with flying" target predicate
//! are not in catalog/script surface. Modeled as 1 damage to each of
//! up to three target creatures; the flying restriction and division
//! semantics are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aerial Volley");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Aerial Volley deals 3 damage divided as you choose among one, two, or three target creatures with flying.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "with flying" target refinement and divided-damage choice not in catalog.
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*id),
                amount: 1,
            });
        }
    }
    effects
}
