//! Cry of the Carnarium — `{1}{B}{B}` sorcery. "All creatures get
//! -2/-2 until end of turn. Exile all creature cards in all
//! graveyards that were put there from the battlefield this turn.
//! If a creature would die this turn, exile it instead." The
//! all-graveyards selective-exile + dies-replacement-this-turn
//! aren't catalog primitives — best-effort: -2/-2 wipe; GAP the
//! exile-replacement and graveyard sweep.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    // GAP: this-turn graveyard exile + dies-replacement.
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
    vec![Effect::ForEach {
        targets: script::ids_matching(state, &ObjectFilter::creature(), entry.controller),
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -2,
            toughness: -2,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
