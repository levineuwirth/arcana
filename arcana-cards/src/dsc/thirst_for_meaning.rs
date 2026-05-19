//! Thirst for Meaning — `{2}{U}` instant, "Draw three cards, then discard two
//! cards unless you discard an enchantment card."
//! GAP: conditional discard (discard fewer if you discard an enchantment) not
//! expressible; best effort draws 3 and discards 2.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thirst for Meaning");
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
                text: "Draw three cards, then discard two cards unless you discard an enchantment card.".into(),
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
    // GAP: conditional discard type (unless enchantment) not expressible
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
        Effect::Discard { player: entry.controller, count: 2, choice: DiscardChoice::ControllerChooses },
    ]
}
