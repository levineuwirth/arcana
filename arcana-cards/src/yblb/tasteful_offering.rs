//! Tasteful Offering — `{1}{B}` sorcery.
//! "You get a two-time boon with 'Whenever you sacrifice one or more permanents,
//! seek a nonland card.' Create a Food token."
//! GAP: 'two-time boon' (limited-use triggered ability) not in catalog;
//! GAP: Seek mechanic not in catalog;
//! GAP: Food token has activated ability not expressible in TokenDefinition;
//! emitting a plain Food artifact token only.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tasteful Offering");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You get a two-time boon with \"Whenever you sacrifice one or more permanents, seek a nonland card.\" Create a Food token.".into(),
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
    let food = reg.interner().lookup("Food").expect("Food interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    // GAP: two-time boon triggered ability not in catalog
    // GAP: Seek mechanic not in catalog
    // GAP: Food token activated ability not expressible in TokenDefinition
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
