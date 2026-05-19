//! Huatli's Final Strike — `{2}{G}` instant, "Target creature you control gets
//! +1/+0 until end of turn. It deals damage equal to its power to target
//! creature an opponent controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huatli's Final Strike");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 until end of turn. It deals damage equal to its power to target creature an opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You)
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)
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
    let mut targets = entry.targets.targets.iter();
    let Some(t0) = targets.next() else { return Vec::new(); };
    let Some(t1) = targets.next() else { return Vec::new(); };
    let TargetChoice::Object(src_id) = t0 else { return Vec::new(); };
    let TargetChoice::Object(dst_id) = t1 else { return Vec::new(); };
    // Pump happens before damage check, so add +1 to power reading
    let power_after_pump = script::power_of(state, *src_id) + 1;
    vec![
        Effect::Pump {
            target: *src_id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*dst_id),
            amount: power_after_pump.max(0) as u32,
        },
    ]
}
