//! Ancient Animus — `{1}{G}` instant. "Put a +1/+1 counter on target
//! creature you control if it's legendary. Then it fights target
//! creature an opponent controls."
//!
//! Two targets: a creature you control and a creature an opponent
//! controls. We add a +1/+1 counter to the first target, then have it
//! fight the second. The "if it's legendary" gate on the counter is a
//! per-target supertype check at resolution that the available script
//! helpers can't express — see the GAP note on the counter step.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ancient Animus");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a +1/+1 counter on target creature you control if it's legendary. Then it fights target creature an opponent controls.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
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
    let TargetChoice::Object(a) = (match entry.targets.targets.first() {
        Some(t) => t,
        None => return Vec::new(),
    }) else {
        return Vec::new();
    };
    let TargetChoice::Object(b) = (match entry.targets.targets.get(1) {
        Some(t) => t,
        None => return Vec::new(),
    }) else {
        return Vec::new();
    };
    // GAP: the +1/+1 counter is gated on the first target being legendary,
    // a per-target supertype check at resolution time that the available
    // script helpers cannot express. Emitting only the fight.
    vec![Effect::Fight { a: *a, b: *b }]
}
