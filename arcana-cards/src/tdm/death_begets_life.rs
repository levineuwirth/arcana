//! Death Begets Life — `{5}{B}{G}{U}` sorcery. "Destroy all creatures and
//! enchantments. Draw a card for each permanent destroyed this way."
//!
//! GAP: drawing a card for each permanent destroyed requires tracking the
//! count of successful destructions at resolution time, which is not
//! expressible (ForEach executes effects but does not count surviving vs.
//! indestructible permanents). Best effort: destroy all creatures and
//! enchantments, then draw based on the pre-wipe count.

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
    let name = reg.interner_mut().intern("Death Begets Life");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures and enchantments. Draw a card for each permanent destroyed this way.".into(),
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
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    let enchantment_ids = script::ids_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
        entry.controller,
    );
    // Count before destruction for draw (best-effort; doesn't account for
    // indestructible permanents surviving).
    let total = (creature_ids.len() + enchantment_ids.len()) as u32;
    let mut effects: Vec<Effect> = Vec::new();
    for id in creature_ids {
        effects.push(Effect::DestroyPermanent { target: id });
    }
    for id in enchantment_ids {
        effects.push(Effect::DestroyPermanent { target: id });
    }
    if total > 0 {
        effects.push(Effect::DrawCards { player: entry.controller, count: total });
    }
    effects
}
