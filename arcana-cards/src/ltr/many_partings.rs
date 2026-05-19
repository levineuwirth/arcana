//! Many Partings — `{G}` sorcery. Search your library for a basic land card,
//! reveal it, put it into your hand, then shuffle. Create a Food token.
//!
//! GAP: Food token has a "sacrifice for 2 life" activated ability; the engine
//! has no `activated_abilities` field on `TokenDefinition`, so the Food token
//! is created as a bare artifact with Food subtype (reminder text not wired).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Many Partings");
    let _food = reg.interner_mut().intern("Food");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for a basic land card, reveal it, put it into your hand, then shuffle. Create a Food token.".into(),
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
    let food = reg.interner().lookup("Food").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    let token = TokenDefinition {
        name: food,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            reveal: true,
        },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
