//! Chandra's Fury — `{4}{R}` instant. "Chandra's Fury deals 4 damage to
//! target player or planeswalker and 1 damage to each creature that player or
//! that planeswalker's controller controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra's Fury");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Chandra's Fury deals 4 damage to target player or planeswalker and 1 damage to each creature that player or that planeswalker's controller controls.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
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
    let (dt, owner) = match target {
        TargetChoice::Player(p) => (DamageTarget::Player(*p), *p),
        TargetChoice::Object(id) => (
            DamageTarget::Object(*id),
            script::target_controller(state, *id, entry.controller),
        ),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Player(p) => (DamageTarget::Player(*p), *p),
            ObjectOrPlayer::Object(id) => (
                DamageTarget::Object(*id),
                script::target_controller(state, *id, entry.controller),
            ),
        },
    };
    let _ = owner;
    // We need creatures controlled by `owner`. ControllerConstraint is You/Opponent
    // relative to entry.controller; if owner==entry.controller use You else Opponent.
    let constraint = if owner == entry.controller {
        ControllerConstraint::You
    } else {
        ControllerConstraint::Opponent
    };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(constraint),
        entry.controller,
    );
    vec![
        Effect::DealDamage { source: entry.source, target: dt, amount: 4 },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DealDamage {
                source: entry.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 1,
            }),
        },
    ]
}
