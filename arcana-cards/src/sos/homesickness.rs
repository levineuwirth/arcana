//! Homesickness — `{4}{U}{U}` instant. "Target player draws two cards. Tap up
//! to two target creatures. Put a stun counter on each of them."
//!
//! # GAP: stun counters (CounterKind::Stun) not listed in the catalog;
//! AddCounters is restricted to CounterKind::PlusOnePlusOne. Tap effects
//! are expressible. Best-effort: draw + tap only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Homesickness");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player draws two cards. Tap up to two target creatures. Put a \
                       stun counter on each of them."
                    .into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(2),
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
    // GAP: stun counter (CounterKind::Stun not available)
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        match t {
            TargetChoice::Player(p) => {
                effects.push(Effect::DrawCards { player: *p, count: 2 });
            }
            TargetChoice::Object(id) => {
                effects.push(Effect::Tap { target: *id });
            }
            _ => {}
        }
    }
    effects
}
