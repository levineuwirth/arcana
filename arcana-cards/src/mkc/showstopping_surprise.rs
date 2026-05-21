//! Showstopping Surprise — `{3}{R}{R}` instant. "Choose target
//! creature you control. Turn it face up if it's face down. Then it
//! deals damage equal to its power to each other creature." GAP:
//! 'turn face up' primitive. Express the damage half via ForEach.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Showstopping Surprise");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control. Turn it face up if it's face down. Then it deals damage equal to its power to each other creature.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'turn face up' not in catalog.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let amount = script::power_of(state, id).max(0) as u32;
    let mut others = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    others.retain(|o| *o != id);
    vec![Effect::ForEach {
        targets: others,
        effect: Box::new(Effect::DealDamage {
            source: id,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount,
        }),
    }]
}
