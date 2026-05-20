//! Compulsive Research — `{2}{U}` sorcery. "Target player draws three
//! cards. Then that player discards two cards unless they discard a
//! land card."
//!
//! GAP: 'discard a land to skip the 2-card discard' branch isn't a
//! catalog primitive. We model draw-3 + base 2-card discard.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Compulsive Research");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target player draws three cards. Then that player discards two cards unless they discard a land card.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: 'discard a land to skip tax' branch.
    vec![
        Effect::DrawCards { player: *p, count: 3 },
        Effect::Discard {
            player: *p,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
