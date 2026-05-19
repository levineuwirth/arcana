//! Knockout Maneuver — `{2}{G}` sorcery. "Put a +1/+1 counter on target
//! creature you control, then it deals damage equal to its power to target
//! creature an opponent controls."
//!
//! GAP: "deals damage equal to its power" (after the counter is added) requires
//! a runtime power-of-object lookup at resolve time, which is not expressible
//! with the catalog's Effect::DealDamage (fixed amount).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knockout Maneuver");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control, then it deals damage equal to its power to target creature an opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
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
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id0) = t0 else { return Vec::new(); };
    vec![
        Effect::AddCounters { target: *id0, kind: CounterKind::PlusOnePlusOne, count: 1 },
        // GAP: deals damage equal to its power (after counter placed) to second
        // target — runtime power lookup not expressible
    ]
}
