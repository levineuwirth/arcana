//! Lofty Denial — `{1}{U}` instant. "Counter target spell unless its controller
//! pays {1}. If you control a creature with flying, counter that spell unless
//! its controller pays {4} instead."
//!
//! GAP: Conditional counter cost based on whether controller controls a creature
//! with flying at resolution time — requires a Conditional wrapping
//! CounterUnlessPays with flying-creature check, which is not expressible with
//! the current Effect::Conditional API (no ConditionKind for board-state checks).
//! Emitting the simpler {1} form.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lofty Denial");
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
                text: "Counter target spell unless its controller pays {1}. If you control a creature with flying, counter that spell unless its controller pays {4} instead.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: conditional counter cost ({1} vs {4}) based on controlling a flying creature
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{1}").expect("valid cost"),
    }]
}
