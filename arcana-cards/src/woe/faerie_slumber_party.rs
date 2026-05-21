//! Faerie Slumber Party — `{4}{U}{U}` sorcery. "Return all creatures to their
//! owners' hands. For each opponent who controlled a creature returned this
//! way, you create two 1/1 blue Faerie creature tokens with flying and 'This
//! token can block only creatures with flying.'"
//!
//! Mass bounce expressible via ForEach + ReturnToHand. The 'for each opponent
//! who controlled a creature returned' token count and the can-only-block
//! creatures-with-flying restriction are not in catalog — GAP both.

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
    let name = reg.interner_mut().intern("Faerie Slumber Party");
    let _ = reg.interner_mut().intern("Faerie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all creatures to their owners' hands. For each opponent who controlled a creature returned this way, you create two 1/1 blue Faerie creature tokens with flying and \"This token can block only creatures with flying.\"".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // GAP: token-creation-count driven by 'distinct opponents who controlled a returned
    // creature' — that bookkeeping isn't expressible with available helpers; GAP rather
    // than emit a fixed token count.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnToHand { target: NULL_OBJECT_ID }),
    }]
}
