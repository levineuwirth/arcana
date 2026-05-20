//! Throw from the Saddle — `{1}{G}` sorcery. "Target creature you
//! control gets +1/+1 until end of turn. Put a +1/+1 counter on it
//! instead if it's a Mount. Then it deals damage equal to its power
//! to target creature you don't control." The Mount-counter
//! alternative is gapped; the +1/+1 and the power-equal damage are
//! emitted.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Throw from the Saddle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gets +1/+1 until end of turn. Put a +1/+1 counter on it instead if it's a Mount. Then it deals damage equal to its power to target creature you don't control.".into(),
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
    let mut it = entry.targets.targets.iter();
    let Some(TargetChoice::Object(own)) = it.next() else { return Vec::new(); };
    let Some(TargetChoice::Object(foe)) = it.next() else { return Vec::new(); };
    // GAP: "Put a +1/+1 counter on it instead if it's a Mount" — the
    // Mount-conditional alternative is not modeled; the EOT pump is
    // applied unconditionally.
    let pwr = script::power_of(state, *own).max(0) as u32;
    vec![
        Effect::Pump {
            target: *own,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DealDamage {
            source: *own,
            target: DamageTarget::Object(*foe),
            amount: pwr,
        },
    ]
}
