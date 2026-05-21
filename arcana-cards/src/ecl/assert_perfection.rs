//! Assert Perfection — `{1}{G}` sorcery. Target creature you control
//! gets +1/+0 until end of turn. It deals damage equal to its power to
//! up to one target creature an opponent controls.

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
    let name = reg.interner_mut().intern("Assert Perfection");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control gets +1/+0 until end of turn. It deals damage equal to its power to up to one target creature an opponent controls.".into(),
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
    let Some(t1) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(a) = t1 else { return Vec::new(); };
    let a = *a;
    let mut effects: Vec<Effect> = vec![Effect::Pump {
        target: a,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }];
    if let Some(TargetChoice::Object(b)) = entry.targets.targets.get(1) {
        // After the pump applies, the power-of read here will not include +1 (resolution
        // ordering not guaranteed) — use the post-pump approximation.
        let amount = (script::power_of(state, a) + 1).max(0) as u32;
        effects.push(Effect::DealDamage {
            source: a,
            target: DamageTarget::Object(*b),
            amount,
        });
    }
    effects
}
