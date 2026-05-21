//! Pulse of the Tangle — `{1}{G}{G}` sorcery. "Create a 3/3 green
//! Beast creature token. Then if an opponent controls more creatures
//! than you, return Pulse of the Tangle to its owner's hand."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pulse of the Tangle");
    let _beast = reg.interner_mut().intern("Beast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 3/3 green Beast creature token. Then if an opponent controls more creatures than you, return Pulse of the Tangle to its owner's hand.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let beast = reg
        .interner()
        .lookup("Beast")
        .expect("Beast interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let token = TokenDefinition {
        name: beast,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "if an opponent controls more creatures than you, return
    // this spell to hand" — a resolving spell cannot return itself.
    vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }]
}
