//! Hail Storm — `{1}{G}{G}` instant. "Hail Storm deals 2 damage to
//! each attacking creature and 1 damage to you and each creature you
//! control."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hail Storm");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Hail Storm deals 2 damage to each attacking creature and 1 damage to you and each creature you control.".into(),
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
    let mut effects = Vec::new();
    // 2 damage to each attacking creature.
    let attackers = script::ids_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        entry.controller,
    );
    for id in attackers {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 2,
        });
    }
    // 1 damage to you and each creature you control.
    effects.push(Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Player(entry.controller),
        amount: 1,
    });
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    for id in ids {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 1,
        });
    }
    effects
}
