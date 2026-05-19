//! Teferi's Time Twist — `{1}{U}` instant. "Exile target permanent you control. Return that card
//! to the battlefield under its owner's control at the beginning of the next end step. If it
//! enters as a creature, it enters with an additional +1/+1 counter on it."
//!
//! # GAP
//! - No support for delayed return at the beginning of the next end step
//! - No support for conditional "enters with +1/+1 counter if it enters as a creature"
//!   (enters-as replacement effect)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teferi's Time Twist");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target permanent you control. Return that card to the battlefield under its owner's control at the beginning of the next end step. If it enters as a creature, it enters with an additional +1/+1 counter on it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::new()),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: delayed return at next end step; conditional +1/+1 counter on creature entry
    vec![Effect::ExilePermanent { target: *id }]
}
