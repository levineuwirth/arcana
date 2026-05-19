//! Unified Front — `{3}{W}` sorcery, "Converge — Create a 1/1 white Kor Ally
//! creature token for each color of mana spent to cast this spell."
//!
//! GAP: Converge (X = colors of mana spent to cast) is not expressible.
//! Partial: create one token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unified Front");
    let _kor = reg.interner_mut().intern("Kor");
    let _ally = reg.interner_mut().intern("Ally");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Converge — Create a 1/1 white Kor Ally creature token for each color of mana spent to cast this spell.".into(),
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
    // GAP: Converge — X = number of colors of mana spent to cast
    let kor = reg.interner().lookup("Kor").expect("Kor interned during register()");
    let ally = reg.interner().lookup("Ally").expect("Ally interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(ally);
    let token = TokenDefinition {
        name: kor,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
