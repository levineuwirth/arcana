//! Signature Slam — `{2}{G}` instant. "Put a +1/+1 counter on target
//! creature you control, then each modified creature you control
//! deals damage equal to its power to target creature you don't
//! control." 'Modified' (with auras/equipment/counters) is not an
//! ObjectFilter primitive — we approximate with all-your-creatures
//! and deal each one's power as damage.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, CounterKind, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Signature Slam");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    // GAP: 'modified' (aura/equipment/counters) filter not in ObjectFilter — approximated as 'creatures you control'.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a +1/+1 counter on target creature you control, then each modified creature you control deals damage equal to its power to target creature you don't control.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
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
    let ts = &entry.targets.targets;
    if ts.len() < 2 { return Vec::new(); }
    let (TargetChoice::Object(own), TargetChoice::Object(victim)) = (&ts[0], &ts[1]) else {
        return Vec::new();
    };
    let mut effects = vec![Effect::AddCounters {
        target: *own,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    let mine = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    for id in mine {
        let p = script::power_of(state, id).max(0) as u32;
        effects.push(Effect::DealDamage {
            source: id,
            target: DamageTarget::Object(*victim),
            amount: p,
        });
    }
    effects
}
