//! Despise — `{B}` sorcery. "Target opponent reveals their hand. You
//! choose a creature or planeswalker card from it. That player discards
//! that card."
//!
//! # GAP: "look at opponent's hand and choose a specific card to discard"
//! — OpponentChooses discard is the closest, but does not let the
//! controller choose. Using ControllerChooses as approximation since
//! the controller picks from the revealed hand.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Despise");
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
                text: "Target opponent reveals their hand. You choose a creature or planeswalker card from it. That player discards that card.".into(),
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
    let TargetChoice::Player(opponent) = target else { return Vec::new(); };
    // GAP: controller chooses a specific creature/planeswalker from opponent's hand to discard
    vec![Effect::Discard { player: *opponent, count: 1, choice: DiscardChoice::OpponentChooses }]
}
