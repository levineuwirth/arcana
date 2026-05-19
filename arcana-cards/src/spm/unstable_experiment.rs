//! Unstable Experiment — `{1}{U}` instant, "Target player draws a card, then
//! up to one target creature you control connives."
//!
//! GAP: Connive mechanic (draw, discard, conditional +1/+1 counter if nonland
//! discarded) not expressible as a single Effect variant. Best effort: draw a
//! card for the target player only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unstable Experiment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target player draws a card, then up to one target creature you control connives.".into(),
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
    let TargetChoice::Player(target_player) = target else { return Vec::new(); };
    vec![
        Effect::DrawCards { player: *target_player, count: 1 },
        // GAP: connive mechanic not expressible (draw, discard, conditional counter)
    ]
}
