//! Rhonas's Last Stand — `{G}{G}` sorcery. "Create a 5/4 green Snake
//! creature token. Lands you control don't untap during your next
//! untap step."
//!
//! # GAP
//! "Lands you control don't untap during your next untap step" is a
//! delayed continuous effect with a once-only duration — not
//! expressible with any catalog variant. The token creation is
//! modeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rhonas's Last Stand");
    let _snake = reg.interner_mut().intern("Snake");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 5/4 green Snake creature token. Lands you control don't untap during your next untap step.".into(),
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
    let snake = reg.interner().lookup("Snake").expect("Snake interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    let token = TokenDefinition {
        name: snake,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "lands don't untap during your next untap step" delayed continuous effect not expressible
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
