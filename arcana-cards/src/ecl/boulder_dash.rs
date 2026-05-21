//! Boulder Dash — `{1}{R}` sorcery. "Boulder Dash deals 2 damage to
//! any target and 1 damage to any other target."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boulder Dash");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Boulder Dash deals 2 damage to any target and 1 damage to any other target.".into(),
                target_requirements: vec![
                    TargetRequirement::any_target(),
                    TargetRequirement::any_target(),
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn target_to_damage(t: &TargetChoice) -> Option<DamageTarget> {
    Some(match t {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    })
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(t) = entry.targets.targets.first().and_then(target_to_damage) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: t,
            amount: 2,
        });
    }
    if let Some(t) = entry.targets.targets.get(1).and_then(target_to_damage) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: t,
            amount: 1,
        });
    }
    effects
}
