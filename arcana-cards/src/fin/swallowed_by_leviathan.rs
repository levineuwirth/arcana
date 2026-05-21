//! Swallowed by Leviathan — `{2}{U}` instant. "Choose target spell.
//! Surveil 2, then counter the chosen spell unless its controller
//! pays {1} for each card in your graveyard."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swallowed by Leviathan");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target spell. Surveil 2, then counter the chosen spell unless its controller pays {1} for each card in your graveyard.".into(),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let n = script::graveyard_size(state, entry.controller);
    // Build a {N} tax cost where N = cards in your graveyard.
    let tax_str = format!("{{{}}}", n);
    let cost = ManaCost::parse(&tax_str).expect("valid cost");
    vec![
        Effect::Surveil { player: entry.controller, count: 2 },
        Effect::CounterUnlessPays { target: *id, cost },
    ]
}
