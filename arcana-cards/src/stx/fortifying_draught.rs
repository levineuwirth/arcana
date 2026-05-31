//! Fortifying Draught — `{G}` instant. "You gain 2 life. Target creature
//! gets +X/+X until end of turn, where X is the amount of life you
//! gained this turn."
//!
//! The life-gain is expressible (`Effect::GainLife`). The pump amount is
//! dynamic — X is "the amount of life you gained this turn" — and no
//! `script::*` helper exposes life gained this turn. A fixed +2/+2 would
//! be a materially wrong card (it ignores other life gained this turn),
//! so the pump is GAP'd rather than hardcoded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fortifying Draught");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You gain 2 life. Target creature gets +X/+X until end of turn, where X is the amount of life you gained this turn.".into(),
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
    let _ = entry.targets.targets.first().and_then(|t| match t {
        TargetChoice::Object(id) => Some(*id),
        _ => None,
    });
    // GAP: the pump amount X is "the amount of life you gained this turn",
    // a dynamic value with no `script::*` helper. Emitting only the
    // expressible life-gain; the +X/+X is omitted rather than hardcoded.
    vec![Effect::GainLife { player: entry.controller, amount: 2 }]
}
