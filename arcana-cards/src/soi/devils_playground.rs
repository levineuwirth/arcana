//! Devils' Playground — `{4}{R}{R}` sorcery.
//! "Create four 1/1 red Devil creature tokens. They have 'When this token dies,
//! it deals 1 damage to any target.'"
//!
//! GAP: Devil token triggered ability "When this token dies, it deals 1 damage
//! to any target" is not expressible via the TokenDefinition abilities field.
//! The tokens are created without the triggered ability.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Devils' Playground");
    let _devil = reg.interner_mut().intern("Devil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create four 1/1 red Devil creature tokens. They have \"When this token dies, it deals 1 damage to any target.\"".into(),
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
    let devil = reg.interner().lookup("Devil").expect("Devil interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    // GAP: devil "when dies deals 1 damage" triggered ability not in TokenDefinition
    let token = TokenDefinition {
        name: devil,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
