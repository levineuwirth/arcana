//! Rabid Bite — `{1}{G}` sorcery, "Target creature you control deals damage
//! equal to its power to target creature you don't control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rabid Bite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control deals damage equal to its power to target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::Creature,
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
    let (Some(t0), Some(t1)) = (targets.get(0), targets.get(1)) else { return Vec::new(); };
    let TargetChoice::Object(attacker_id) = t0 else { return Vec::new(); };
    let TargetChoice::Object(defender_id) = t1 else { return Vec::new(); };
    let power = script::power_of(state, *attacker_id).max(0) as u32;
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*defender_id),
        amount: power,
    }]
}
