//! Calamitous Cave-In — `{3}{R}` sorcery. "Calamitous Cave-In deals X damage
//! to each creature and each planeswalker, where X is the number of Caves you
//! control plus the number of Cave cards in your graveyard."
//!
//! # GAP: counting Cave-subtype permanents on battlefield and Cave cards in
//! graveyard — subtype_filter matches creatures only, not generic permanent
//! subtypes (land subtypes like Cave). graveyard_matching may cover graveyard
//! side partially. Damage to planeswalkers is also not directly expressible.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Calamitous Cave-In");
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
                text: "Calamitous Cave-In deals X damage to each creature and each planeswalker, where X is the number of Caves you control plus the number of Cave cards in your graveyard.".into(),
                target_requirements: vec![],
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
    use arcana_core::script;
    // GAP: count Cave-subtype lands (subtype_filter is creature-only); graveyard Cave count
    // GAP: damage to planeswalkers
    // Best effort: deal 0 damage to each creature (X approximated as 0)
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    creature_ids.into_iter().map(|id| {
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(id),
            amount: 0,
        }
    }).collect()
}
