//! Psychic Spear — `{B}` sorcery. "Target player reveals their hand.
//! You choose a Spirit or Arcane card from it. That player discards
//! that card."
//!
//! GAP: choosing a specific subtype (Spirit or Arcane) from a revealed
//! hand is not expressible — no ChooseFromHand / RevealHand effect
//! variant exists. Best-effort: target player discards a card of
//! controller's choice (OpponentChooses approximation dropped to
//! ControllerChooses to represent the caster choosing).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Psychic Spear");
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
                text: "Target player reveals their hand. You choose a Spirit or Arcane card from it. That player discards that card.".into(),
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
    // GAP: RevealHand + choose Spirit/Arcane card from hand; using discard
    // ControllerChooses as best-effort (caster picks the card).
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
