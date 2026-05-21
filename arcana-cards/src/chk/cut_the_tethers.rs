//! Cut the Tethers — `{2}{U}{U}` sorcery. "For each Spirit, return it
//! to its owner's hand unless that player pays {3}." Unless-pays is
//! not a battlefield-effect primitive (only CounterUnlessPays exists
//! for stack spells); emit the bounce and GAP the tax.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cut the Tethers");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "For each Spirit, return it to its owner's hand unless that player pays {3}.".into(),
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
    // GAP: "unless that player pays {3}" tax on a permanent-affecting effect is not in catalog.
    let ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Spirit"),
        entry.controller,
    );
    ids.into_iter()
        .map(|id| Effect::ReturnToHand { target: id })
        .collect()
}
