//! Careful Consideration — `{2}{U}{U}` instant. "Target player draws
//! four cards, then discards three cards. If you cast this spell
//! during your main phase, instead that player draws four cards,
//! then discards two cards."
//!
//! GAP: 'cast during your main phase' phase introspection isn't
//! available. We model the base draw-4 / discard-3 shape.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Careful Consideration");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player draws four cards, then discards three cards. If you cast this spell during your main phase, instead that player draws four cards, then discards two cards.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: 'cast during your main phase' phase check.
    vec![
        Effect::DrawCards { player: *p, count: 4 },
        Effect::Discard {
            player: *p,
            count: 3,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
