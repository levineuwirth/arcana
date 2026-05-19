//! Blightning — `{1}{B}{R}` sorcery. "Blightning deals 3 damage to
//! target player or planeswalker. That player or that planeswalker's
//! controller discards two cards."
//!
//! GAP: TargetPlayerOrPlaneswalkerController (determining the player to
//! discard from when the target is a planeswalker) requires reading the
//! controller of a targeted permanent — not shown in catalog. Damage
//! is expressed to any target; discard applied to entry.controller as
//! approximation (incorrect for planeswalker targets).

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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Blightning deals 3 damage to target player or planeswalker. That player or that planeswalker's controller discards two cards.".into(),
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
    let (dt, discard_player) = match target {
        TargetChoice::Object(id) => (DamageTarget::Object(*id), entry.controller),
        TargetChoice::Player(p) => (DamageTarget::Player(*p), *p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => (DamageTarget::Object(*id), entry.controller),
            ObjectOrPlayer::Player(p) => (DamageTarget::Player(*p), *p),
        },
    };
    // GAP: TargetPlaneswalkerController (when target is planeswalker, discard_player should be
    //      its controller, not entry.controller)
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 3,
        },
        Effect::Discard {
            player: discard_player,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
