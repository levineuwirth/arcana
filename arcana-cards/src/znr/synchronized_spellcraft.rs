//! Synchronized Spellcraft — `{4}{R}` instant, "Deals 4 damage to target creature
//! and X damage to that creature's controller, where X is the number of creatures
//! in your party."
//!
//! # GAP: party-count damage — no Effect variant for dealing damage equal to the
//! number of creatures in the controller's party (Cleric, Rogue, Warrior, Wizard).

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
    let name = reg.interner_mut().intern("Synchronized Spellcraft");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Synchronized Spellcraft deals 4 damage to target creature and X damage to that creature's controller, where X is the number of creatures in your party.".into(),
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
    // GAP: party-count damage to controller — no Effect variant for dealing
    // damage equal to the party count (Cleric/Rogue/Warrior/Wizard) to a player.
    // Emitting the fixed 4 damage to the creature only.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
