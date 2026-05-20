//! Return of the Nightstalkers — `{5}{B}{B}` sorcery. "Return all
//! Nightstalker permanent cards from your graveyard to the
//! battlefield. Then destroy all Swamps you control."
//!
//! Mass graveyard-to-battlefield return by subtype has no script
//! helper (no graveyard-ids enumerator) — that half is a GAP. The
//! "destroy all Swamps you control" half is emitted via a subtype
//! filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Return of the Nightstalkers");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return all Nightstalker permanent cards from your graveyard to the battlefield. Then destroy all Swamps you control.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mass graveyard-to-battlefield return by subtype has no
    // script helper.
    let swamps = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Swamp")
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: swamps,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}
