//! Ribbons of the Reikai — `{4}{U}` sorcery — Arcane.
//! "Draw a card for each Spirit you control."
//! GAP: no subtype filter for Spirit-typed creatures in the scripting API (script::subtype_filter is for creatures by creature subtype, but here it's used in a count — this is expressible via subtype_filter + count_matching).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ribbons of the Reikai");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card for each Spirit you control.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Spirit");
    let n = script::count_matching(state, &filter, entry.controller);
    vec![Effect::DrawCards { player: entry.controller, count: n }]
}
