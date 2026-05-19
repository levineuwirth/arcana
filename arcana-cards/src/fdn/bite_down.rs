//! Bite Down — `{1}{G}` instant. "Target creature you control deals damage
//! equal to its power to target creature or planeswalker you don't control."
//!
//! Two-target spell: first target is the attacker (creature you control),
//! second is the defender (creature or planeswalker opponent controls).
//! Damage equals the attacker's power at resolution.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bite Down");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals damage equal to its power to target creature or planeswalker you don't control.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ENCHANTMENT)),
                    ),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = &entry.targets.targets;
    if targets.len() < 2 { return Vec::new(); }
    let TargetChoice::Object(attacker_id) = &targets[0] else { return Vec::new(); };
    let TargetChoice::Object(defender_id) = &targets[1] else { return Vec::new(); };
    let power = script::power_of(state, *attacker_id).max(0) as u32;
    if power == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*defender_id),
        amount: power,
    }]
}
