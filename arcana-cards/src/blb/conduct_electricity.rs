//! Conduct Electricity — `{4}{R}` instant. "Conduct Electricity deals 6
//! damage to target creature and 2 damage to up to one target creature
//! token."
//!
//! The second target is specifically a creature token; no ObjectFilter
//! builder restricts to tokens. Best-effort: target any creature for the
//! second target.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conduct Electricity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Conduct Electricity deals 6 damage to target creature and 2 damage to up \
                       to one target creature token."
                    .into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        // GAP: token restriction not supported; targeting any creature
                        filter: TargetFilter::Creature,
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
    let mut effects = Vec::new();
    if let Some(t) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*id),
                amount: 6,
            });
        }
    }
    if let Some(t) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*id),
                amount: 2,
            });
        }
    }
    effects
}
