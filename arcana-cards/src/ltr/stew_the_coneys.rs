//! Stew the Coneys — `{2}{G}` instant. "Target creature you control
//! deals damage equal to its power to target creature you don't
//! control. Create a Food token."

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
    let name = reg.interner_mut().intern("Stew the Coneys");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its power to target creature you don't control. Create a Food token.".into(),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(own)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(foe)) = entry.targets.targets.get(1) else {
        return Vec::new();
    };
    let power = script::power_of(state, *own).max(0) as u32;
    vec![Effect::DealDamage {
        source: *own,
        target: DamageTarget::Object(*foe),
        amount: power,
    }]
    // GAP: "Create a Food token" — Food is an artifact token carrying
    // an activated ability ("{2}, {T}, Sacrifice: gain 3 life") which
    // TokenDefinition.abilities cannot express; the token is omitted.
}
