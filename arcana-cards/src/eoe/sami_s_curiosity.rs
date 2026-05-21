//! Sami's Curiosity — `{G}` sorcery. "You gain 2 life. Create a Lander token.
//! (It's an artifact with '{2}, {T}, Sacrifice this token: Search your library
//! for a basic land card, put it onto the battlefield tapped, then shuffle.')"

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sami's Curiosity");
    let _lander = reg.interner_mut().intern("Lander");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You gain 2 life. Create a Lander token.".into(),
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
    let lander = reg.interner().lookup("Lander").expect("interned");
    let subtypes = SubtypeSet::default();
    // GAP: the Lander token's activated land-tutor ability cannot be attached to a
    // TokenDefinition — the artifact token is created without its ability.
    let token = TokenDefinition {
        name: lander,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::GainLife { player: entry.controller, amount: 2 },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
