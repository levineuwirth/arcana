//! Splendid Reclamation — `{3}{G}` sorcery.
//! "Return all land cards from your graveyard to the battlefield tapped."
//!
//! Collects land-card IDs from your graveyard via `graveyard_ids_matching`,
//! then issues a ForEach `ReturnFromGraveyardToBattlefield` followed by a
//! second ForEach `Tap` pass (IDs are stable across zone changes, so the tap
//! fires on the newly placed battlefield permanents).

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
    let name = reg.interner_mut().intern("Splendid Reclamation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all land cards from your graveyard to the battlefield tapped.".into(),
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
    let filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let ids = script::graveyard_ids_matching(state, &filter, entry.controller, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::ReturnFromGraveyardToBattlefield {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Tap {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        },
    ]
}
