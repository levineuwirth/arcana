//! Throw from the Saddle — `{1}{G}` sorcery. "Target creature you control gets
//! +1/+1 until end of turn. Put a +1/+1 counter on it instead if it's a Mount.
//! Then it deals damage equal to its power to target creature you don't control."
//!
//! # GAP: conditional effect (Pump vs AddCounters based on Mount subtype check)
//! and damage equal to the creature's power (dynamic value) are not expressible.
//! Fight is used as the closest approximation for the damage portion; the
//! Mount-conditional counter is omitted in favor of a plain Pump.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Throw from the Saddle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+1 until end of turn. Put a +1/+1 counter on it instead if it's a Mount. Then it deals damage equal to its power to target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
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
    let targets = &entry.targets.targets;
    let (Some(t0), Some(t1)) = (targets.get(0), targets.get(1)) else { return Vec::new(); };
    let (TargetChoice::Object(id0), TargetChoice::Object(id1)) = (t0, t1) else { return Vec::new(); };
    // GAP: conditional Mount check for counter vs pump not expressible
    // GAP: damage = power of creature (dynamic amount) approximated with Fight
    vec![
        Effect::Pump { target: *id0, power: 1, toughness: 1, duration: Duration::EndOfTurn, keywords: vec![] },
        Effect::Fight { a: *id0, b: *id1 },
    ]
}
