//! Liberating Combustion — `{4}{R}` sorcery. "Liberating Combustion
//! deals 6 damage to target creature. You may search your library
//! and/or graveyard for a card named Chandra, Pyrogenius, reveal it,
//! and put it into your hand."
//!
//! Searching by exact card name across library+graveyard isn't
//! expressible (no name-filtered tutor); the 6 damage is emitted and
//! the search is GAP'd.

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
    let name = reg.interner_mut().intern("Liberating Combustion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Liberating Combustion deals 6 damage to target creature. You may search your library and/or graveyard for a card named Chandra, Pyrogenius, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: name-filtered library/graveyard search is not expressible.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 6,
    }]
}
