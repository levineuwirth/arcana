//! Chandra's Flame Wave — `{3}{R}{R}` sorcery. "Deals 2 damage to
//! target player and each creature that player controls. Search your
//! library and/or graveyard for a card named Chandra, Flame's Fury,
//! reveal it, and put it into your hand. If you search your library
//! this way, shuffle."
//!
//! # GAP
//! - "Deals 2 damage to target player AND each creature that player
//!   controls" requires iterating a player's creatures and dealing
//!   damage to each plus the player; no combined Effect variant exists.
//! - Search library and/or graveyard for a specific named card is
//!   not representable with `ObjectFilter` (no name filter).
//! The DealDamage to the player is modeled; creature damage and
//! named-card tutor are noted as gaps.

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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Chandra's Flame Wave deals 2 damage to target player and each creature that player controls. Search your library and/or graveyard for a card named Chandra, Flame's Fury, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: "each creature that player controls" damage sweep not expressible without state access for that player's creatures
    // GAP: named-card tutor from library and/or graveyard not expressible (no name filter on ObjectFilter)
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(*p),
        amount: 2,
    }]
}
