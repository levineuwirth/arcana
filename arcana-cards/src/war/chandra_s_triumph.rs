//! Chandra's Triumph — `{1}{R}` instant.
//! "Chandra's Triumph deals 3 damage to target creature or planeswalker an
//! opponent controls. Chandra's Triumph deals 5 damage instead if you control
//! a Chandra planeswalker."
//!
//! # GAP: target creature or planeswalker — no planeswalker TargetFilter
//! variant in the catalog.
//! # GAP: conditional damage based on controlling a named planeswalker.
//! Modelled as 3 damage to target creature.

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
    let name = reg.interner_mut().intern("Chandra's Triumph");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Chandra's Triumph deals 3 damage to target creature or planeswalker an opponent controls. Chandra's Triumph deals 5 damage instead if you control a Chandra planeswalker.".into(),
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
    // GAP: target planeswalker option
    // GAP: conditional +2 damage if controller has a Chandra planeswalker
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 3,
    }]
}
