//! Paradoxical Outcome — `{3}{U}` instant. "Return any number of target
//! nonland, nontoken permanents you control to their owners' hands. Draw
//! a card for each card returned to your hand this way."
//!
//! # GAP: "nonland nontoken" ObjectFilter predicate not available.
//! # GAP: draw count equals number of permanents actually returned (dynamic).
//! Emitting ReturnToHand for each target and drawing for each target as
//! approximation; the nonland/nontoken restriction is a targeting gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paradoxical Outcome");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return any number of target nonland, nontoken permanents you control to their owners' hands. Draw a card for each card returned to your hand this way.".into(),
                // GAP: nonland nontoken filter not available; using general permanent filter
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Any,
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
    let targets: Vec<_> = entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t { Some(*id) } else { None }
    }).collect();
    let count = targets.len() as u32;
    let mut effects: Vec<Effect> = targets.into_iter()
        .map(|id| Effect::ReturnToHand { target: id })
        .collect();
    if count > 0 {
        effects.push(Effect::DrawCards { player: entry.controller, count });
    }
    effects
}
