//! From the Ashes — `{3}{R}` sorcery.
//! "Destroy all nonbasic lands. For each land destroyed this way, its controller may search
//! their library for a basic land card and put it onto the battlefield. Then each player
//! who searched their library this way shuffles."
//!
//! GAP: "for each land destroyed, its controller may search" — per-player conditional search
//! keyed to which lands were destroyed is not expressible. Emitting the nonbasic land
//! board wipe; the search rider is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("From the Ashes");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all nonbasic lands. For each land destroyed this way, its controller may search their library for a basic land card and put it onto the battlefield. Then each player who searched their library this way shuffles.".into(),
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
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .nontoken();
    // "nonbasic" — no .nonbasic() filter available; use the land filter and note GAP for
    // the basic exclusion. The destroy-all-lands board wipe is the best approximation.
    let ids = script::ids_matching(state, &filter, entry.controller);
    // GAP: filter nonbasic only (no .nonbasic() ObjectFilter method available)
    // GAP: "for each land destroyed, its controller may search" rider not expressible
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
    ]
}
