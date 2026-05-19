//! Reign of Terror — `{3}{B}{B}` sorcery.
//! "Destroy all green creatures and all white creatures. You gain life
//! equal to the number of creatures destroyed this way."
//!
//! GAP: modal board-wipe with life-gain equal to the count of destroyed
//! permanents requires tracking destruction count at resolution;
//! no script:: helper counts objects destroyed during the same resolution.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reign of Terror");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all green creatures and all white creatures. You gain life equal to the number of creatures destroyed this way.".into(),
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
    let green_filter = ObjectFilter::creature().with_colors(ColorSet::green());
    let white_filter = ObjectFilter::creature().with_colors(ColorSet::white());
    let green_ids = script::ids_matching(state, &green_filter, entry.controller);
    let white_ids = script::ids_matching(state, &white_filter, entry.controller);
    // Combine unique ids (white and green overlap if multicolored)
    let mut all_ids: Vec<_> = green_ids;
    for id in white_ids {
        if !all_ids.contains(&id) {
            all_ids.push(id);
        }
    }
    // GAP: life-gain equal to destruction count not expressible (count tracked
    // during resolution not available); omitting the life-gain component.
    let n = all_ids.len() as u32;
    let mut effects = vec![Effect::ForEach {
        targets: all_ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    // Best-effort: emit life-gain with pre-resolution count (may overcount due
    // to indestructible, but is the closest approximation available)
    if n > 0 {
        effects.push(Effect::GainLife { player: entry.controller, amount: n });
    }
    effects
}
