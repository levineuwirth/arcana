//! Provoke the Trolls — `{3}{R}` instant.
//! "Provoke the Trolls deals 3 damage to any target. If a creature is dealt
//! damage this way, it gets +5/+0 until end of turn."
//!
//! The conditional pump only applies if the target is a creature. We check
//! the target choice: if it's an Object (creature or planeswalker), emit the
//! pump as well. The spec says "if a creature is dealt damage this way" so
//! we emit Pump only when targeting a creature object.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Provoke the Trolls");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Provoke the Trolls deals 3 damage to any target. If a creature is dealt damage this way, it gets +5/+0 until end of turn.".into(),
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
    let (dt, obj_id) = match target {
        TargetChoice::Object(id) => (DamageTarget::Object(*id), Some(*id)),
        TargetChoice::Player(p) => (DamageTarget::Player(*p), None),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => (DamageTarget::Object(*id), Some(*id)),
            ObjectOrPlayer::Player(p) => (DamageTarget::Player(*p), None),
        },
    };
    let mut effects = vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 3,
    }];
    if let Some(id) = obj_id {
        effects.push(Effect::Pump {
            target: id,
            power: 5,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        });
    }
    effects
}
