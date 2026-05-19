//! Cerebral Vortex — `{1}{U}{R}` instant. "Target player draws two cards,
//! then Cerebral Vortex deals damage to that player equal to the number of
//! cards they've drawn this turn."
//!
//! GAP: damage amount dependent on cards-drawn-this-turn counter (runtime
//! state query not in catalog); the draw-two portion is expressed but the
//! conditional damage requires a state-based integer read that is not
//! available via any catalog Effect variant.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cerebral Vortex");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player draws two cards, then Cerebral Vortex deals damage to that player equal to the number of cards they've drawn this turn.".into(),
                target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: damage equal to cards drawn this turn by target player (no runtime
    // draw-counter query in catalog); emitting draw only.
    vec![
        Effect::DrawCards { player: *p, count: 2 },
    ]
}
