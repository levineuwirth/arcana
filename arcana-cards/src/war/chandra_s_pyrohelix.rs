//! Chandra's Pyrohelix — `{1}{R}` instant. "Chandra's Pyrohelix deals
//! 2 damage divided as you choose among one or two targets."
//!
//! GAP: "divided as you choose" damage has no Effect. We take up to
//! two any-targets and deal a flat 1 damage to each — matches the 1+1
//! division but not other distributions.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra's Pyrohelix");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Chandra's Pyrohelix deals 2 damage divided as you choose among one or two targets.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::AnyTarget,
                count: TargetCount::UpTo(2),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: divided damage — proxy with 1 per chosen target.
    entry
        .targets
        .targets
        .iter()
        .map(|t| {
            let dt = match t {
                TargetChoice::Object(id) => DamageTarget::Object(*id),
                TargetChoice::Player(p) => DamageTarget::Player(*p),
                TargetChoice::ObjectOrPlayer(o) => match o {
                    ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                    ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
                },
            };
            Effect::DealDamage {
                source: entry.source,
                target: dt,
                amount: 1,
            }
        })
        .collect()
}
