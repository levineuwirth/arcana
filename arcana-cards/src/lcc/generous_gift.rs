//! Generous Gift — `{2}{W}` instant. "Destroy target permanent. Its controller creates a 3/3
//! green Elephant creature token."
//!
//! # GAP
//! - "Its controller" (the target permanent's controller, not the spell caster) cannot be
//!   used as the CreateToken controller without a state lookup
//! - Destroy + token-for-opponent combo is only partially expressible

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Generous Gift");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target permanent. Its controller creates a 3/3 green Elephant creature token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::Permanent(
                        arcana_core::targets::ObjectFilter::new(),
                    ),
                    count: arcana_core::targets::TargetCount::Exactly(1),
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
    // GAP: token for target's controller (not entry.controller); cannot address opponent's
    // controller via CreateToken without state lookup
    vec![Effect::DestroyPermanent { target: *id }]
}
