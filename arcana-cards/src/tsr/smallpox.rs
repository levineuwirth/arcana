//! Smallpox — `{B}{B}` sorcery. "Each player loses 1 life, discards a
//! card, sacrifices a creature, then sacrifices a land."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smallpox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player loses 1 life, discards a card, sacrifices a creature, then sacrifices a land.".into(),
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
    let effects: Vec<Effect> = script::all_players(state).into_iter().flat_map(|p| {
        vec![
            Effect::LoseLife { player: p, amount: 1 },
            Effect::Discard { player: p, count: 1, choice: DiscardChoice::ControllerChooses },
            Effect::Sacrifice { player: p, filter: ObjectFilter::creature(), count: 1 },
            Effect::Sacrifice {
                player: p,
                filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                count: 1,
            },
        ]
    }).collect();
    vec![Effect::Sequence(effects)]
}
