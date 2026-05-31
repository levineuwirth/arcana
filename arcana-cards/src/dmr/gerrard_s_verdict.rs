//! Gerrard's Verdict — `{W}{B}` sorcery, "Target player discards two
//! cards. You gain 3 life for each land card discarded this way."
//!
//! The discard is expressible. The "gain 3 life for each LAND card
//! discarded this way" rider is a GAP: there is no hook to inspect
//! what cards a just-resolved discard put into the graveyard, and the
//! script:: helper set offers no "count of lands discarded by this
//! effect" amount. Emitting a fixed life-gain would be a materially
//! wrong (dynamic) value, so we emit only the discard and GAP the
//! life-gain rider.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gerrard's Verdict");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player discards two cards. You gain 3 life for each land card discarded this way.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: "gain 3 life for each land card discarded this way" — no
    // engine hook to count how many of the just-discarded cards were
    // lands; the dynamic life-gain rider is therefore omitted.
    vec![Effect::Discard {
        player: *p,
        count: 2,
        choice: DiscardChoice::ControllerChooses,
    }]
}
