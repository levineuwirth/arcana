//! Honor the Fallen — `{1}{W}` instant. "Exile all creature cards from all
//! graveyards. You gain 1 life for each card exiled this way."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::{TargetFilter, TargetCount};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Honor the Fallen");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all creature cards from all graveyards. You gain 1 life for each card exiled this way.".into(),
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
    // Collect all creature cards in all graveyards
    // GAP: no per-player graveyard enumeration helper; using graveyard_matching
    // to count, but ids_matching only looks at the battlefield. We cannot
    // enumerate graveyard object ids with the available script helpers.
    // Best effort: count creatures in all graveyards, gain that much life,
    // but cannot produce ForEach over graveyard ids.
    // GAP: script::ids_matching_in_graveyard — enumerate creature card ids
    //      across all graveyards to produce ExileFromGraveyard effects
    let n = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        // player arg: use entry.controller as sentinel; engine must handle
        // "all players" — this only counts for controller's graveyard
        entry.controller,
        entry.controller,
    );
    // We can only gain life equal to what we can count; the actual exile
    // effects are a GAP.
    vec![
        Effect::GainLife { player: entry.controller, amount: n },
    ]
    // GAP: ExileFromGraveyard for each creature card in every player's graveyard
}
