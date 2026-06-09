//! Broken Wings — `{2}{G}` instant. "Destroy target artifact, enchantment, or
//! creature with flying."
//!
//! The disjunction's keyword-qualified third arm (creature WITH FLYING) is
//! enforced via the filter's `custom` predicate: artifacts and enchantments
//! always qualify; an object that's only a creature must have flying
//! (layer-aware via `GameState::has_keyword`).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, GameObject};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Broken Wings");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target artifact, enchantment, or creature with flying.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter {
                        custom: Some(artifact_enchantment_or_flier),
                        ..ObjectFilter::permanent().with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::ENCHANTMENT | TypeLine::CREATURE,
                        ))
                    }),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

/// Disjunction arms: artifact or enchantment (any), or creature with flying.
fn artifact_enchantment_or_flier(obj: &GameObject, state: &GameState) -> bool {
    let types = obj.characteristics.types.0;
    if types & (TypeLine::ARTIFACT | TypeLine::ENCHANTMENT) != 0 {
        return true;
    }
    types & TypeLine::CREATURE != 0
        && state.has_keyword(obj.id, &KeywordAbility::Flying)
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
