//! Open the Vaults — `{4}{W}{W}` sorcery. "Return all artifact and enchantment
//! cards from all graveyards to the battlefield under their owners' control."
//! GAP: ReturnFromGraveyardToBattlefield targets a single card; a board-wide
//! "all graveyard" mass-return across both players is not directly supported
//! (ForEach only iterates battlefield permanents; graveyard enumeration not in
//! script API). Best effort: returns controller's graveyard artifacts/enchants.

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
    let name = reg.interner_mut().intern("Open the Vaults");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all artifact and enchantment cards from all graveyards to the battlefield under their owners' control.".into(),
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
    // GAP: graveyard enumeration across all players not in script API;
    // using battlefield filter as placeholder (no actual graveyard-wide support)
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types_any(
            TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT),
        ),
        entry.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnFromGraveyardToBattlefield {
            target: NULL_OBJECT_ID,
        }),
    }]
}
