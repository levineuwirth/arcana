//! Essence Capture — `{U}{U}` instant. Counter target creature spell.
//! Put a +1/+1 counter on up to one target creature you control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Essence Capture");
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
                text: "Counter target creature spell. Put a +1/+1 counter on up to one target creature you control.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Spell(
                            ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::UpTo(1),
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
    let targets = &entry.targets.targets;
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(a)) = targets.first() {
        effects.push(Effect::Counter { target: *a });
    }
    if let Some(TargetChoice::Object(b)) = targets.get(1) {
        effects.push(Effect::AddCounters {
            target: *b,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects
}
