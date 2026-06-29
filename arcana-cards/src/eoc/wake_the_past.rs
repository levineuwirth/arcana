//! Wake the Past — `{5}{R}{W}` sorcery. "Return all artifact cards from your
//! graveyard to the battlefield. They gain haste until end of turn."
//! Partial implementation: returns all artifact cards from graveyard to
//! battlefield via ForEach + ReturnFromGraveyardToBattlefield.
//! GAP: cannot grant Haste to the newly-returned permanents — zone-change
//! assigns new object IDs on entering the battlefield, so the graveyard IDs
//! used in ForEach are invalid for a subsequent GrantKeyword effect.

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
    let name = reg.interner_mut().intern("Wake the Past");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all artifact cards from your graveyard to the battlefield. They gain haste until end of turn.".into(),
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
    let filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let ids = script::graveyard_ids_matching(state, &filter, entry.controller, entry.controller);
    // GAP: cannot grant Haste to returned permanents — zone-change assigns new
    // battlefield object IDs; the graveyard ids in `ids` are no longer valid
    // for GrantKeyword after ReturnFromGraveyardToBattlefield resolves.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnFromGraveyardToBattlefield {
            target: NULL_OBJECT_ID,
        }),
    }]
}
