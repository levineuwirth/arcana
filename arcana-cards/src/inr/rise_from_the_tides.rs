//! Rise from the Tides — `{5}{U}` sorcery. "Create a tapped 2/2 black Zombie
//! creature token for each instant and sorcery card in your graveyard."
//!
//! Uses `script::graveyard_matching` with an instant-or-sorcery filter to count
//! qualifying cards, then creates that many Zombie tokens.

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
    let name = reg.interner_mut().intern("Rise from the Tides");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a tapped 2/2 black Zombie creature token for each instant and sorcery card in your graveyard.".into(),
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
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let n = script::graveyard_matching(state, &filter, entry.controller, entry.controller);
    if n == 0 {
        return Vec::new();
    }
    let zombie = reg.interner().lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n).map(|_| Effect::CreateToken { controller: entry.controller, token: token.clone() }).collect()
}
