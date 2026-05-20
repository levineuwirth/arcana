//! Smashing Success — `{3}{R}` instant. "Destroy target artifact or
//! land. If an artifact is destroyed this way, create a Treasure
//! token."
//!
//! Only the destruction is expressed; the "if an artifact was
//! destroyed, create a Treasure" conditional cannot inspect what was
//! destroyed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smashing Success");
    let _treasure = reg.interner_mut().intern("Treasure");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target artifact or land. If an artifact is destroyed this way, create a Treasure token.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::LAND)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "if an artifact is destroyed this way, create a Treasure" —
    // cannot inspect the destroyed object's type post-destruction.
    vec![Effect::DestroyPermanent { target: *id }]
}
