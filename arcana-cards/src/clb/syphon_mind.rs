//! Syphon Mind — `{3}{B}` sorcery, "Each other player discards a card. You
//! draw a card for each card discarded this way."
//! "Each other player" discard and drawing based on total discards is not
//! fully expressible; the engine has no multi-opponent-iteration effect. Best
//! effort: single opponent discard + single draw.
//!
//! # GAP: each-other-player iteration (no Effect for applying to all opponents)
//! # GAP: draw equal to cards discarded this way (count of successful discards not trackable)

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syphon Mind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each other player discards a card. You draw a card for each card discarded this way.".into(),
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
    // GAP: each-other-player iteration (no Effect for applying to all opponents)
    // GAP: draw equal to cards discarded this way (count of successful discards not trackable)
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![
        Effect::Discard { player: *p, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: entry.controller, count: 1 },
    ]
}
