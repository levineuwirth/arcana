//! Structural Distortion — `{3}{R}` sorcery, "Exile target artifact or
//! land. Structural Distortion deals 2 damage to that permanent's
//! controller." Partial: ExilePermanent expressed; damage to the
//! controller of the exiled permanent requires state lookup at resolve
//! time not supported in the current catalog signature.
//!
//! # GAP: deal damage to the controller of the just-exiled permanent not in catalog.

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
    let name = reg.interner_mut().intern("Structural Distortion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target artifact or land. Structural Distortion deals 2 damage to that permanent's controller.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::ARTIFACT | TypeLine::LAND),
                        ),
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
    // GAP: deal 2 damage to the controller of the exiled permanent not in catalog
    vec![Effect::ExilePermanent { target: *id }]
}
