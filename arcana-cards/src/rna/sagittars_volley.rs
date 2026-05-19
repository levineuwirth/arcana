//! Sagittars' Volley — `{2}{G}` instant, "Destroy target creature with flying.
//! Sagittars' Volley deals 1 damage to each other creature with flying your
//! opponents control."
//!
//! # GAP
//! GAP: ObjectFilter lacks keyword-ability (Flying) filtering for ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sagittars' Volley");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature with flying. Sagittars' Volley deals 1 damage to each other creature with flying your opponents control.".into(),
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
    let arcana_core::targets::TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: ObjectFilter lacks keyword (Flying) filtering for the secondary damage effect
    vec![Effect::DestroyPermanent { target: *id }]
}
