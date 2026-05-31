//! Snarl Song — `{5}{G}` sorcery. "Converge — Create two 0/0 green and
//! blue Fractal creature tokens. Put X +1/+1 counters on each of them and
//! you gain X life, where X is the number of colors of mana spent to cast
//! this spell."
//!
//! The two Fractal tokens are created. The Converge value X (the number of
//! colors of mana spent to cast this spell) is not exposed by any `script::`
//! helper, so the X +1/+1 counters and the X life gain cannot be computed.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snarl Song");
    let _fractal = reg.interner_mut().intern("Fractal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Converge — Create two 0/0 green and blue Fractal creature tokens. Put X +1/+1 counters on each of them and you gain X life, where X is the number of colors of mana spent to cast this spell.".into(),
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
    // GAP: Converge X (number of colors of mana spent to cast this spell) is
    // not exposed by any script:: helper, so the X +1/+1 counters on each
    // token and the X life gain cannot be computed.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
