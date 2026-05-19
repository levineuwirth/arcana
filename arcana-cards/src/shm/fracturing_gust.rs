//! Fracturing Gust — `{2}{G/W}{G/W}{G/W}` instant. "Destroy all artifacts and
//! enchantments. You gain 2 life for each permanent destroyed this way."
//!
//! GAP: gaining 2 life per permanent destroyed requires counting how many were
//! actually destroyed (accounting for indestructible, etc.), which is not
//! accessible at resolve time. We emit the board wipe and a life-gain of 2×
//! the number of matching permanents currently on the battlefield (pre-destroy
//! count as best effort).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fracturing Gust");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all artifacts and enchantments. You gain 2 life for each permanent destroyed this way.".into(),
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
    let filter = ObjectFilter::permanent()
        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT));
    let ids = script::ids_matching(state, &filter, entry.controller);
    let count = ids.len() as u32;
    let mut effects = Vec::new();
    if !ids.is_empty() {
        effects.push(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        });
    }
    if count > 0 {
        effects.push(Effect::GainLife { player: entry.controller, amount: count * 2 });
    }
    effects
}
