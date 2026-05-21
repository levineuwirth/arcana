//! Windswift Slice — `{2}{G}` instant. "Target creature you control
//! deals damage equal to its power to target creature you don't
//! control. Create a number of 1/1 green Elf Warrior creature tokens
//! equal to the amount of excess damage dealt this way." The
//! excess-damage tracking isn't in the catalog — best effort:
//! one-way fight (the token rider is the GAP).

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
    let name = reg.interner_mut().intern("Windswift Slice");
    let _elf = reg.interner_mut().intern("Elf");
    let _warrior = reg.interner_mut().intern("Warrior");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature you don't control. Create a number of 1/1 green Elf Warrior creature tokens equal to the amount of excess damage dealt this way.".into(),
                target_requirements: vec![
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = &entry.targets.targets;
    let Some(TargetChoice::Object(a)) = targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Object(b)) = targets.get(1) else { return Vec::new(); };
    let amount = script::power_of(state, *a).max(0) as u32;
    // GAP: excess-damage tracking + Elf Warrior tokens equal to that excess.
    vec![Effect::DealDamage {
        source: *a,
        target: DamageTarget::Object(*b),
        amount,
    }]
}
