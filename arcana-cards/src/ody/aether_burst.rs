//! Aether Burst — `{1}{U}` instant. "Return up to X target creatures
//! to their owners' hands, where X is one plus the number of cards
//! named Aether Burst in all graveyards as you cast this spell."
//!
//! X is a CAST-TIME target-count cap derived from a name-count across
//! ALL graveyards (1 + that count). The `script::*` helpers expose
//! per-player graveyard size but no "cards named N in all graveyards"
//! accessor, and `target_requirements` (which fix the legal target
//! count) are static — they cannot read graveyard state at cast time.
//! So the dynamic cap X cannot be computed. We honestly model the
//! expressible part — "return up to any number of target creatures to
//! their owners' hands" — and GAP the X cap derivation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aether Burst");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return up to X target creatures to their owners' hands, where X is one plus the number of cards named Aether Burst in all graveyards as you cast this spell.".into(),
                // GAP: X cap = 1 + (cards named "Aether Burst" in all
                // graveyards) is a cast-time dynamic target count; no
                // script:: accessor for a name-count across all
                // graveyards, and static target_requirements cannot
                // read graveyard state. Modeled as "up to any number".
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Any,
                    controller: None,
                }],
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
    entry
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ReturnToHand { target: *id }),
            _ => None,
        })
        .collect()
}
