//! Arc Trail — `{1}{R}` sorcery. "Arc Trail deals 2 damage to any
//! target and 1 damage to any other target."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arc Trail");
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
                text: "Arc Trail deals 2 damage to any target and 1 damage to any other target.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::AnyTarget,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: arcana_core::targets::TargetFilter::AnyTarget,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn to_damage_target(t: &TargetChoice) -> Option<DamageTarget> {
    match t {
        TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
        TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => Some(DamageTarget::Object(*id)),
            ObjectOrPlayer::Player(p) => Some(DamageTarget::Player(*p)),
        },
    }
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    let mut targets = entry.targets.targets.iter();
    if let Some(t) = targets.next() {
        if let Some(dt) = to_damage_target(t) {
            effects.push(Effect::DealDamage { source: entry.source, target: dt, amount: 2 });
        }
    }
    if let Some(t) = targets.next() {
        if let Some(dt) = to_damage_target(t) {
            effects.push(Effect::DealDamage { source: entry.source, target: dt, amount: 1 });
        }
    }
    effects
}
