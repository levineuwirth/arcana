//! Ob Nixilis's Cruelty — `{2}{B}` instant. "Target creature gets
//! -5/-5 until end of turn. If that creature would die this turn,
//! exile it instead."
//!
//! The replacement "exile instead of dies" rider is not expressible;
//! only the -5/-5 is emitted.

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
    let name = reg.interner_mut().intern("Ob Nixilis's Cruelty");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature gets -5/-5 until end of turn. If that creature would die this turn, exile it instead.".into(),
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
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "if it would die this turn, exile it instead" replacement
    // rider is not expressible.
    vec![Effect::Pump {
        target: *id,
        power: -5,
        toughness: -5,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
