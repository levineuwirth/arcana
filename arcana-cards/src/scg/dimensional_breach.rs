//! Dimensional Breach — `{5}{W}{W}` sorcery, "Exile all permanents. For as
//! long as any of those cards remain exiled, at the beginning of each player's
//! upkeep, that player returns one of the exiled cards they own to the
//! battlefield."
//! GAP: triggered upkeep return mechanic for exiled-by-this-spell tracking not
//! expressible; best effort exiles all permanents.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dimensional Breach");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all permanents. For as long as any of those cards remain exiled, at the beginning of each player's upkeep, that player returns one of the exiled cards they own to the battlefield.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent(),
        entry.controller,
    );
    // GAP: upkeep-triggered return for exiled permanents not expressible
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
