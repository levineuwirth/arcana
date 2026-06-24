//! Thirst for Discovery — `{2}{U}` instant. "Draw three cards. Then
//! discard two cards unless you discard a basic land card."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thirst for Discovery");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw three cards. Then discard two cards unless you discard a basic land card.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "unless you discard a basic land card" payment can't constrain
    // the discarded card to a basic-land type (OptionalPaymentKind::Discard is a
    // bare count); emit the draw plus the baseline two-card discard.
    vec![
        Effect::DrawCards {
            player: entry.controller,
            count: 3,
        },
        Effect::Discard {
            player: entry.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
