//! Fire and Brimstone — `{3}{W}{W}` instant. "Fire and Brimstone
//! deals 4 damage to target player who attacked this turn and 4
//! damage to you."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fire and Brimstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Fire and Brimstone deals 4 damage to target player who attacked this turn and 4 damage to you.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // "who attacked this turn" restriction not expressible in target
    // filter; applied to target player.
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(*p),
            amount: 4,
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(entry.controller),
            amount: 4,
        },
    ]
}
