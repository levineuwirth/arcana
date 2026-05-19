//! Peel from Reality — `{1}{U}` instant. "Return target creature you
//! control and target creature you don't control to their owners' hands."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Peel from Reality");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature you control and target creature you don't control to their owners' hands.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
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
    let targets = &entry.targets.targets;
    let Some(t0) = targets.first() else { return Vec::new(); };
    let Some(t1) = targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(a) = t0 else { return Vec::new(); };
    let TargetChoice::Object(b) = t1 else { return Vec::new(); };
    vec![
        Effect::ReturnToHand { target: *a },
        Effect::ReturnToHand { target: *b },
    ]
}
