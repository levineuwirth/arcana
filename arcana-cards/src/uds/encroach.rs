//! Encroach — `{B}` sorcery, "Target player reveals their hand. You choose a
//! nonbasic land card from it. That player discards that card."
//!
//! GAP: reveal hand and choose a specific nonbasic-land card to discard —
//! no ExileFromHand or choose-from-revealed-hand variant. Best effort:
//! target player discards 1 card (OpponentChooses).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Encroach");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player reveals their hand. You choose a nonbasic land card from it. That player discards that card.".into(),
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
    let TargetChoice::Player(target_player) = target else { return Vec::new(); };
    vec![
        // GAP: reveal hand and choose a specific nonbasic land card to discard
        Effect::Discard { player: *target_player, count: 1, choice: DiscardChoice::OpponentChooses },
    ]
}
