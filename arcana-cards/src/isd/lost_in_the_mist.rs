//! Lost in the Mist — `{3}{U}{U}` instant. "Counter target spell. Return
//! target permanent to its owner's hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lost in the Mist");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Return target permanent to its owner's hand.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Spell(ObjectFilter::default()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::permanent()),
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
    let targets = &entry.targets.targets;
    if targets.len() < 2 { return Vec::new(); }
    let TargetChoice::Object(spell) = &targets[0] else { return Vec::new(); };
    let TargetChoice::Object(perm) = &targets[1] else { return Vec::new(); };
    vec![
        Effect::Counter { target: *spell },
        Effect::ReturnToHand { target: *perm },
    ]
}
