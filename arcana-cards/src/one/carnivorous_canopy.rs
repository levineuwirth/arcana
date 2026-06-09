//! Carnivorous Canopy — `{2}{G}` sorcery. "Destroy target artifact,
//! enchantment, or creature with flying. If that permanent's mana
//! value was 3 or less, proliferate."

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
    let name = reg.interner_mut().intern("Carnivorous Canopy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target artifact, enchantment, or creature with flying. If that permanent's mana value was 3 or less, proliferate.".into(),
            // "artifact, enchantment, or creature with flying" — the
            // type+keyword disjunction is expressed via the `custom`
            // predicate (layer-aware flying check for the creature arm).
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

fn artifact_enchantment_or_flier(obj: &GameObject, state: &GameState) -> bool {
    let t = obj.characteristics.types.0;
    if t & (TypeLine::ARTIFACT | TypeLine::ENCHANTMENT) != 0 {
        return true;
    }
    t & TypeLine::CREATURE != 0 && state.has_keyword(obj.id, &KeywordAbility::Flying)
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: proliferate (no Effect::Proliferate) and "mana value was
    // 3 or less at time of destruction" predicate. Emit destroy only.
    vec![Effect::DestroyPermanent { target: *id }]
}
