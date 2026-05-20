//! Hungry Flames — `{2}{R}` instant. "Hungry Flames deals 3 damage
//! to target creature and 2 damage to target player or planeswalker."

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
    let name = reg.interner_mut().intern("Hungry Flames");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Hungry Flames deals 3 damage to target creature and 2 damage to target player or planeswalker.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_player(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn dt(t: &TargetChoice) -> Option<DamageTarget> {
    match t {
        TargetChoice::Object(id) => Some(DamageTarget::Object(*id)),
        TargetChoice::Player(p) => Some(DamageTarget::Player(*p)),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => Some(DamageTarget::Object(*id)),
            ObjectOrPlayer::Player(p) => Some(DamageTarget::Player(*p)),
        },
    }
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(t) = entry.targets.targets.first().and_then(dt) {
        effects.push(Effect::DealDamage { source: entry.source, target: t, amount: 3 });
    }
    if let Some(t) = entry.targets.targets.get(1).and_then(dt) {
        effects.push(Effect::DealDamage { source: entry.source, target: t, amount: 2 });
    }
    effects
}
