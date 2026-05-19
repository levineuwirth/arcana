//! Arachnogenesis — `{2}{G}` instant.
//! "Create X 1/2 green Spider creature tokens with reach, where X is the number of creatures attacking you.
//! Prevent all combat damage that would be dealt this turn by non-Spider creatures."
//! GAP: counting attacking creatures, and preventing combat damage from non-Spider creatures, are not expressible.
//! Best effort: create one Spider token (X=1 fallback) and omit damage prevention.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arachnogenesis");
    let _spider = reg.interner_mut().intern("Spider");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create X 1/2 green Spider creature tokens with reach, where X is the number of creatures attacking you. Prevent all combat damage that would be dealt this turn by non-Spider creatures.".into(),
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
    // GAP: counting attacking creatures is not expressible; GAP: damage prevention is not expressible
    let spider = reg.interner().lookup("Spider")
        .expect("Spider interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    let token = TokenDefinition {
        name: spider,
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
