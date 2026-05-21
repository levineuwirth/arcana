//! Devouring Tendrils — `{1}{G}` sorcery. "Target creature you control deals
//! damage equal to its power to target creature or planeswalker you don't
//! control. When the permanent you don't control dies this turn, you gain 2
//! life."

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
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devouring Tendrils");
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
                text: "Target creature you control deals damage equal to its power to target creature or planeswalker you don't control. When the permanent you don't control dies this turn, you gain 2 life.".into(),
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
                            ObjectFilter::permanent()
                                .with_types_any(TypeLine(
                                    TypeLine::CREATURE | TypeLine::PLANESWALKER,
                                ))
                                .controlled_by(ControllerConstraint::Opponent),
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
    let (Some(t0), Some(t1)) = (targets.first(), targets.get(1)) else { return Vec::new(); };
    let (TargetChoice::Object(a), TargetChoice::Object(b)) = (t0, t1) else { return Vec::new(); };
    let dmg = script::power_of(state, *a).max(0) as u32;
    // GAP: 'When the permanent you don't control dies this turn, you gain 2 life'
    // delayed-trigger-on-target's-death rider not in DelayedAction list.
    vec![Effect::DealDamage {
        source: *a,
        target: DamageTarget::Object(*b),
        amount: dmg,
    }]
}
