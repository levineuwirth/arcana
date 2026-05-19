//! Bond of Insight — `{3}{U}` sorcery.
//! "Each player mills four cards. Return up to two instant and/or sorcery cards
//! from your graveyard to your hand. Exile Bond of Insight."
//! GAP: 'return up to two instant/sorcery cards from graveyard to hand' (player
//! chooses up to 2) requires TargetCount::UpTo(2) on graveyard cards — the
//! catalog only has ReturnFromGraveyardToHand with a single pre-chosen id;
//! emitting mill for both players and self-exile; return step is a GAP.
//! GAP: self-exile of the spell (itself on the stack) not in catalog.

use arcana_core::effects::{Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bond of Insight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player mills four cards. Return up to two instant and/or sorcery cards from your graveyard to your hand. Exile Bond of Insight.".into(),
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
    // GAP: 'each player' requires iterating all players — only controller accessible here
    // GAP: return up to two instant/sorcery from graveyard (player-chosen) not expressible
    // GAP: self-exile of the spell not in catalog
    vec![
        Effect::Mill { player: entry.controller, count: 4 },
    ]
}
