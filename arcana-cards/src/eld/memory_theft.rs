//! Memory Theft — `{2}{B}` sorcery. "Target opponent reveals their hand. You choose a nonland
//! card from it. That player discards that card. You may put a card that has an Adventure that
//! player owns from exile into that player's graveyard."
//! GAP: inspect opponent's hand and choose a specific card to discard (targeted discard) not in catalog.
//! GAP: move exile card with Adventure to graveyard not in catalog.
//! Best effort: opponent discards 1 (OpponentChooses).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Memory Theft");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent reveals their hand. You choose a nonland card from it. That player discards that card. You may put a card that has an Adventure that player owns from exile into that player's graveyard.".into(),
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
    // GAP: controller-chooses-which-card discard (not random / opponent's choice)
    // GAP: exile-with-Adventure → graveyard
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::OpponentChooses,
    }]
}
