//! Candlekeep Inspiration — `{4}{U}` sorcery, "Create a 0/0 blue Wizard creature
//! token. Put X +1/+1 counters on it, where X is the number of instant and
//! sorcery cards and cards with adventure you own in exile and your graveyard."
//!
//! GAP: X = count of instants/sorceries/adventures in exile and graveyard
//! (dynamic counter count based on graveyard/exile state); 0/0 base P/T with
//! dynamic counters on creation.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Candlekeep Inspiration");
    let _wizard = reg.interner_mut().intern("Wizard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 0/0 blue Wizard creature token. Put X +1/+1 counters on it, where X is the number of instant and sorcery cards and cards with adventure you own in exile and your graveyard.".into(),
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
    // GAP: dynamic X = count of instants/sorceries/adventures in exile+graveyard;
    // AddCounters on token by that dynamic amount
    let wizard = reg.interner().lookup("Wizard")
        .expect("Wizard interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let token = TokenDefinition {
        name: wizard,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
