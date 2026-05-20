//! Ixidor's Will — `{2}{U}` instant. "Counter target spell unless its
//! controller pays {2} for each Wizard on the battlefield."
//!
//! Dynamic tax: 2 generic mana per Wizard. We compute the count via
//! script::subtype_filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ixidor's Will");
    let _wizard = reg.interner_mut().intern("Wizard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell unless its controller pays {2} for each Wizard on the battlefield.".into(),
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

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    let wizards = script::count_matching(
        state,
        &script::subtype_filter(reg, "Wizard"),
        entry.controller,
    );
    let cost_str = format!("{{{}}}", wizards * 2);
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse(&cost_str).expect("valid cost"),
    }]
}
