//! Mind Burst — `{1}{B}` sorcery. "Target player discards X cards, where X
//! is one plus the number of cards named Mind Burst in all graveyards."
//!
//! X is dynamic: 1 plus the count of cards named "Mind Burst" across every
//! player's graveyard. Computed at resolution with `script::graveyard_matching`
//! summed over all players, using an `ObjectFilter` constrained by name.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mind Burst");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player discards X cards, where X is one plus the number of cards named Mind Burst in all graveyards.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(player) = target else { return Vec::new(); };

    let name = reg.interner().lookup("Mind Burst");
    let filter = ObjectFilter { name, ..ObjectFilter::default() };
    let mut count: u32 = 1;
    for p in script::all_players(state) {
        count += script::graveyard_matching(state, &filter, p, entry.controller);
    }

    vec![Effect::Discard {
        player: *player,
        count,
        choice: DiscardChoice::ControllerChooses,
    }]
}
