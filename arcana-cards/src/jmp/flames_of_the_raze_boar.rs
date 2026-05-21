//! Flames of the Raze-Boar — `{5}{R}` instant. Deals 4 damage to target
//! creature an opponent controls. Then deals 2 damage to each other
//! creature that player controls if you control a creature with power 4
//! or greater.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Flames of the Raze-Boar");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flames of the Raze-Boar deals 4 damage to target creature an opponent controls. Then Flames of the Raze-Boar deals 2 damage to each other creature that player controls if you control a creature with power 4 or greater.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let mut effects = vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(id),
        amount: 4,
    }];
    let has_power_4 = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_min_power(4),
        entry.controller,
    ) > 0;
    if has_power_4 {
        let opponent = script::target_controller(state, id, entry.controller);
        let others = script::ids_matching(
            state,
            &ObjectFilter::creature()
                .controlled_by(ControllerConstraint::You),
            opponent,
        );
        for other in others {
            if other == id {
                continue;
            }
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(other),
                amount: 2,
            });
        }
    }
    effects
}
