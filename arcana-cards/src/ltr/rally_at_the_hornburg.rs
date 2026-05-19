//! Rally at the Hornburg — `{1}{R}` sorcery. "Create two 1/1 white Human
//! Soldier creature tokens. Humans you control gain haste until end of turn."
//!
//! # GAP: "Humans you control gain haste" requires ForEach over creatures with
//! Human subtype, which is not filterable via the catalog. We create the tokens
//! and note the haste-grant gap.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rally at the Hornburg");
    let _human = reg.interner_mut().intern("Human");
    let _soldier = reg.interner_mut().intern("Soldier");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create two 1/1 white Human Soldier creature tokens. Humans you control gain haste until end of turn.".into(),
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
    let human = reg.interner().lookup("Human").expect("Human interned during register()");
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    // GAP: grant haste to all Humans you control (subtype filter not in catalog)
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
