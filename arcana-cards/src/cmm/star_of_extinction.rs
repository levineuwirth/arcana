//! Star of Extinction — `{5}{R}{R}` sorcery, "Destroy target land. Star of
//! Extinction deals 20 damage to each creature and each planeswalker."
//!
//! The planeswalker filter is not a supported ObjectFilter type; damage
//! is applied only to creatures. GAP: no planeswalker type filter.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Star of Extinction");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target land. Star of Extinction deals 20 damage to each creature and each planeswalker.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into())
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(land_id) = target else { return Vec::new(); };
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let mut effects = vec![Effect::DestroyPermanent { target: *land_id }];
    effects.push(Effect::ForEach {
        targets: creature_ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(arcana_core::objects::NULL_OBJECT_ID),
            amount: 20,
        }),
    });
    effects
}
