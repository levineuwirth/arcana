//! Cabal Conditioning — `{6}{B}` sorcery. "Any number of target players
//! each discard a number of cards equal to the greatest mana value among
//! permanents you control."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cabal Conditioning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Any number of target players each discard a number of cards equal to the greatest mana value among permanents you control.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Any,
                    controller: None,
                }],
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
    let count = script::max_cmc_of(
        state,
        &ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Player(p) = t {
                Some(Effect::Discard {
                    player: *p,
                    count,
                    choice: DiscardChoice::ControllerChooses,
                })
            } else {
                None
            }
        })
        .collect()
}
