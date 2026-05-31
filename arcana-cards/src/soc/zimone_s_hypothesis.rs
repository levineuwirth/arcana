//! Zimone's Hypothesis — `{3}{U}{U}` instant.
//! "You may put a +1/+1 counter on a creature. Then choose odd or even.
//! Return each creature with power of the chosen quality to its owner's
//! hand. (Zero is even.)"
//!
//! The first clause (an optional +1/+1 counter on a target creature) is
//! expressible and emitted. The second clause — "choose odd or even,
//! then return each creature with power of the chosen parity to its
//! owner's hand" — is not: there is no "choose odd/even" primitive and
//! no power-parity ObjectFilter refinement, so the parity-gated mass
//! bounce cannot be expressed. GAP-ing that clause rather than emitting
//! a wrong fixed-parity board sweep.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zimone's Hypothesis");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You may put a +1/+1 counter on a creature. Then choose odd or even. Return each creature with power of the chosen quality to its owner's hand. (Zero is even.)".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    // GAP: no "choose odd or even" primitive and no power-parity ObjectFilter
    // refinement, so "return each creature with power of the chosen quality
    // to its owner's hand" cannot be expressed.
    effects
}
