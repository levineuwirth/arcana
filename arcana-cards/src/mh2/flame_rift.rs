//! Flame Rift — `{1}{R}` sorcery, "Flame Rift deals 4 damage to each player."
//!
//! GAP: 'each player' requires iterating over all PlayerIds; no script helper
//! enumerates all players. Best-effort: deal 4 to controller (self) only;
//! dealing to all players is a GAP.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flame Rift");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flame Rift deals 4 damage to each player.".into(),
                target_requirements: vec![],
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
    // GAP: no way to enumerate all players; dealing to controller only as best-effort
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(entry.controller),
        amount: 4,
    }]
}
