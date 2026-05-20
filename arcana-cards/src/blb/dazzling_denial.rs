//! Dazzling Denial — `{1}{U}` instant. "Counter target spell unless
//! its controller pays {2}. If you control a Bird, counter that spell
//! unless its controller pays {4} instead."
//!
//! The Bird-control upgrade to {4} needs a resolution-time conditional
//! whose condition variant isn't in the catalog; the base soft counter
//! ({2}) is emitted.

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
    let name = reg.interner_mut().intern("Dazzling Denial");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell unless its controller pays {2}. If you control a Bird, counter that spell unless its controller pays {4} instead.".into(),
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

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: "If you control a Bird ... {4} instead" — no catalog
    // conditional condition for "you control a Bird".
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{2}").expect("valid cost"),
    }]
}
