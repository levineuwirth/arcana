//! Nahiri's Warcrafting — `{1}{R}{R}` sorcery.
//! "Nahiri's Warcrafting deals 5 damage to target creature, planeswalker, or battle. Look at the
//! top X cards of your library, where X is the excess damage dealt this way. You may exile one of
//! those cards. Put the rest on the bottom of your library in a random order. You may play the
//! exiled card this turn."
//!
//! # GAP: ExcessDamageLookTop — no Effect variant for computing excess damage dealt and using
//! that as X for looking at the top X cards, nor for "you may play the exiled card this turn".
//! Best effort: deal 5 damage to target creature; the look-top-X and play-from-exile clause are
//! dropped.

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
    // GAP: ExcessDamageLookTop — no Effect variant for excess-damage-based top-X look or
    // play-from-exile-this-turn permission.
    vec![Effect::DealDamage {
        source: entry.source,
        target: DamageTarget::Object(*id),
        amount: 5,
    }]
}
