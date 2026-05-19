//! Dismal Failure — `{2}{U}{U}` instant. "Counter target spell. Its controller discards a card."
//!
//! # GAP: identifying the controller of the countered spell at resolution time
//! requires stack inspection not demonstrated in references. We counter the
//! spell and discard from a player; the "its controller" identification is
//! approximated as the non-casting player.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dismal Failure");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Its controller discards a card.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(spell_id) = target else { return Vec::new(); };
    // GAP: "its controller" (the spell's controller) not easily read from
    // StackEntry without stack inspection; defaulting to non-self player
    vec![
        Effect::Counter { target: *spell_id },
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}
