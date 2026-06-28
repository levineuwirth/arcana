//! Lava Storm — `{3}{R}{R}` instant. "Lava Storm deals 2 damage to each attacking creature or Lava Storm deals 2 damage to each blocking creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardRegistry, ModeClause, ModalSpec, SpellAbilityDef, dispatch_modal_effect,
};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lava Storm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Lava Storm deals 2 damage to each attacking creature or Lava Storm deals 2 damage to each blocking creature.".into(),
                target_requirements: vec![],
                modal: Some(ModalSpec {
                    min_modes: 1,
                    max_modes: 1,
                    clauses: vec![
                        ModeClause {
                            text: "Lava Storm deals 2 damage to each attacking creature.".into(),
                            target_requirements: vec![],
                        },
                        ModeClause {
                            text: "Lava Storm deals 2 damage to each blocking creature.".into(),
                            target_requirements: vec![],
                        },
                    ],
                }),
                effect: dispatch_modal_effect,
            })
            .with_mode_effects(vec![mode_attacking, mode_blocking]),
    )
}

fn mode_attacking(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature().attacking_only(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(0),
            amount: 2,
        }),
    }]
}

fn mode_blocking(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature().blocking_only(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(0),
            amount: 2,
        }),
    }]
}
