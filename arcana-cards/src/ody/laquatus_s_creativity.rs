//! Laquatus's Creativity — `{4}{U}` sorcery.
//! "Target player draws cards equal to the number of cards in their hand,
//! then discards that many cards."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Laquatus's Creativity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player draws cards equal to the number of cards in their hand, then discards that many cards.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let target_player = match target {
        arcana_core::targets::TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    let hand = script::hand_size(state, target_player);
    if hand == 0 {
        return Vec::new();
    }
    vec![
        Effect::DrawCards { player: target_player, count: hand },
        Effect::Discard {
            player: target_player,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
