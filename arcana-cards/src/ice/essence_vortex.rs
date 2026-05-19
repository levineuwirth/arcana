//! Essence Vortex — `{1}{U}{B}` instant. "Destroy target creature unless its controller pays
//! life equal to its toughness. A creature destroyed this way can't be regenerated."
//!
//! GAP: "Pay life equal to toughness" is a life-payment cost variant not covered by
//! CounterUnlessPays (which takes a ManaCost). The "can't be regenerated" rider is also
//! not modeled. Best-effort: destroy the creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Essence Vortex");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature unless its controller pays life equal to its toughness.".into(),
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
    // GAP: "Unless its controller pays life equal to its toughness" — life-payment cost variant
    // not supported by CounterUnlessPays (mana-only). Emitting plain destroy as best-effort.
    // GAP: "Can't be regenerated" rider not modeled.
    vec![Effect::DestroyPermanent { target: *id }]
}
