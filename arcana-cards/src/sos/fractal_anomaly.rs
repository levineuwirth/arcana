//! Fractal Anomaly — `{U}` instant. "Create a 0/0 green and blue
//! Fractal creature token and put X +1/+1 counters on it, where X is
//! the number of cards you've drawn this turn."
//!
//! The token is created faithfully, but the +1/+1 counters can't be
//! placed: the resolver has no handle to the freshly-created token's
//! ObjectId, and "the number of cards you've drawn this turn" is not
//! one of the permitted `script::*` quantities. Emitting the token
//! and dropping the counters would be a materially wrong card, so the
//! counter rider is GAPed below.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fractal Anomaly");
    let _fractal = reg.interner_mut().intern("Fractal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 0/0 green and blue Fractal creature token and put X +1/+1 counters on it, where X is the number of cards you've drawn this turn.".into(),
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
    let fractal = reg
        .interner()
        .lookup("Fractal")
        .expect("Fractal interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    let token = TokenDefinition {
        name: fractal,
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: cannot place the X +1/+1 counters — no ObjectId handle to the
    // freshly-created token, and "number of cards drawn this turn" is not
    // an available script:: quantity.
    vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }]
}
