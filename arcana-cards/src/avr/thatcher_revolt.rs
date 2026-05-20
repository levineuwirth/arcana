//! Thatcher Revolt — `{2}{R}` sorcery. "Create three 1/1 red Human creature
//! tokens with haste. Sacrifice those tokens at the beginning of the next
//! end step."
//!
//! GAP: DelayedAction requires a known object id at resolve time; newly
//! created tokens do not have ids available during resolution. The token
//! creation is expressible; the sacrifice riders are not (token ids unknown).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thatcher Revolt");
    let _human = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create three 1/1 red Human creature tokens with haste. Sacrifice those tokens at the beginning of the next end step.".into(),
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
    let human = reg.interner().lookup("Human")
        .expect("Human interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };
    // GAP: sacrifice-at-end-step rider requires known token ids, which are
    // unavailable at resolution time
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
