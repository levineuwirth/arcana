//! Pit Fight — `{1}{R/G}` instant. "Target creature you control
//! fights another target creature." Each deals damage equal to its
//! power to the other.

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
    let name = reg.interner_mut().intern("Pit Fight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control fights another target creature.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_creature(),
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
    let Some(a) = ids.next() else { return Vec::new(); };
    let Some(b) = ids.next() else { return Vec::new(); };
    vec![Effect::Fight { a, b }]
}
