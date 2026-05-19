//! Into the Maw of Hell — `{4}{R}{R}` sorcery, "Destroy target land. Into the
//! Maw of Hell deals 13 damage to target creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Into the Maw of Hell");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target land. Into the Maw of Hell deals 13 damage to target creature.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::new().with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_creature(),
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
    let mut effects = Vec::new();
    if let Some(t0) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = t0 {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    if let Some(t1) = entry.targets.targets.get(1) {
        if let TargetChoice::Object(id) = t1 {
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(*id),
                amount: 13,
            });
        }
    }
    effects
}
