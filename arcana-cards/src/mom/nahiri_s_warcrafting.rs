//! Nahiri's Warcrafting — `{1}{R}{R}` sorcery. "Nahiri's Warcrafting
//! deals 5 damage to target creature, planeswalker, or battle. Look at
//! the top X cards of your library, where X is the excess damage dealt
//! this way. You may exile one of those cards. Put the rest on the
//! bottom of your library in a random order. You may play the exiled
//! card this turn."
//!
//! The damage is expressible. The impulse-dig rider depends on X =
//! "excess damage dealt this way" (lethal-overflow excess), which no
//! script:: helper exposes, and the "you may play the exiled card this
//! turn" permission is also not expressible. Emitting the 5 damage and
//! GAP-ing the dynamic dig.

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
    let name = reg.interner_mut().intern("Nahiri's Warcrafting");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Nahiri's Warcrafting deals 5 damage to target creature, planeswalker, or battle. Look at the top X cards of your library, where X is the excess damage dealt this way. You may exile one of those cards. Put the rest on the bottom of your library in a random order. You may play the exiled card this turn.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: the "look at the top X cards where X is the excess damage dealt"
    // impulse-dig (excess-damage amount + play-the-exiled-card permission)
    // is not expressible. Also: target restriction is creature-only here;
    // planeswalker/battle target widening not expressible via the helpers.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 5,
    }]
}
