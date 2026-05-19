//! Tundra Fumarole — `{1}{R}{R}` Snow Sorcery. "Tundra Fumarole deals 4
//! damage to target creature or planeswalker. Add {C} for each {S} spent
//! to cast this spell. Until end of turn, you don't lose this mana as
//! steps and phases end."
//!
//! GAP: snow mana tracking ({S} count) and mana addition with persistent-
//! until-end-of-turn rider not in Effect catalog. Emitting the 4 damage.

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
    let name = reg.interner_mut().intern("Tundra Fumarole");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tundra Fumarole deals 4 damage to target creature or planeswalker. Add {C} for each {S} spent to cast this spell. Until end of turn, you don't lose this mana as steps and phases end.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine::CREATURE.into())
                            .with_types_any(TypeLine::PLANESWALKER.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: snow mana {S} count and mana addition with persistent rider not
    // in catalog.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
