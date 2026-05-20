//! Angrath's Fury — `{3}{B}{R}` sorcery. "Destroy target creature.
//! Angrath's Fury deals 3 damage to target player or planeswalker. You
//! may search your library and/or graveyard for a card named Angrath,
//! Minotaur Pirate, reveal it, put it into your hand, shuffle."

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
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

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the by-name tutor (search a specific card) is not
    // expressible; emitting the destroy + damage halves.
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::DestroyPermanent { target: *id });
    }
    if let Some(TargetChoice::Player(p)) = entry.targets.targets.get(1) {
        effects.push(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(*p),
            amount: 3,
        });
    }
    effects
}
