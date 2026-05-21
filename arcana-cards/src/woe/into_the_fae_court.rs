//! Into the Fae Court — `{3}{U}{U}` sorcery. Draw three cards. Create a
//! 1/1 blue Faerie creature token with flying. (Restricted-blocking
//! clause on token not modeled.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Into the Fae Court");
    let _faerie = reg.interner_mut().intern("Faerie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw three cards. Create a 1/1 blue Faerie creature token with flying.".into(),
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
    let faerie = reg
        .interner()
        .lookup("Faerie")
        .expect("Faerie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    let token = TokenDefinition {
        name: faerie,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![
        Effect::DrawCards {
            player: entry.controller,
            count: 3,
        },
        Effect::CreateToken {
            controller: entry.controller,
            token,
        },
    ]
}
