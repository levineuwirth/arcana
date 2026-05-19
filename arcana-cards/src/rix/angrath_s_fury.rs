//! Angrath's Fury — `{3}{B}{R}` sorcery. "Destroy target creature. Angrath's
//! Fury deals 3 damage to target player or planeswalker. You may search your
//! library and/or graveyard for a card named Angrath, Minotaur Pirate, reveal
//! it, and put it into your hand. If you search your library this way, shuffle."
//!
//! # GAP: TutorToHand with specific named-card filter (not subtype/type)
//! # GAP: optional search of library AND/OR graveyard for a named card
//! Best-effort: destroy creature + deal damage; the tutor is omitted.

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
    let name = reg.interner_mut().intern("Angrath's Fury");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target creature. Angrath's Fury deals 3 damage to target player or planeswalker. You may search your library and/or graveyard for a card named Angrath, Minotaur Pirate, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_player(),
                ],
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
    // GAP: named-card tutor (search library/graveyard for specific card) not in catalog
    let mut effects = Vec::new();
    if let Some(t) = entry.targets.targets.first() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    if let Some(t) = entry.targets.targets.get(1) {
        let dt = match t {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            _ => return effects,
        };
        effects.push(Effect::DealDamage { source: entry.source, target: dt, amount: 3 });
    }
    effects
}
