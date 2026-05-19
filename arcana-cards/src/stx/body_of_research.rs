//! Body of Research — `{G}{G}{G}{U}{U}{U}` sorcery.
//! "Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it,
//! where X is the number of cards in your library."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Body of Research");
    let _fractal = reg.interner_mut().intern("Fractal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{G}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 0/0 green and blue Fractal creature token. Put X +1/+1 counters on it, where X is the number of cards in your library.".into(),
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
    let lib_count = script::library_size(state, entry.controller);
    let fractal = reg.interner().lookup("Fractal")
        .expect("Fractal interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    let token = TokenDefinition {
        name: fractal,
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: no way to AddCounters to a token that was just created (token has no ObjectId yet at resolve time)
    // Best effort: create the token; the counter addition cannot be expressed.
    vec![
        Effect::CreateToken { controller: entry.controller, token },
    ]
    // GAP: AddCounters equal to library_size on the newly created token is not expressible with the current catalog
}
