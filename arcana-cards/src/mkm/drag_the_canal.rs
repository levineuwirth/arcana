//! Drag the Canal — `{U}{B}` instant, "Create a 2/2 white-blue Detective creature
//! token. If a creature died this turn, you may pay {1}. If you do, gain 2 life,
//! surveil 2, and investigate."
//!
//! GAP: conditional triggered by "if a creature died this turn" (no death-this-turn
//! state query); optional payment branch; Investigate effect not in catalog.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drag the Canal");
    let _detective = reg.interner_mut().intern("Detective");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 2/2 white-blue Detective creature token. If a creature died this turn, you may pay {1}. If you do, gain 2 life, surveil 2, and investigate.".into(),
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
    let detective = reg.interner().lookup("Detective")
        .expect("Detective interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(detective);
    let token = TokenDefinition {
        name: detective,
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: conditional "if a creature died this turn" check + optional {1}
    // payment + GainLife(2) + Surveil(2) + Investigate
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
