//! Glyph of Destruction — `{R}` instant. "Target blocking Wall you control
//! gets +10/+0 until end of combat. Prevent all damage that would be dealt to
//! it this turn. Destroy it at the beginning of the next end step." No
//! Duration::EndOfCombat, no damage prevention; pump until end of turn + a
//! delayed destruction via `DelayedAction::Sacrifice` is the closest shape,
//! and the prevent-damage rider is GAPped.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glyph of Destruction");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target blocking Wall you control gets +10/+0 until end of combat. Prevent all damage that would be dealt to it this turn. Destroy it at the beginning of the next end step.".into(),
            // GAP: no filter for "blocking" or for Wall-subtype-specific target; using plain target_creature.
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    // GAP: no Duration::EndOfCombat — using EndOfTurn instead. No damage-prevention Effect.
    // GAP: DelayedAction has no Destroy variant — using Sacrifice as the closest end-step removal.
    vec![
        Effect::Pump {
            target: *id,
            power: 10,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
