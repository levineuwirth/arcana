//! Chart a Course — `{1}{U}` sorcery, "Draw two cards. Then discard a card
//! unless you attacked this turn."
//!
//! GAP: "unless you attacked this turn" conditional is not in the Effect
//! catalog. Best-effort: draw 2 + discard 1 unconditionally.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chart a Course");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. Then discard a card unless you attacked this turn.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "unless you attacked this turn" conditional not in Effect catalog
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}
