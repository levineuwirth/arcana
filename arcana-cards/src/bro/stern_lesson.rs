//! Stern Lesson — `{2}{U}` instant, "Draw two cards, then discard a card. Create
//! a tapped colorless Powerstone artifact token with '{T}: Add {C}. Spend this
//! mana only to cast artifact spells or activate abilities of artifacts.'"
//!
//! # GAP: Powerstone token's restricted activated mana ability not expressible
//!        via TokenDefinition.abilities (activated ability syntax not in catalog)

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stern Lesson");
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards, then discard a card. Create a tapped colorless Powerstone artifact token with '{T}: Add {C}. Spend this mana only to cast artifact spells or activate abilities of artifacts.'".into(),
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
    let powerstone = reg.interner().lookup("Powerstone").expect("Powerstone interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(powerstone);
    // GAP: Powerstone token's restricted activated mana ability not expressible
    let token = TokenDefinition {
        name: powerstone,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
