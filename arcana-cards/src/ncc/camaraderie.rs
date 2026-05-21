//! Camaraderie — `{4}{G}{W}` sorcery. "You gain X life and draw X cards,
//! where X is the number of creatures you control. Creatures you control get
//! +1/+1 until end of turn."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Camaraderie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You gain X life and draw X cards, where X is the number of creatures you control. Creatures you control get +1/+1 until end of turn.".into(),
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
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, entry.controller);
    let ids = script::ids_matching(state, &filter, entry.controller);
    vec![
        Effect::GainLife { player: entry.controller, amount: n },
        Effect::DrawCards { player: entry.controller, count: n },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 1,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        },
    ]
}
