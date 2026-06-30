//! Syncopate — `{X}{U}` instant. "Counter target spell unless its controller
//! pays {X}." (The "if countered this way, exile it" rider is GAP'd —
//! `CounterUnlessPays` routes the countered spell to the graveyard.)

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
    let name = reg.interner_mut().intern("Syncopate");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell unless its controller pays {X}.".into(),
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
    let x = entry.x_value.unwrap_or(0);
    match entry.targets.targets.first() {
        Some(TargetChoice::Object(id)) => vec![Effect::CounterUnlessPays {
            target: *id,
            cost: ManaCost::empty().with_generic_delta(x as i32),
        }],
        _ => Vec::new(),
    }
}
