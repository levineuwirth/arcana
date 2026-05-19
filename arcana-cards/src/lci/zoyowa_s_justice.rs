//! Zoyowa's Justice — `{1}{R}` instant. "The owner of target artifact or creature
//! with mana value 1 or greater shuffles it into their library. Then that player
//! discovers X, where X is its mana value."
//!
//! # GAP: PutOnBottomOfLibrary/shuffle-into-library and Discover effect are
//! not expressible via the catalog. We emit PutOnBottomOfLibrary as a
//! closest approximation and note the Discover gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zoyowa's Justice");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "The owner of target artifact or creature with mana value 1 or greater shuffles it into their library. Then that player discovers X, where X is its mana value.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: Discover X effect not in catalog; shuffle-into-library approximated
    vec![Effect::PutOnBottomOfLibrary { target: *id }]
}
