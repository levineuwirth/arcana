//! Cry of the Carnarium — `{1}{B}{B}` sorcery, "All creatures get -2/-2 until end of turn.
//! Exile all creature cards in all graveyards that were put there from the battlefield this turn.
//! If a creature would die this turn, exile it instead."
//!
//! GAP: "exile all creature cards put into graveyards from battlefield this turn" (tracking
//! zone-move history) and replacement "if a creature would die, exile it instead" are not
//! expressible via the catalog. Partial: emit -2/-2 to all creatures via ForEach+Pump.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cry of the Carnarium");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "All creatures get -2/-2 until end of turn. Exile all creature cards in all graveyards that were put there from the battlefield this turn. If a creature would die this turn, exile it instead.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Pump {
                target: arcana_core::objects::NULL_OBJECT_ID,
                power: -2,
                toughness: -2,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
        // GAP: exile graveyard creatures that died this turn not expressible
        // GAP: replacement effect "die → exile" not expressible
    ]
}
