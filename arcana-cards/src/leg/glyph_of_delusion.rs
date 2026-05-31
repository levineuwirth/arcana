//! Glyph of Delusion — `{U}` instant. "Put X glyph counters on target
//! creature that target Wall blocked this turn, where X is the power of
//! that blocked creature. The creature gains \"This creature doesn't
//! untap during your untap step if it has a glyph counter on it\" and
//! \"At the beginning of your upkeep, remove a glyph counter from this
//! creature.\""
//!
//! Implemented partially: we target a creature and place X glyph
//! counters on it, where X is that creature's current power
//! (`script::power_of`). The "that target Wall blocked this turn"
//! restriction (a combat-history target filter on a second Wall target)
//! and the two granted abilities (don't-untap-while-glyph-countered and
//! the upkeep remove-a-glyph-counter trigger) are not expressible with
//! the available API.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glyph of Delusion");
    let _glyph = reg.interner_mut().intern("glyph");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put X glyph counters on target creature that target Wall blocked this turn, where X is the power of that blocked creature. The creature gains \"This creature doesn't untap during your untap step if it has a glyph counter on it\" and \"At the beginning of your upkeep, remove a glyph counter from this creature.\"".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let x = script::power_of(state, *id).max(0) as u32;
    let glyph = reg.interner().lookup("glyph").expect("interned in register");
    // GAP: "that target Wall blocked this turn" combat-history target filter
    // (second Wall target) and the two granted abilities (don't-untap-while-
    // glyph-countered static + upkeep remove-a-glyph-counter trigger) are not
    // expressible with the available API.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(glyph),
        count: x,
    }]
}
