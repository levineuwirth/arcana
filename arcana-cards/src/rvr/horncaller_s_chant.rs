//! Horncaller's Chant — `{7}{G}` sorcery. Create a 4/4 green Rhino
//! creature token with trample, then populate. (Populate not modeled.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horncaller's Chant");
    let _rhino = reg.interner_mut().intern("Rhino");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 4/4 green Rhino creature token with trample, then populate.".into(),
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
    let rhino = reg
        .interner()
        .lookup("Rhino")
        .expect("Rhino interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    let token = TokenDefinition {
        name: rhino,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        abilities: vec![],
    };
    // GAP: "populate" not modeled (would copy a creature token you control).
    vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }]
}
