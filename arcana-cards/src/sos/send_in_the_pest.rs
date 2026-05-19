//! Send in the Pest — `{1}{B}` sorcery, "Each opponent discards a card. You
//! create a 1/1 black and green Pest creature token with 'Whenever this token
//! attacks, you gain 1 life.'"
//!
//! GAP: "each opponent discards" — no ForEach-over-players variant; Pest token
//! has an attack-triggered ability that cannot be expressed in TokenDefinition
//! (abilities: vec![] only).

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Send in the Pest");
    let _pest = reg.interner_mut().intern("Pest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each opponent discards a card. You create a 1/1 black and green Pest creature token with \"Whenever this token attacks, you gain 1 life.\"".into(),
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
    let pest = reg.interner().lookup("Pest")
        .expect("Pest interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest);
    let token = TokenDefinition {
        name: pest,
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: "Whenever this token attacks, you gain 1 life" triggered ability
        // not expressible in TokenDefinition
        abilities: vec![],
    };
    // GAP: each opponent discards a card (no ForEach-over-players variant)
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
