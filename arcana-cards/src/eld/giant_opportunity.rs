//! Giant Opportunity — `{2}{G}` sorcery. "You may sacrifice two Foods. If you do, create a 7/7
//! green Giant creature token. Otherwise, create three Food tokens."
//! GAP: optional sacrifice-two-Foods cost choice + conditional token branch not expressible.
//! Best effort: create three Food tokens (the non-sacrifice branch).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Opportunity");
    let _food = reg.interner_mut().intern("Food");
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
                text: "You may sacrifice two Foods. If you do, create a 7/7 green Giant creature token. Otherwise, create three Food tokens.".into(),
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
    let food_token = TokenDefinition {
        name: food,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        // GAP: Food token activated ability ("{2},{T},Sacrifice: gain 3 life") not modeled
        abilities: vec![],
    };
    // GAP: optional sacrifice-two-Foods → 7/7 Giant branch; defaulting to three Food tokens
    vec![
        Effect::CreateToken { controller: entry.controller, token: food_token.clone() },
        Effect::CreateToken { controller: entry.controller, token: food_token.clone() },
        Effect::CreateToken { controller: entry.controller, token: food_token },
    ]
}
