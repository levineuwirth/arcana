//! Invoke the Ancients — `{1}{G}{G}{G}{G}` sorcery, "Create two 4/5
//! green Spirit creature tokens. For each of them, put your choice of a
//! vigilance counter, a reach counter, or a trample counter on it."
//!
//! The two 4/5 green Spirit tokens are created faithfully. The follow-up
//! "put your choice of a vigilance/reach/trample counter on it" cannot be
//! expressed: it requires a per-token modal counter choice on a token
//! whose id is not available to the resolver, and keyword-counters
//! (vigilance/reach/trample counters) are not in the CounterKind surface.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invoke the Ancients");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create two 4/5 green Spirit creature tokens. For each of them, put your choice of a vigilance counter, a reach counter, or a trample counter on it.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spirit = reg
        .interner()
        .lookup("Spirit")
        .expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "put your choice of a vigilance/reach/trample counter on it" —
    // a per-token modal counter choice on a token whose id isn't available
    // at resolution, and keyword counters are outside CounterKind.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
