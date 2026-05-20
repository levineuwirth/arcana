//! Tidal Wave — `{2}{U}` instant, "Create a 5/5 blue Wall creature token
//! with defender. Sacrifice it at the beginning of the next end step."
//!
//! GAP: DelayedAction requires a known ObjectId but token ids are not
//! available at creation time. Only the CreateToken effect is modeled;
//! the delayed sacrifice is omitted.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tidal Wave");
    let _wall = reg.interner_mut().intern("Wall");
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
                text: "Create a 5/5 blue Wall creature token with defender. Sacrifice it at the beginning of the next end step.".into(),
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
    let wall = reg.interner().lookup("Wall").expect("Wall interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);
    let token = TokenDefinition {
        name: wall,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Defender],
        abilities: vec![],
    };
    // GAP: cannot schedule DelayedAction::Sacrifice on a freshly-created token
    // (token ObjectId unknown at resolve time).
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
