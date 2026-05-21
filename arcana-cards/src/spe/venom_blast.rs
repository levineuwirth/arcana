//! Venom Blast — `{2}{G}{G}` sorcery. "Put two +1/+1 counters on target
//! creature you control. It deals damage equal to its power to up to one
//! other target creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put two +1/+1 counters on target creature you control. It deals damage equal to its power to up to one other target creature.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You)
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(own) = t0 else { return Vec::new(); };
    effects.push(Effect::AddCounters { target: *own, kind: CounterKind::PlusOnePlusOne, count: 2 });
    if let Some(t1) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(other) = t1 {
            let pw = script::power_of(state, *own).max(0) as u32;
            effects.push(Effect::DealDamage {
                source: *own,
                target: DamageTarget::Object(*other),
                amount: pw,
            });
        }
    }
    effects
}
