//! Liliana's Influence — `{4}{B}{B}` sorcery.
//! "Put a -1/-1 counter on each creature you don't control. You may search your library
//! and/or graveyard for a card named Liliana, Death Wielder, reveal it, and put it into
//! your hand. If you search your library this way, shuffle."
//! GAP: search library/graveyard for a specific named card not in Effect catalog (TutorToHand
//! uses ObjectFilter, not a name-based filter).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana's Influence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Put a -1/-1 counter on each creature you don't control. You may search your library and/or graveyard for a card named Liliana, Death Wielder, reveal it, and put it into your hand. If you search your library this way, shuffle.".into(),
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
    // GAP: search library and/or graveyard for a specific named card not expressible with ObjectFilter
    let opponent_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    opponent_creatures
        .into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::MinusOneMinusOne,
            count: 1,
        })
        .collect()
}
