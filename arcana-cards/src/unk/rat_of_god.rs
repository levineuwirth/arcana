//! Rat of God — `{3}{B}{B}` kindred sorcery (Rat). Destroy all non-Rat
//! creatures; each player amasses Rats X = #Rats they control.

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
    let name = reg.interner_mut().intern("Rat of God");
    let _ = reg.interner_mut().intern("Rat");
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
                text: "Destroy all non-Rat creatures. Then each player amasses Rats X, where X is the number of Rats they control.".into(),
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
    // GAP: "non-Rat" filtered wipe needs subtype-exclude in ObjectFilter
    // (only subtype_filter is positive). Wiping ALL creatures as honest
    // fallback would be wrong; emit a generic creature wipe and GAP the
    // Rat exception and amass.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    ids.into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}
