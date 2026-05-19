//! Inspired Ultimatum — `{U}{U}{R}{R}{R}{W}{W}` sorcery.
//! "Target player gains 5 life, Inspired Ultimatum deals 5 damage to any
//! target, then you draw five cards."

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
    let name = reg.interner_mut().intern("Inspired Ultimatum");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{R}{R}{R}{W}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player gains 5 life, Inspired Ultimatum deals 5 damage to any target, then you draw five cards.".into(),
                target_requirements: vec![
                    TargetRequirement::target_player(),
                    TargetRequirement::any_target(),
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(life_player) = first else { return Vec::new(); };

    let Some(second) = entry.targets.targets.get(1) else { return Vec::new(); };
    let dt = match second {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };

    vec![
        Effect::GainLife { player: *life_player, amount: 5 },
        Effect::DealDamage { source: entry.source, target: dt, amount: 5 },
        Effect::DrawCards { player: entry.controller, count: 5 },
    ]
}
