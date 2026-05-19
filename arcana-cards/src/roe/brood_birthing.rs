//! Brood Birthing — `{1}{R}` sorcery, "If you control an Eldrazi Spawn, create
//! three 0/1 colorless Eldrazi Spawn creature tokens. They have 'Sacrifice this
//! token: Add {C}.' Otherwise, create one of those tokens."
//!
//! GAP: (1) conditional on controlling an Eldrazi Spawn (subtype predicate on
//! battlefield query); (2) Eldrazi Spawn token has an activated mana ability
//! ('Sacrifice this: Add {C}') — token activated abilities not in TokenDefinition.
//! Best effort: create one 0/1 colorless Eldrazi Spawn token (no activated ability).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brood Birthing");
    let _eldrazi = reg.interner_mut().intern("Eldrazi");
    let _spawn = reg.interner_mut().intern("Spawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "If you control an Eldrazi Spawn, create three 0/1 colorless Eldrazi Spawn creature tokens. They have 'Sacrifice this token: Add {C}.' Otherwise, create one of those tokens.".into(),
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
    // GAP: conditional on controlling an Eldrazi Spawn (subtype battlefield query)
    // GAP: token activated ability "Sacrifice: Add {C}" not expressible
    let eldrazi = reg.interner().lookup("Eldrazi").expect("Eldrazi interned during register()");
    let spawn = reg.interner().lookup("Spawn").expect("Spawn interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(spawn);
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
