//! Explosive Welcome — `{7}{R}` instant. "Explosive Welcome deals 5
//! damage to any target and 3 damage to any other target. Add
//! {R}{R}{R}."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

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

fn dt(t: &TargetChoice) -> Option<DamageTarget> {
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
    let mut out = Vec::new();
    if let Some(t) = entry.targets.targets.first().and_then(dt) {
        out.push(Effect::DealDamage { source: entry.source, target: t, amount: 5 });
    }
    if let Some(t) = entry.targets.targets.get(1).and_then(dt) {
        out.push(Effect::DealDamage { source: entry.source, target: t, amount: 3 });
    }
    out.push(Effect::AddMana {
        player: entry.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, entry.source); 3],
    });
    out
}
