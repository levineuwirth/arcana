//! Outlaws' Fury — `{2}{R}` instant, "Creatures you control get +2/+0 until end of turn.
//! If you control an outlaw, exile the top card of your library. Until the end of your
//! next turn, you may play that card."
//!
//! GAP: 'creatures you control get +2/+0' requires a board-wide Pump applied to each
//! controlled creature. The conditional 'exile top card + play until next turn' requires
//! an exile-and-cast effect not in the catalog. Returning Vec::new() for the full effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::layers::Duration;
use arcana_core::effects::KeywordAbility;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Outlaws' Fury");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Creatures you control get +2/+0 until end of turn. If you control an outlaw, exile the top card of your library. Until the end of your next turn, you may play that card.".into(),
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
    // GAP: conditional exile-top-card-and-play-until-next-turn effect not in catalog
    // GAP: 'outlaw' subtype check (Assassin, Mercenary, Pirate, Rogue, Warlock) not in ObjectFilter
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(arcana_core::targets::ControllerConstraint::You),
        entry.controller,
    );
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect()
}
