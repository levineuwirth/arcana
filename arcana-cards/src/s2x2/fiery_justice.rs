//! Fiery Justice — `{R}{G}{W}` sorcery. "Fiery Justice deals 5 damage
//! divided as you choose among any number of targets. Target opponent
//! gains 5 life."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fiery Justice");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Fiery Justice deals 5 damage divided as you choose among any number of targets. Target opponent gains 5 life.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Any,
                    controller: None,
                },
                TargetRequirement::target_player(),
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
    // "divided as you choose" has no primitive; deal 1 to each damage
    // target. The final target is the opponent who gains 5 life.
    let mut effects = Vec::new();
    let targets = &entry.targets.targets;
    if let Some((last, dmg)) = targets.split_last() {
        for t in dmg {
            let dt = match t {
                TargetChoice::Object(id) => DamageTarget::Object(*id),
                TargetChoice::Player(p) => DamageTarget::Player(*p),
                TargetChoice::ObjectOrPlayer(o) => match o {
                    ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                    ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
                },
            };
            effects.push(Effect::DealDamage {
                source: entry.source,
                target: dt,
                amount: 1,
            });
        }
        if let TargetChoice::Player(p) = last {
            effects.push(Effect::GainLife { player: *p, amount: 5 });
        }
    }
    effects
}
