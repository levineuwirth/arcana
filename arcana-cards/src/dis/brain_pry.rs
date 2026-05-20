//! Brain Pry — `{1}{B}` sorcery. "Choose a nonland card name. Target
//! player reveals their hand. That player discards a card with that
//! name. If they can't, you draw a card."
//!
//! Name-pick + conditional-on-found discard is not expressible from
//! catalog Effects. Best-effort: a controller-chooses discard against
//! the targeted player.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brain Pry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose a nonland card name. Target player reveals their hand. That player discards a card with that name. If they can't, you draw a card.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: card-name pick + "if they can't, draw" conditional not in catalog.
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::OpponentChooses }]
}
