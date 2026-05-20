//! Inspired Ultimatum — `{U}{U}{R}{R}{R}{W}{W}` sorcery, "Target player
//! gains 5 life, Inspired Ultimatum deals 5 damage to any target, then
//! you draw five cards."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetRequirement,
};
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
    let mut iter = entry.targets.targets.iter();
    let Some(TargetChoice::Player(p)) = iter.next() else { return Vec::new(); };
    let mut effects = vec![Effect::GainLife { player: *p, amount: 5 }];
    if let Some(t) = iter.next() {
        let dt = match t {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(pl) => DamageTarget::Player(*pl),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(pl) => DamageTarget::Player(*pl),
            },
        };
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 5,
        });
    }
    effects.push(Effect::DrawCards { player: entry.controller, count: 5 });
    effects
}
