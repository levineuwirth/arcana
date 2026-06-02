//! Mutational Advantage — `{1}{G}{U}` instant. "Permanents you control
//! with counters on them gain hexproof and indestructible until end of
//! turn. Prevent all damage that would be dealt to those permanents this
//! turn. Proliferate."
//!
//! The Proliferate clause is expressed with `Effect::Proliferate`. The
//! protection clause ("permanents you control WITH COUNTERS ON THEM gain
//! hexproof and indestructible; prevent all damage to those permanents")
//! cannot be expressed: the `ObjectFilter` counter predicate matches a
//! SINGLE named counter kind, not "has any counter at all", so the
//! affected-set ("with counters on them") is not selectable with the
//! available filter builders. That clause is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mutational Advantage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Permanents you control with counters on them gain hexproof \
                   and indestructible until end of turn. Prevent all damage \
                   that would be dealt to those permanents this turn. \
                   Proliferate."
                .into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "permanents you control with counters on them" (any-counter set)
    // is not selectable — the has_counter filter predicate matches only a
    // single named CounterKind, not "has any counter at all", so the
    // hexproof/indestructible grant and the damage prevention to that set
    // cannot be expressed. Only the Proliferate clause is emitted.
    vec![Effect::Proliferate]
}
