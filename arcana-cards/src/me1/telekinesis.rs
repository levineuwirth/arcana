//! Telekinesis — `{U}{U}` instant. "Tap target creature. Prevent all combat
//! damage that would be dealt by that creature this turn. It doesn't untap
//! during its controller's next two untap steps."
//!
//! # GAP: prevent-combat-damage rider on a creature for a turn
//! # GAP: delayed-trigger "doesn't untap during next two untap steps"
//! The tap effect is expressible; the prevention and skip-untap effects are not
//! covered by the current Effect catalog. Best-effort: emit the Tap only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Telekinesis");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tap target creature. Prevent all combat damage that would be dealt by that creature this turn. It doesn't untap during its controller's next two untap steps.".into(),
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
    // GAP: prevent combat damage dealt by a specific creature this turn
    // GAP: skip-untap for next two untap steps
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Tap { target: *id }]
}
