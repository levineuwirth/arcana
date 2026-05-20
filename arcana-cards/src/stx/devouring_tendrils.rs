//! Devouring Tendrils — `{1}{G}` sorcery. "Target creature you control deals
//! damage equal to its power to target creature or planeswalker you don't
//! control. When the permanent you don't control dies this turn, you gain
//! 2 life."
//! GAP: DelayedAction supports only Sacrifice/Exile/ReturnToHand; "when it
//! dies gain 2 life" cannot be expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
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
    let Some(attacker_t) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(attacker_id) = attacker_t else { return Vec::new(); };
    let Some(defender_t) = entry.targets.targets.get(1) else { return Vec::new(); };
    let TargetChoice::Object(defender_id) = defender_t else { return Vec::new(); };

    let pwr = script::power_of(state, *attacker_id);
    let amount = pwr.max(0) as u32;

    // GAP: "when the permanent you don't control dies this turn, you gain 2 life" —
    // DelayedAction does not support GainLife as an action
    vec![Effect::DealDamage {
        source: entry.source,
        target: arcana_core::events::DamageTarget::Object(*defender_id),
        amount,
    }]
}
