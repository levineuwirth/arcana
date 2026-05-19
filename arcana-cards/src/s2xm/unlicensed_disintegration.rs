//! Unlicensed Disintegration — `{1}{B}{R}` instant.
//! "Destroy target creature. If you control an artifact, Unlicensed
//! Disintegration deals 3 damage to that creature's controller."
//!
//! # GAP: conditional on controlling an artifact; damage to destroyed
//! creature's controller (not a fixed target player)
//! The destroy is expressible. The conditional artifact check and "deal damage
//! to the target's controller" rider are not expressible with the catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unlicensed Disintegration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. If you control an artifact, Unlicensed Disintegration deals 3 damage to that creature's controller.".into(),
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
    // GAP: conditional on controlling an artifact; damage to target creature's controller
    vec![Effect::DestroyPermanent { target: *id }]
}
