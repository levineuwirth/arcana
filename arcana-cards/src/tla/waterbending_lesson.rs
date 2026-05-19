//! Waterbending Lesson — `{3}{U}` sorcery (Lesson). "Draw three cards. Then
//! discard a card unless you waterbend {2}."
//!
//! # GAP: Waterbend cost mechanic — no Effect or keyword variant for waterbend
//! (tapping artifacts/creatures to help pay costs). Modeled as draw 3 then
//! discard 1 (worst-case path).
//! # GAP: Lesson subtype — TypeLine has no LESSON constant; subtype intern
//! would be needed but no SubtypeSet on a sorcery is shown in the API.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waterbending Lesson");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards. Then discard a card unless you waterbend {2}.".into(),
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
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
        // GAP: "unless you waterbend {2}" optional cost — modeled as unconditional discard
        Effect::Discard {
            player: entry.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
