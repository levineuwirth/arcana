//! Road Rage — `{R}` instant. "Road Rage deals X damage to target
//! creature or planeswalker, where X is 2 plus the number of Mounts
//! and Vehicles you control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::targets::ObjectFilter;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Road Rage");
    let _mount = reg.interner_mut().intern("Mount");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Road Rage deals X damage to target creature or planeswalker, where X is 2 plus the number of Mounts and Vehicles you control.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().with_types_any(
                        arcana_core::types::TypeLine(
                            TypeLine::CREATURE | TypeLine::PLANESWALKER,
                        ),
                    ),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(t) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = t else { return Vec::new(); };
    let mounts = script::count_matching(
        state,
        &script::subtype_filter(reg, "Mount")
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let vehicles = script::count_matching(
        state,
        &script::subtype_filter(reg, "Vehicle")
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    let amount = 2 + mounts + vehicles;
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}
