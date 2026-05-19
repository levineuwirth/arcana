//! Tendrils of Corruption — `{3}{B}` instant.
//! "Tendrils of Corruption deals X damage to target creature and you gain
//! X life, where X is the number of Swamps you control."
//!
//! GAP: counting Swamps (land subtype) is not available via script helpers.
//! Best-effort: deal 1 damage and gain 1 life as placeholder.

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
    let name = reg.interner_mut().intern("Tendrils of Corruption");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tendrils of Corruption deals X damage to target creature and you gain X life, where X is the number of Swamps you control.".into(),
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
    // GAP: counting Swamps (land subtype) not available via script helpers
    // Using 1 as placeholder for X
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 1,
        },
        Effect::GainLife { player: entry.controller, amount: 1 },
    ]
}
