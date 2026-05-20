//! Blightning — `{1}{B}{R}` sorcery. "Blightning deals 3 damage to
//! target player or planeswalker. That player or that planeswalker's
//! controller discards two cards."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blightning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Blightning deals 3 damage to target player or planeswalker. That player or that planeswalker's controller discards two cards.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let p = match target {
        TargetChoice::Player(p) => *p,
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => *p,
        _ => return Vec::new(),
    };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(p),
            amount: 3,
        },
        Effect::Discard {
            player: p,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
