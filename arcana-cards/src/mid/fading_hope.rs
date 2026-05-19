//! Fading Hope — `{U}` instant. "Return target creature to its owner's
//! hand. If its mana value was 3 or less, scry 1."
//!
//! The bounce is fully expressible via `Effect::ReturnToHand`. The
//! conditional scry ("if its mana value was 3 or less") requires
//! `Effect::Conditional` with a mana-value predicate on the target
//! object, which is not listed in the `condition` surface of the catalog.
//!
//! # GAP: ConditionalManaValue — no `Condition` variant for checking a
//! permanent's mana value is available; the conditional scry 1 clause
//! is dropped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fading Hope");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature to its owner's hand. If its mana value was 3 or less, scry 1.".into(),
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
    // GAP: ConditionalManaValue — cannot check target's mana value; scry 1 clause omitted.
    vec![Effect::ReturnToHand { target: *id }]
}
