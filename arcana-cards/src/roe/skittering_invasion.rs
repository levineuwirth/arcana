//! Skittering Invasion — `{7}` kindred sorcery (Eldrazi).
//! "Create five 0/1 colorless Eldrazi Spawn creature tokens. They have
//! 'Sacrifice this token: Add {C}.'"
//! GAP: token activated ability 'Sacrifice this: Add {C}' not expressible in TokenDefinition.abilities.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skittering Invasion");
    let _eldrazi = reg.interner_mut().intern("Eldrazi");
    let _spawn = reg.interner_mut().intern("Spawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create five 0/1 colorless Eldrazi Spawn creature tokens. They have \"Sacrifice this token: Add {C}.\"".into(),
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
    let eldrazi = reg.interner().lookup("Eldrazi")
        .expect("Eldrazi interned during register()");
    let spawn = reg.interner().lookup("Spawn")
        .expect("Spawn interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(spawn);
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: token activated ability 'Sacrifice this: Add {C}'
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
