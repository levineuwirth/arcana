//! Well Done — `{2}{R}{R}` Sorcery. "Well Done deals 5 damage to
//! target creature. If that creature is legendary or mythic rare, Well
//! Done deals 3 damage to its controller."
//!
//! # Implementation note
//! The 5 damage to target creature is expressible. The conditional
//! 3 damage (if the creature is legendary or mythic rare) requires a
//! rarity/legendary check not available as a Conditional condition.
//!
//! # GAP
//! Conditional damage based on rarity/legendary status not expressible.

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
    let name = reg.interner_mut().intern("Well Done");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Well Done deals 5 damage to target creature. If that creature is legendary or mythic rare, Well Done deals 3 damage to its controller.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 5,
        },
        // GAP: conditional 3 damage to controller if creature is legendary/mythic rare not expressible
    ]
}
