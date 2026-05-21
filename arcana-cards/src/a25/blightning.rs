//! Blightning — `{1}{B}{R}` sorcery. "Blightning deals 3 damage to
//! target player or planeswalker. That player or that planeswalker's
//! controller discards two cards." Player-or-planeswalker target uses
//! `any_target` since TargetFilter has no union of Player and
//! Permanent(planeswalker).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
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
                // GAP: target restricted to player-or-planeswalker not
                // expressible in TargetFilter; using any_target.
                text: "Blightning deals 3 damage to target player or planeswalker. That player or that planeswalker's controller discards two cards.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let (dt, discarder) = match target {
        TargetChoice::Object(id) => (
            DamageTarget::Object(*id),
            script::target_controller(state, *id, entry.controller),
        ),
        TargetChoice::Player(p) => (DamageTarget::Player(*p), *p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => (
                DamageTarget::Object(*id),
                script::target_controller(state, *id, entry.controller),
            ),
            ObjectOrPlayer::Player(p) => (DamageTarget::Player(*p), *p),
        },
    };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 3,
        },
        Effect::Discard {
            player: discarder,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
