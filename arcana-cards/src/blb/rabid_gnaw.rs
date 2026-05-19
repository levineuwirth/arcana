//! Rabid Gnaw — `{1}{R}` instant. "Target creature you control gets +1/+0 until
//! end of turn. Then it deals damage equal to its power to target creature you
//! don't control."
//!
//! # GAP: "deals damage equal to its power" requires reading the pumped
//! creature's power at resolution time, which is not expressible directly.
//! We emit the Pump and note the variable-damage fight-like clause as a gap.

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
    let name = reg.interner_mut().intern("Rabid Gnaw");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 until end of turn. Then it deals damage equal to its power to target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
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
    let mut targets = entry.targets.targets.iter();
    let Some(t1) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(id1) = t1 else { return Vec::new(); };
    // GAP: "deals damage equal to its power" (variable from game state) not expressible
    vec![Effect::Pump {
        target: *id1,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
