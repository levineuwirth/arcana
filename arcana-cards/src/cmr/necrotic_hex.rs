//! Necrotic Hex — `{6}{B}` sorcery. "Each player sacrifices six creatures of their choice. You
//! create six tapped 2/2 black Zombie creature tokens."
//! GAP: forced sacrifice of N creatures of each player's choice not in catalog.
//! Best effort: create six Zombie tokens; sacrifice part is GAP.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necrotic Hex");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player sacrifices six creatures of their choice. You create six tapped 2/2 black Zombie creature tokens.".into(),
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
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: each player sacrifices six creatures of their choice
    // Note: tokens should enter tapped; tapped-entry not modeled in TokenDefinition — GAP.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
