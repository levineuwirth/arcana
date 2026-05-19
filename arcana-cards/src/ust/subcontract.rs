//! Subcontract — `{B}` sorcery.
//! "A person outside the game looks at target opponent's hand and chooses a nonland
//! card from it. That player discards that card."
//!
//! GAP: "a person outside the game" look at hand and choose — no external-player
//! choice variant in Effect catalog. Approximated as OpponentChooses discard of 1
//! from a targeted opponent. "Nonland" filter on hand cards not directly expressible.

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Subcontract");
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
                text: "A person outside the game looks at target opponent's hand and chooses a nonland card from it. That player discards that card.".into(),
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
    // GAP: "person outside the game" choice — not expressible; approximated as
    // OpponentChooses (closest available).
    // GAP: "nonland" card filter on discard — DiscardChoice has no filter variant.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::OpponentChooses }]
}
