//! Howl of the Night Pack — `{6}{G}` sorcery.
//! "Create a 2/2 green Wolf creature token for each Forest you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Howl of the Night Pack");
    let _wolf = reg.interner_mut().intern("Wolf");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 2/2 green Wolf creature token for each Forest you control.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg.interner().lookup("Forest").expect("Forest interned during register()");
    let wolf = reg.interner().lookup("Wolf").expect("Wolf interned during register()");
    let forest_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into());
    // Count forests: use ids_matching on lands then check subtype — script::subtype_filter gives
    // creature subtype only; land subtype filter not available. Best effort: count all lands.
    // GAP: no script helper to count lands of a specific subtype (Forest).
    let count = script::count_matching(
        state,
        &forest_filter,
        entry.controller,
    );
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    let token = TokenDefinition {
        name: wolf,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..count)
        .map(|_| Effect::CreateToken { controller: entry.controller, token: token.clone() })
        .collect()
}
