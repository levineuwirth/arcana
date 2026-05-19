//! Double Cross — `{3}{B}{B}` sorcery. "Choose another player. Look at
//! that player's hand and choose a card other than a basic land card
//! from it. They discard that card. At the beginning of the first upkeep
//! in your next game with that player, look at that player's hand and
//! choose a card other than a basic land card from it. They discard that
//! card."
//!
//! GAP: look at a player's hand and choose a specific card from it;
//! cross-game triggered ability ("next game") are not expressible.
//! Best-effort: target player discards a card (controller chooses).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Double Cross");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose another player. Look at that player's hand and choose a card other than a basic land card from it. They discard that card. At the beginning of the first upkeep in your next game with that player, look at that player's hand and choose a card other than a basic land card from it. They discard that card.".into(),
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
    // GAP: look at hand and choose a specific non-basic-land card
    // GAP: cross-game triggered ability for next game
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
