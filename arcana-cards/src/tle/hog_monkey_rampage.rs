//! Hog-Monkey Rampage — `{1}{R/G}` instant. Choose target creature you
//! control and target creature an opponent controls. Put a +1/+1 counter
//! on the creature you control if it has power 4 or greater. Then those
//! creatures fight each other.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hog-Monkey Rampage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature you control and target creature an opponent controls. Put a +1/+1 counter on the creature you control if it has power 4 or greater. Then those creatures fight each other.".into(),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(t1) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(t2) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(a) = t1 else { return Vec::new(); };
    let TargetChoice::Object(b) = t2 else { return Vec::new(); };
    let a = *a;
    let b = *b;
    let mut effects: Vec<Effect> = Vec::new();
    if script::power_of(state, a) >= 4 {
        effects.push(Effect::AddCounters {
            target: a,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects.push(Effect::Fight { a, b });
    effects
}
