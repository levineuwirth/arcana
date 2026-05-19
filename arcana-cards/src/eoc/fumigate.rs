//! Fumigate — `{3}{W}{W}` sorcery. "Destroy all creatures. You gain 1 life for
//! each creature destroyed this way."
//!
//! # GAP: "gain 1 life for each creature destroyed this way" — life total equal
//! to number of permanents destroyed by this specific spell cannot be tracked
//! mid-resolution. Best effort: count matching creatures pre-destroy with
//! `script::count_matching`, then ForEach DestroyPermanent, then GainLife(count).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fumigate");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all creatures. You gain 1 life for each creature destroyed this way.".into(),
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
    let all_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    );
    // GAP: life count approximated as pre-destroy creature count, not post-destroy
    let count = all_creatures.len() as u32;
    let mut effects: Vec<Effect> = vec![Effect::ForEach {
        targets: all_creatures,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }];
    if count > 0 {
        effects.push(Effect::GainLife { player: entry.controller, amount: count });
    }
    effects
}
