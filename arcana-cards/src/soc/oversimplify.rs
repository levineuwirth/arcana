//! Oversimplify — `{3}{G}{U}` sorcery. "Exile all creatures. Each
//! player creates a 0/0 green and blue Fractal creature token and
//! puts a number of +1/+1 counters on it equal to the total power of
//! creatures they controlled that were exiled this way." We emit the
//! exile-all-creatures; the per-player token sized by exiled-this-
//! cast power isn't tractable with current helpers.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oversimplify");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile all creatures. Each player creates a 0/0 green and blue Fractal creature token and puts a number of +1/+1 counters on it equal to the total power of creatures they controlled that were exiled this way.".into(),
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
    // GAP: per-player Fractal token whose counter count tracks
    // exiled-this-cast power-by-controller — not expressible via
    // catalog (no two-step token creation referencing exiled batch).
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    ids.into_iter().map(|id| Effect::ExilePermanent { target: id }).collect()
}
