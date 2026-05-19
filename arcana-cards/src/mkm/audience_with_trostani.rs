//! Audience with Trostani — `{2}{G}` sorcery, "Create a 0/1 green Plant
//! creature token, then draw cards equal to the number of differently named
//! creature tokens you control."
//!
//! GAP: counting differently named creature tokens you control is not in the
//! script helpers. Best-effort: create the Plant token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Audience with Trostani");
    let _plant = reg.interner_mut().intern("Plant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 0/1 green Plant creature token, then draw cards equal to the number of differently named creature tokens you control.".into(),
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
    let plant = reg.interner().lookup("Plant").expect("Plant interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    let token = TokenDefinition {
        name: plant,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: draw count = number of differently named creature tokens not in script helpers
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
