//! Fell the Pheasant — `{1}{G}` instant. "This spell costs {1} less to cast if it targets a
//! creature with flying. Destroy target creature with flying. Create a Food token."
//!
//! # GAP
//! - No support for cost reduction based on target's keyword (flying filter on cast cost)
//! - Food token requires an activated ability ({2}, {T}, Sacrifice this: Gain 3 life)
//!   which is not expressible via TokenDefinition.abilities in the current catalog
//! - Flying filter on TargetFilter::Creature not available

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fell the Pheasant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "This spell costs {1} less to cast if it targets a creature with flying. Destroy target creature with flying. Create a Food token.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: flying filter on target; Food token with activated ability
    vec![Effect::DestroyPermanent { target: *id }]
}
