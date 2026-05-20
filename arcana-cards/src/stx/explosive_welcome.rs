//! Explosive Welcome — `{7}{R}` instant. "Explosive Welcome deals 5 damage
//! to any target and 3 damage to any other target. Add {R}{R}{R}."
//!
//! GAP: no Effect for adding mana to mana pool. Damage portion delivered.

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
    let name = reg.interner_mut().intern("Explosive Welcome");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Explosive Welcome deals 5 damage to any target and 3 damage to any other target. Add {R}{R}{R}.".into(),
                target_requirements: vec![
                    TargetRequirement::any_target(),
                    TargetRequirement::any_target(),
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
        TargetChoice::ObjectOrPlayer(o) => Some(match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        }),
    }
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();
    let targets = &entry.targets.targets;
    if let Some(t) = targets.first().and_then(to_damage_target) {
        effects.push(Effect::DealDamage { source: entry.source, target: t, amount: 5 });
    }
    if let Some(t) = targets.get(1).and_then(to_damage_target) {
        effects.push(Effect::DealDamage { source: entry.source, target: t, amount: 3 });
    }
    // GAP: 'Add {R}{R}{R}' to mana pool not modeled
    effects
}
