//! Last Breath — `{1}{W}` instant.
//! "Exile target creature with power 2 or less. Its controller gains 4 life."
//!
//! # GAP: TargetControllerGainsLife — after exiling the permanent, the effect says "its
//! controller gains 4 life", but the engine's Effect::GainLife requires a PlayerId known at
//! write time; the target creature's controller is only resolvable at runtime and there is
//! no Effect variant that reads "target's controller" as the recipient. Best effort: exile
//! the creature; the life gain is omitted.
//!
//! The power-2-or-less filter is also not expressible via TargetRequirement (ObjectFilter has no
//! power-ceiling predicate), so we use a plain creature target requirement.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Last Breath");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature with power 2 or less. Its controller gains 4 life.".into(),
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
    // GAP: PowerCeilingFilter — ObjectFilter has no power-ceiling predicate (power 2 or less).
    // GAP: TargetControllerGainsLife — no Effect variant for "target's controller gains N life".
    vec![Effect::ExilePermanent { target: *id }]
}
