//! Clowning Around — `{1}{W}` sorcery. "Create two 1/1 white Clown Robot
//! artifact creature tokens, then roll a six-sided die. If the result is
//! equal to or less than the number of Robots you control, create a 1/1 white
//! Clown Robot artifact creature token."
//!
//! GAP: die-rolling mechanic and conditional token creation based on Robot
//! count are not expressible with the catalog. Two base tokens are created.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clowning Around");
    let _clown = reg.interner_mut().intern("Clown");
    let _robot = reg.interner_mut().intern("Robot");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create two 1/1 white Clown Robot artifact creature tokens, then roll a six-sided die. If the result is equal to or less than the number of Robots you control, create a 1/1 white Clown Robot artifact creature token.".into(),
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
    let clown = reg.interner().lookup("Clown").expect("Clown interned during register()");
    let robot = reg.interner().lookup("Robot").expect("Robot interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clown);
    subtypes.0.insert(robot);
    let token = TokenDefinition {
        name: clown,
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: die roll and conditional third token not supported
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
