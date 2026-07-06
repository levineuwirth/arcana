//! Memory Theft — `{2}{B}` sorcery. "Target opponent reveals their hand. You
//! choose a nonland card from it. That player discards that card. You may put
//! a card that has an Adventure that player owns from exile into that player's
//! graveyard."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
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
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
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
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    // "You choose a nonland card, that player discards it" — approximated by an
    // opponent-chooses discard on the target player.
    // GAP: the "nonland" restriction on the chosen card, and the optional Adventure
    // exile-to-graveyard clause, are not expressible.
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::OpponentChooses }]
}
