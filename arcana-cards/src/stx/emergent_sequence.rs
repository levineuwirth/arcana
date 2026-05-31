//! Emergent Sequence — `{1}{G}` sorcery. "Search your library for a
//! basic land card, put it onto the battlefield tapped, then shuffle.
//! That land becomes a 0/0 green and blue Fractal creature that's still
//! a land. Put a +1/+1 counter on it for each land you had enter the
//! battlefield under your control this turn."
//!
//! The library search for a basic land entering tapped is expressible
//! via `Effect::TutorToBattlefield`. The animation of that specific
//! land into a Fractal creature and the dynamic +1/+1 counters are NOT
//! expressible: the resolver has no handle on the just-fetched land's
//! ObjectId, and "lands that entered this turn" is not a `script::`
//! quantity. Those riders are GAPped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emergent Sequence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle. That land becomes a 0/0 green and blue Fractal creature that's still a land. Put a +1/+1 counter on it for each land you had enter the battlefield under your control this turn.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: cannot animate the just-fetched land into a 0/0 Fractal
    // creature (resolver has no handle on the tutored land's ObjectId),
    // and cannot count "lands that entered the battlefield this turn"
    // for the dynamic +1/+1 counters (no matching script:: helper).
    let basic_land = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: basic_land,
        tapped: true,
    }]
}
