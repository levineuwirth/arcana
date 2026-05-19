//! Venom Blast — `{2}{G}{G}` sorcery. "Put two +1/+1 counters on target
//! creature you control. It deals damage equal to its power to up to one
//! other target creature."
//!
//! GAP: "deals damage equal to its power" after counters are added requires
//! reading the first target's power at resolution time, which is not
//! expressible with the current Effect catalog (no Effect::DealDamageEqualToPower).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement, ObjectFilter};
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
                    TargetRequirement::target_creature(),
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
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "deals damage equal to its power" — cannot read runtime power value
    vec![
        Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 2 },
    ]
}
