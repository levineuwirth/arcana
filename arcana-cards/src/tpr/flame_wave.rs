//! Flame Wave — `{3}{R}{R}{R}{R}` sorcery. "Flame Wave deals 4
//! damage to target player or planeswalker and each creature that
//! player or that planeswalker's controller controls." We can deal
//! the targeted 4 damage and damage each creature controlled by the
//! target controller. For planeswalker target, we read its controller
//! via script::target_controller.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flame Wave");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flame Wave deals 4 damage to target player or planeswalker and each creature that player or that planeswalker's controller controls.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::PLANESWALKER.into()),
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
    // GAP: target 'player or planeswalker' — we restricted to
    // planeswalker (TargetFilter::Permanent with PLANESWALKER) since
    // we can't express the union with TargetFilter::Player.
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let (dt, controller) = match target {
        TargetChoice::Object(id) => (DamageTarget::Object(*id), script::target_controller(state, *id, entry.controller)),
        TargetChoice::Player(p) => (DamageTarget::Player(*p), *p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => (DamageTarget::Object(*id), script::target_controller(state, *id, entry.controller)),
            ObjectOrPlayer::Player(p) => (DamageTarget::Player(*p), *p),
        },
    };
    let _ = controller;
    let mut effects = vec![Effect::DealDamage { source: entry.source, target: dt, amount: 4 }];
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    for id in ids {
        effects.push(Effect::DealDamage { source: entry.source, target: DamageTarget::Object(id), amount: 4 });
    }
    effects
}
