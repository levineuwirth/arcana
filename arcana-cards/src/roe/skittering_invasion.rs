//! Skittering Invasion — `{7}` Kindred Sorcery — Eldrazi. "Create
//! five 0/1 colorless Eldrazi Spawn creature tokens. They have
//! 'Sacrifice this token: Add {C}.'"
//!
//! The token's mana ability is not expressible via TokenDefinition;
//! the five 0/1 tokens are created without it.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skittering Invasion");
    let _el = reg.interner_mut().intern("Eldrazi");
    let _sp = reg.interner_mut().intern("Spawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create five 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\"".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let el = reg.interner().lookup("Eldrazi").expect("Eldrazi interned");
    let sp = reg.interner().lookup("Spawn").expect("Spawn interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(el);
    subtypes.0.insert(sp);
    let token = TokenDefinition {
        name: sp,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token's "Sacrifice this token: Add {C}" mana ability is
    // not expressible via TokenDefinition.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
