//! Pulse of the Forge — `{1}{R}{R}` instant. "Pulse of the Forge deals
//! 4 damage to target player or planeswalker. Then if that player or
//! that planeswalker's controller has more life than you, return Pulse
//! of the Forge to its owner's hand."
//!
//! GAP: ReturnSelfToHandConditional (returning this spell to hand based
//! on a life-total comparison) is not in the engine effect catalog.
//! The damage is expressed; the conditional return is omitted.

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
    let name = reg.interner_mut().intern("Pulse of the Forge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Pulse of the Forge deals 4 damage to target player or planeswalker. Then if that player or that planeswalker's controller has more life than you, return Pulse of the Forge to its owner's hand.".into(),
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
    // GAP: ReturnSelfToHandConditional (return this card to hand if target controller has more life)
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 4,
    }]
}
