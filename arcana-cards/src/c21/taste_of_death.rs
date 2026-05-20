//! Taste of Death — `{4}{B}{B}` sorcery. "Each player sacrifices
//! three creatures of their choice. You create three Food tokens."
//!
//! Food's activated "{2}, {T}, sacrifice: gain 3 life" mana ability is
//! not constructable on a TokenDefinition from this surface; the
//! `TokenDefinition` is emitted with empty abilities. The per-player
//! sacrifice loop is modeled.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Taste of Death");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player sacrifices three creatures of their choice. You create three Food tokens.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Sacrifice { player: p, filter: ObjectFilter::creature(), count: 3 })
        .collect();
    let food = reg.interner().lookup("Food").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: Food activated mana/life-gain ability not constructable on the token.
    for _ in 0..3 {
        effects.push(Effect::CreateToken { controller: entry.controller, token: token.clone() });
    }
    effects
}
