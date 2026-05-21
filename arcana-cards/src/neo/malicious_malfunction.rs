//! Malicious Malfunction — `{1}{B}{B}` sorcery. "All creatures get
//! -2/-2 until end of turn. If a creature would die this turn, exile
//! it instead." The -X/-X to all creatures is a Pump on each creature
//! with negative numbers; the exile-instead replacement is not in the
//! catalog, so we GAP that part.

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
    let name = reg.interner_mut().intern("Malicious Malfunction");
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
                text: "All creatures get -2/-2 until end of turn. If a creature would die this turn, exile it instead.".into(),
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
    // GAP: 'if a creature would die this turn, exile it instead' —
    // replacement effect not in catalog.
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    ids.into_iter()
        .map(|id| Effect::Pump { target: id, power: -2, toughness: -2, duration: Duration::EndOfTurn, keywords: vec![] })
        .collect()
}
