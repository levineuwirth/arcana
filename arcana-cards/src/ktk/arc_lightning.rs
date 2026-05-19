//! Arc Lightning — `{2}{R}` sorcery.
//! "Arc Lightning deals 3 damage divided as you choose among one, two, or three targets."
//!
//! GAP: Divided damage (player-chosen allocation among multiple targets) is not expressible
//! with catalog DealDamage which takes a fixed amount per target. Best effort: cannot split.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arc Lightning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Arc Lightning deals 3 damage divided as you choose among one, two, or three targets.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: arcana_core::targets::TargetFilter::AnyTarget,
                    count: TargetCount::UpTo(3),
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
    // GAP: divided damage (player-chosen split) not expressible; dealing 1 to each target as fallback
    entry.targets.targets.iter().map(|target| {
        let dt = match target {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        Effect::DealDamage { source: entry.source, target: dt, amount: 1 }
    }).collect()
}
