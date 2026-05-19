//! Trapmaker's Snare — `{1}{U}` instant. "Search your library for a Trap card,
//! reveal it, put it into your hand, then shuffle."
//!
//! GAP: TutorToHand's ObjectFilter can filter by type but "Trap" is a subtype,
//! and ObjectFilter has no .with_subtypes() builder shown in the catalog.
//! Best-effort: TutorToHand with a plain permanent filter; Trap subtype
//! restriction is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trapmaker's Snare");
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
                text: "Search your library for a Trap card, reveal it, put it into your hand, then shuffle.".into(),
                target_requirements: vec![],
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
    // GAP: ObjectFilter has no .with_subtypes() for Trap subtype filtering.
    use arcana_core::effects::Effect;
    use arcana_core::targets::ObjectFilter;
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::permanent(),
        reveal: true,
    }]
}
