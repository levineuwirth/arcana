//! Undercity Plunder — `{1}{B}` sorcery. "Target opponent discards a card.
//! Then they may discard an additional card. If they don't, conjure a duplicate
//! of a random card from their library into your hand. It perpetually gains
//! 'You may spend mana as though it were mana of any color to cast this spell.'"
//!
//! # GAP: "may discard" optional-discard branch for the opponent
//! # GAP: Conjure (digital-only: create a copy of a random library card in hand)
//! # GAP: Perpetually modify a card

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undercity Plunder");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent discards a card. Then they may discard an additional card. If they don't, conjure a duplicate of a random card from their library into your hand. It perpetually gains \"You may spend mana as though it were mana of any color to cast this spell.\"".into(),
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
    // GAP: optional "may discard additional card" branch for opponent
    // GAP: Conjure (digital-only mechanic — random library card copy into hand)
    // GAP: perpetually modify a card
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::ControllerChooses }]
}
