//! Joust Through — `{W}` instant. "Deals 3 damage to target attacking
//! or blocking creature. You gain 1 life." The combat-state
//! restriction is enforced via
//! `ObjectFilter::creature().attacking_or_blocking_only()`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Joust Through");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Joust Through deals 3 damage to target attacking or blocking creature. You gain 1 life.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().attacking_or_blocking_only(),
                    ),
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
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 3,
        },
        Effect::GainLife { player: entry.controller, amount: 1 },
    ]
}
