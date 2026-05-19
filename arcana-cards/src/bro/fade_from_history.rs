//! Fade from History — `{2}{G}{G}` sorcery, "Each player who controls an artifact or
//! enchantment creates a 2/2 green Bear creature token. Then destroy all artifacts and enchantments."
//!
//! GAP: "each player who controls an artifact or enchantment creates a token" requires
//! iterating over players with a condition, which is not expressible with available Effect variants.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fade from History");
    let _bear = reg.interner_mut().intern("Bear");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player who controls an artifact or enchantment creates a 2/2 green Bear creature token. Then destroy all artifacts and enchantments.".into(),
                target_requirements: vec![],
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
    // GAP: conditional per-player token creation (each player who controls an artifact or enchantment) not expressible
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let enchantment_filter = ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into());
    let artifact_ids = script::ids_matching(state, &artifact_filter, entry.controller);
    let enchantment_ids = script::ids_matching(state, &enchantment_filter, entry.controller);
    let mut effects: Vec<Effect> = artifact_ids.into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect();
    effects.extend(enchantment_ids.into_iter().map(|id| Effect::DestroyPermanent { target: id }));
    effects
}
