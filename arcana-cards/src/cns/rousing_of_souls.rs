//! Rousing of Souls — `{2}{W}` sorcery. "Parley — Each player reveals the top card of their
//! library. For each nonland card revealed this way, you create a 1/1 white Spirit creature token
//! with flying. Then each player draws a card."
//! GAP: Parley mechanic (each player reveals top card, count nonlands) not expressible.
//! Best effort: create one Spirit token and each player draws; parley count is GAP.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rousing of Souls");
    let _spirit = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Parley — Each player reveals the top card of their library. For each nonland card revealed this way, you create a 1/1 white Spirit creature token with flying. Then each player draws a card.".into(),
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
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // GAP: Parley — reveal top of each library, count nonlands, create that many tokens; each player draws
    // Best effort: one token + controller draws
    vec![
        Effect::CreateToken { controller: entry.controller, token },
        Effect::DrawCards { player: entry.controller, count: 1 },
    ]
}
