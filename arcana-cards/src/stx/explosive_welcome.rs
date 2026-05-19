//! Explosive Welcome — `{7}{R}` instant. "Explosive Welcome deals 5
//! damage to any target and 3 damage to any other target. Add {R}{R}{R}."
//!
//! GAP: Add mana ({R}{R}{R}) effect not in catalog.
//! Emitting DealDamage 5 to first target and DealDamage 3 to second target.

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
    let name = reg.interner_mut().intern("Explosive Welcome");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Explosive Welcome deals 5 damage to any target and 3 damage to any other target. Add {R}{R}{R}.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::AnyTarget,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::AnyTarget,
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
    let mut effects = Vec::new();
    let to_damage_target = |tc: &TargetChoice| -> DamageTarget {
        match tc {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        }
    };
    if let Some(t1) = entry.targets.targets.first() {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: to_damage_target(t1),
            amount: 5,
        });
    }
    if let Some(t2) = entry.targets.targets.get(1) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: to_damage_target(t2),
            amount: 3,
        });
    }
    // GAP: Add mana {R}{R}{R} (mana addition effect) not in catalog.
    effects
}
