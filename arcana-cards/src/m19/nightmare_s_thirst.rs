//! Nightmare's Thirst — `{B}` instant. "You gain 1 life. Target creature gets -X/-X until
//! end of turn, where X is the amount of life you gained this turn."
//!
//! GAP: -X/-X where X is total life gained this turn is a dynamic amount tracked across
//! the turn; not expressible with fixed Pump values.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nightmare's Thirst");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You gain 1 life. Target creature gets -X/-X until end of turn, where X is the amount of life you gained this turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: -X/-X where X is total life gained this turn is a dynamic amount not in catalog;
    // using the minimum expressible value (-1/-1) as placeholder for the Pump, but this
    // misrepresents the card. Return gain life only + GAP for the -X/-X portion.
    vec![
        Effect::GainLife { player: entry.controller, amount: 1 },
        Effect::Pump {
            target: *id,
            power: -1,
            toughness: -1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ]
}
