//! Pest Summoning — `{1}{B/G}{B/G}` sorcery (Lesson). "Create two 1/1
//! black and green Pest creature tokens with 'When this token dies,
//! you gain 1 life.'"
//!
//! GAP: the per-token death-triggered ability isn't expressible on a
//! TokenDefinition (abilities vec accepts the engine's ability
//! struct, not a free-form trigger spec). Tokens are emitted without
//! the trigger.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pest Summoning");
    let _pest = reg.interner_mut().intern("Pest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create two 1/1 black and green Pest creature tokens with \"When this token dies, you gain 1 life.\"".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let pest = reg.interner().lookup("Pest").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest);
    let token = TokenDefinition {
        name: pest,
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: token death trigger 'you gain 1 life' not encoded.
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
