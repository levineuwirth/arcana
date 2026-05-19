//! Appetite for Brains — `{B}` sorcery. "Target opponent reveals their hand.
//! You choose a card from it with mana value 4 or greater and exile that card."
//!
//! # GAP: "target opponent reveals hand, you choose a card with mana value ≥4
//! to exile" — targeted hand inspection with controller-chooses-a-specific-card
//! exile is not expressible; only random/opponent-chooses discard variants exist.
//! Best effort: forced discard (opponent chooses) with no exile.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Appetite for Brains");
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
                text: "Target opponent reveals their hand. You choose a card from it with mana value 4 or greater and exile that card.".into(),
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
    // GAP: hand-inspection + controller-chooses card with CMC ≥4 to exile from hand
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::ControllerChooses }]
}
