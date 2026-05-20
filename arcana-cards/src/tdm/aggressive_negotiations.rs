//! Aggressive Negotiations — `{2}{B}` sorcery.
//! "Target opponent reveals their hand. You choose a nonland card from it and
//! exile that card. Put a +1/+1 counter on up to one target creature you control."
//!
//! GAP: "target opponent reveals their hand, you choose a nonland card and exile it"
//! — this is a targeted discard/exile from hand (Thoughtseize-style) which requires
//! a choice from the opponent's revealed hand. No Effect variant supports
//! exile-from-opponent's-hand with controller choice.
//! The +1/+1 counter on a creature is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aggressive Negotiations");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target opponent reveals their hand. You choose a nonland card from it and exile that card. Put a +1/+1 counter on up to one target creature you control.".into(),
                target_requirements: vec![
                    TargetRequirement::target_player(),
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
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
    // GAP: exile a chosen nonland card from opponent's revealed hand not in Effect catalog.
    let mut effects = Vec::new();
    for t in entry.targets.targets.iter().skip(1) {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    effects
}
