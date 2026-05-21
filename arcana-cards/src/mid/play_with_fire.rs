//! Play with Fire — `{R}` instant. Deals 2 damage to any target. (If a
//! player was dealt damage this way, scry 1 — conditional scry not
//! modeled.)

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
    let name = reg.interner_mut().intern("Play with Fire");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Play with Fire deals 2 damage to any target. If a player is dealt damage this way, scry 1.".into(),
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
    let (dt, is_player) = match target {
        TargetChoice::Object(id) => (DamageTarget::Object(*id), false),
        TargetChoice::Player(p) => (DamageTarget::Player(*p), true),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => (DamageTarget::Object(*id), false),
            ObjectOrPlayer::Player(p) => (DamageTarget::Player(*p), true),
        },
    };
    let mut effects: Vec<Effect> = vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 2,
    }];
    if is_player {
        effects.push(Effect::Scry {
            player: entry.controller,
            count: 1,
        });
    }
    effects
}
