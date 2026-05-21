//! Thoughtseize — `{B}` sorcery. "Target player reveals their hand.
//! You choose a nonland card from it. That player discards that card.
//! You lose 2 life." Map to a single targeted discard
//! (controller-chooses) + 2 life loss for the caster. The 'reveals
//! their hand and you pick' is the engine's intent for
//! ControllerChooses but the chooser is the OPPONENT-of-the-target's
//! controller, not the caster; without a 'caster chooses' discard
//! variant we use the closest available.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thoughtseize");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: 'reveal hand, caster picks a nonland card' — discard choice is target-controller scoped, no caster-picks-from-opponent discard variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player reveals their hand. You choose a nonland card from it. That player discards that card. You lose 2 life.".into(),
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
    vec![
        Effect::Discard {
            player: *p,
            count: 1,
            choice: DiscardChoice::OpponentChooses,
        },
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
