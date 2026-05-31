//! Mutant's Prey — `{G}` instant. "Target creature you control with a
//! +1/+1 counter on it fights target creature an opponent controls."
//!
//! Two targets: a creature you control (constrained to those bearing a
//! +1/+1 counter) and a creature an opponent controls. They fight via
//! `Effect::Fight`. The "with a +1/+1 counter on it" restriction on the
//! first target is not expressible in the `ObjectFilter` builder
//! surface — the controller/type constraints are applied; the
//! counter-presence restriction is left off.

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
    let name = reg.interner_mut().intern("Mutant's Prey");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control with a +1/+1 counter on it fights target creature an opponent controls.".into(),
                target_requirements: vec![
                    // GAP: "with a +1/+1 counter on it" restriction on the
                    // first target is not expressible in the filter builder.
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
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
    let mut ids = entry.targets.targets.iter().filter_map(|t| match t {
        TargetChoice::Object(id) => Some(*id),
        _ => None,
    });
    let (Some(a), Some(b)) = (ids.next(), ids.next()) else { return Vec::new(); };
    vec![Effect::Fight { a, b }]
}
