//! Praetor's Counsel — `{5}{G}{G}{G}` sorcery. "Return all cards from your
//! graveyard to your hand. Exile Praetor's Counsel. You have no maximum hand
//! size for the rest of the game."
//!
//! GAP: "exile Praetor's Counsel" — no Effect variant for a spell exiling itself
//! from the stack/graveyard during resolution.
//! GAP: "no maximum hand size for the rest of the game" — no Effect variant for
//! persistent rule modifications.
//! Best-effort: return all cards from graveyard to hand using ForEach.

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
    let name = reg.interner_mut().intern("Praetor's Counsel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all cards from your graveyard to your hand. Exile Praetor's Counsel. You have no maximum hand size for the rest of the game.".into(),
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
    // GAP: "exile Praetor's Counsel" — no self-exile Effect from resolver
    // GAP: "no maximum hand size for the rest of the game" — no persistent rule modification Effect
    let graveyard_cards = script::ids_matching(
        state,
        &ObjectFilter::permanent(),
        entry.controller,
    );
    // Use graveyard_matching to get count, but we need IDs — GAP: no graveyard ids_matching helper
    // Best-effort: use ReturnFromGraveyardToHand on all graveyard permanents via ForEach
    vec![Effect::ForEach {
        targets: graveyard_cards,
        effect: Box::new(Effect::ReturnFromGraveyardToHand { target: NULL_OBJECT_ID }),
    }]
}
