//! Quenchable Fire — `{3}{R}` sorcery.
//! "Quenchable Fire deals 3 damage to target player or planeswalker. It deals an
//! additional 3 damage to that player or planeswalker at the beginning of your next
//! upkeep step unless that player or that planeswalker's controller pays {U} before
//! that step."
//!
//! GAP: delayed trigger ("at beginning of your next upkeep unless pays {U}") is not
//! expressible with the single-spell-ability model. The initial 3 damage is expressible.

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
    let name = reg.interner_mut().intern("Quenchable Fire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Quenchable Fire deals 3 damage to target player or planeswalker. It deals an additional 3 damage to that player or planeswalker at the beginning of your next upkeep step unless that player or that planeswalker's controller pays {U} before that step.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
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
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![
        Effect::DealDamage { source: entry.source, target: dt, amount: 3 },
        // GAP: delayed "unless pays {U}" trigger for additional 3 damage at next upkeep
        // — no deferred conditional damage variant in Effect catalog.
    ]
}
