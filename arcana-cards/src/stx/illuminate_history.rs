//! Illuminate History — `{2}{R}{R}` sorcery — Lesson. "Discard any
//! number of cards, then draw that many cards. Then if there are
//! seven or more cards in your graveyard, create a 3/2 red and white
//! Spirit creature token." The 'any number' discard prompt with
//! follow-on draw equal to count isn't in the catalog; the
//! graveyard-conditional token-create IS expressible. We GAP the
//! discard/draw and emit the conditional Spirit on graveyard>=7.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illuminate History");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard any number of cards, then draw that many cards. Then if there are seven or more cards in your graveyard, create a 3/2 red and white Spirit creature token.".into(),
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
    // GAP: 'discard any number, then draw that many' — variable-count
    // discard prompt with chained draw not in catalog.
    let gy = script::graveyard_size(state, entry.controller);
    if gy < 7 {
        return Vec::new();
    }
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
