//! Match the Odds — `{2}{G}` Sorcery — Lesson.
//! "Create a 1/1 white Ally creature token. Put a +1/+1 counter on it for
//! each creature your opponents control."
//!
//! # GAP: AddCounters count = number of opponent-controlled creatures
//! The token creation is expressible. The dynamic counter count based on
//! opponent board state is not expressible with the catalog.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Match the Odds");
    let _ally = reg.interner_mut().intern("Ally");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 1/1 white Ally creature token. Put a +1/+1 counter on it for each creature your opponents control.".into(),
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
    let ally = reg.interner().lookup("Ally").expect("Ally interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ally);
    let token = TokenDefinition {
        name: ally,
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: +1/+1 counters equal to opponents' creature count — dynamic count not expressible
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
