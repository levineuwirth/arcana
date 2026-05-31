//! Nightmare's Thirst — `{B}` instant. "You gain 1 life. Target
//! creature gets -X/-X until end of turn, where X is the amount of life
//! you gained this turn."
//!
//! The life-gain is expressible, but X (the amount of life gained this
//! turn) has no `script::*` helper, so the dynamic -X/-X pump cannot be
//! computed and is GAPped rather than emitted as a wrong fixed value.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
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
    // GAP: X is "the amount of life you gained this turn" — there is no
    // script:: helper for life gained this turn, so the dynamic -X/-X
    // pump on the target creature cannot be computed. Only the life gain
    // is emitted.
    vec![Effect::GainLife {
        player: entry.controller,
        amount: 1,
    }]
}
