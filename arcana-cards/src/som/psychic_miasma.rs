//! Psychic Miasma — `{1}{B}` sorcery. "Target player discards a card. If a
//! land card is discarded this way, return Psychic Miasma to its owner's
//! hand."
//!
//! # GAP: ConditionalReturnSelfToHand — no Effect variant for 'if the
//!   discarded card was a land, return this spell to its owner's hand'

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Psychic Miasma");
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
                text: "Target player discards a card. If a land card is discarded this way, return Psychic Miasma to its owner's hand.".into(),
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
    // GAP: ConditionalReturnSelfToHand — no Effect variant for returning this spell based on
    // what the target player discarded (land check on discarded card)
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard { player: *p, count: 1, choice: DiscardChoice::ControllerChooses }]
}
