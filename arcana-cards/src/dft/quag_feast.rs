//! Quag Feast — `{1}{B}` sorcery. "Choose target creature, planeswalker,
//! or Vehicle. Mill two cards, then destroy the chosen permanent if its
//! mana value is less than or equal to the number of cards in your
//! graveyard."
//!
//! The mill is expressible; the conditional destroy (compare the chosen
//! permanent's mana value to your graveyard size, then destroy) requires
//! a per-resolution numeric comparison gate on the target that the
//! Effect catalog does not provide.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quag Feast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature, planeswalker, or Vehicle. Mill two cards, then destroy the chosen permanent if its mana value is less than or equal to the number of cards in your graveyard.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
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
    // GAP: the conditional destroy ("destroy the chosen permanent if its
    // mana value <= number of cards in your graveyard") needs a
    // resolution-time numeric comparison between the target's mana value
    // and graveyard size as a gate on Effect::DestroyPermanent; no such
    // conditional/comparison primitive over a target's CMC is available.
    // Only the unconditional mill is emitted.
    vec![Effect::Mill { player: entry.controller, count: 2 }]
}
