//! Cone of Flame — `{3}{R}{R}` sorcery. "Cone of Flame deals 1 damage to any
//! target, 2 damage to another target, and 3 damage to a third target."

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
    let name = reg.interner_mut().intern("Cone of Flame");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Cone of Flame deals 1 damage to any target, 2 damage to another target, and 3 damage to a third target.".into(),
                target_requirements: vec![
                    TargetRequirement::any_target(),
                    TargetRequirement::any_target(),
                    TargetRequirement::any_target(),
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn damage_target_from_choice(choice: &TargetChoice) -> Option<DamageTarget> {
    match choice {
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
    let amounts = [1u32, 2, 3];
    entry
        .targets
        .targets
        .iter()
        .zip(amounts.iter())
        .filter_map(|(choice, &amount)| {
            damage_target_from_choice(choice).map(|dt| Effect::DealDamage {
                source: entry.source,
                target: dt,
                amount,
            })
        })
        .collect()
}
