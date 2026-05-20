//! Venom Blast — `{2}{G}{G}` sorcery. "Put two +1/+1 counters on
//! target creature you control. It deals damage equal to its power to
//! up to one other target creature."
//!
//! Two targets: the creature getting counters, and (up to one) other
//! creature it damages for its (post-counter) power.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Venom Blast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put two +1/+1 counters on target creature you control. It deals damage equal to its power to up to one other target creature.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(src)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let mut out = vec![Effect::AddCounters {
        target: *src,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }];
    if let Some(TargetChoice::Object(victim)) = entry.targets.targets.get(1) {
        let power = (script::power_of(state, *src) + 2).max(0) as u32;
        out.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*victim),
            amount: power,
        });
    }
    out
}
