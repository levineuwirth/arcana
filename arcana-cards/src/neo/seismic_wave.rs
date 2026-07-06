//! Seismic Wave — `{2}{R}` instant. "Deals 2 damage to any target and
//! 1 damage to each nonartifact creature target opponent controls."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seismic Wave");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Seismic Wave deals 2 damage to any target and 1 damage to each nonartifact creature target opponent controls.".into(),
                target_requirements: vec![
                    TargetRequirement::any_target(),
                    TargetRequirement::target_opponent(),
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
    let mut effects = Vec::new();
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match first {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    effects.push(Effect::DealDamage { source: entry.source, target: dt, amount: 2 });
    let Some(second) = entry.targets.targets.get(1) else { return effects; };
    let TargetChoice::Player(opp) = second else { return effects; };
    let opp = *opp;
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .without_types(TypeLine::ARTIFACT.into()),
        opp,
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
