//! Elminster's Simulacrum — `{4}{U}{U}` instant. "For each opponent, you
//! create a token that's a copy of up to one target creature that player
//! controls."
//!
//! Implemented via [`Effect::CopyPermanent`] over the chosen target
//! creature(s): one token copy is minted per targeted creature. The "up to
//! one target creature that player controls" clause is modeled as up-to-one
//! creature target (the common two-player case targets a single opponent's
//! creature); each declared target yields one `CopyPermanent`.
//!
//! GAP: the `CopyPermanent` primitive places the token under the COPIED
//! creature's controller (the opponent), not under you ("you create a
//! token"). The control attribution is a fidelity gap. Multi-opponent
//! "for each opponent" targeting beyond the declared targets is bounded by
//! the target list.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elminster's Simulacrum");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "For each opponent, you create a token that's a copy of up to \
                   one target creature that player controls."
                .into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::CopyPermanent { target: *id }),
            _ => None,
        })
        .collect()
}
