//! Crackling Doom — `{R}{W}{B}` instant. "Crackling Doom deals 2
//! damage to each opponent. Each opponent sacrifices a creature with
//! the greatest power among creatures that player controls."
//!
//! GAP: "creature with the greatest power among creatures that
//! player controls" is not a representable per-player filter. We emit
//! the each-opponent damage and a generic each-opponent sacrifice.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crackling Doom");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Crackling Doom deals 2 damage to each opponent. Each opponent sacrifices a creature with the greatest power among creatures that player controls.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let opps = script::opponents(state, entry.controller);
    let mut effects: Vec<Effect> = opps
        .iter()
        .map(|p| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(*p),
            amount: 2,
        })
        .collect();
    // GAP: "greatest power" restriction on the sacrifice — emit a
    // generic per-opponent creature sacrifice.
    for p in opps {
        effects.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        });
    }
    effects
}
