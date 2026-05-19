//! Aerial Assault — `{2}{W}` sorcery. "Destroy target tapped creature.
//! You gain 1 life for each creature you control with flying."
//!
//! GAP: target filter for "tapped creature" not available (no tapped
//! ObjectFilter); life gain equal to count of flying creatures requires
//! runtime enumeration. Using target_creature() and emitting destroy only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aerial Assault");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target tapped creature. You gain 1 life for each creature you control with flying.".into(),
                // GAP: no TargetFilter for tapped creatures
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
    // GAP: life gain equal to number of flying creatures you control
    // requires runtime count query; not expressible.
    vec![Effect::DestroyPermanent { target: *id }]
}
