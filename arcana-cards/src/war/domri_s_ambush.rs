//! Domri's Ambush — `{R}{G}` sorcery. "Put a +1/+1 counter on target
//! creature you control. Then that creature deals damage equal to its
//! power to target creature or planeswalker an opponent controls."
//! Uses `script::power_of` for the dynamic damage amount.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domri's Ambush");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control. Then that creature deals damage equal to its power to target creature or planeswalker an opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::creature()),
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
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(t1) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(my_creature) = t0 else { return Vec::new(); };
    let TargetChoice::Object(their_target) = t1 else { return Vec::new(); };
    let power = script::power_of(state, *my_creature).max(0) as u32;
    vec![
        Effect::AddCounters { target: *my_creature, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*their_target),
            amount: power,
        },
    ]
}
