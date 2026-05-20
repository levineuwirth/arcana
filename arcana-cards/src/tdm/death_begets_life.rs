//! Death Begets Life — `{5}{B}{G}{U}` sorcery. "Destroy all creatures
//! and enchantments. Draw a card for each permanent destroyed this
//! way."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death Begets Life");
    let chars = Characteristics {
        name,
        mana_cost: Some(
            ManaCost::parse("{5}{B}{G}{U}").expect("valid cost"),
        ),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures and enchantments. Draw a card for each permanent destroyed this way.".into(),
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
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    );
    let enchantments = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types(TypeLine::ENCHANTMENT.into()),
        entry.controller,
    );
    let n = (creatures.len() + enchantments.len()) as u32;
    let mut targets = creatures;
    targets.extend(enchantments);
    vec![
        Effect::ForEach {
            targets,
            effect: Box::new(Effect::DestroyPermanent {
                target: NULL_OBJECT_ID,
            }),
        },
        Effect::DrawCards { player: entry.controller, count: n },
    ]
}
