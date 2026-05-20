//! Chandra's Flame Wave — `{3}{R}{R}` sorcery. "Chandra's Flame Wave
//! deals 2 damage to target player and each creature that player
//! controls. Search your library and/or graveyard for a card named
//! Chandra, Flame's Fury, reveal it, and put it into your hand."
//!
//! Per-target-player creature filtering for ForEach isn't available
//! in script::* (no `controlled_by(target_player)`); damages the
//! player, and the per-player creatures sweep and the name-specific
//! library+graveyard tutor are GAP'd.

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
    let name = reg.interner_mut().intern("Chandra's Flame Wave");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Chandra's Flame Wave deals 2 damage to target player and each creature that player controls. Search your library and/or graveyard for a card named Chandra, Flame's Fury, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: target-player-controlled creatures sweep cannot be derived from script::*;
    // name-specific tutor across library AND graveyard also not in catalog.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(*p),
        amount: 2,
    }]
}
