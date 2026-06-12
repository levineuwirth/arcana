//! Hazardous Blast — `{3}{R}` sorcery. Deals 1 damage to each creature
//! your opponents control. Creatures your opponents control can't block
//! this turn (filtered can't-block static until end of turn).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hazardous Blast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Hazardous Blast deals 1 damage to each creature your opponents control. Creatures your opponents control can't block this turn.".into(),
            target_requirements: vec![],
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
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    let mut effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 1,
        })
        .collect();
    // "Creatures your opponents control can't block this turn."
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_cant_block(
            entry.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            Duration::EndOfTurn,
        ),
    });
    effects
}
