//! Liliana's Triumph — `{1}{B}` instant. "Each opponent sacrifices a
//! creature of their choice. If you control a Liliana planeswalker,
//! each opponent also discards a card."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana's Triumph");
    let _liliana = reg.interner_mut().intern("Liliana");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each opponent sacrifices a creature of their choice. If you control a Liliana planeswalker, each opponent also discards a card.".into(),
            target_requirements: vec![],
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
    let mut effects: Vec<Effect> = Vec::new();
    for opp in script::opponents(state, entry.controller) {
        effects.push(Effect::Sacrifice {
            player: opp,
            filter: ObjectFilter::creature(),
            count: 1,
        });
    }
    // Conditional rider: "If you control a Liliana planeswalker, each
    // opponent also discards a card." Use a Liliana-subtype filter.
    let liliana_filter = script::subtype_filter(reg, "Liliana")
        .controlled_by(ControllerConstraint::You);
    let count_liliana = script::count_matching(state, &liliana_filter, entry.controller);
    if count_liliana > 0 {
        for opp in script::opponents(state, entry.controller) {
            effects.push(Effect::Discard {
                player: opp,
                count: 1,
                choice: DiscardChoice::ControllerChooses,
            });
        }
    }
    effects
}
