//! Drive to Work — `{W}` instant. "Exile target creature or Vehicle.
//! Return that card to the battlefield under its owner's control at
//! the beginning of the next end step."
//! GAP: TargetFilter cannot filter for "creature or Vehicle" (Vehicle
//! is a subtype of Artifact, no combined type filter). Uses
//! TargetFilter::Permanent for any permanent. The return is modeled
//! as DelayedAction{NextEndStep, ReturnToHand} — note: ReturnToBattlefield
//! is not a DelayedAction variant; best-effort uses ReturnToHand.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drive to Work");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile target creature or Vehicle. Return that card to the battlefield under its owner's control at the beginning of the next end step.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: entry.source,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnToHand,
        },
    ]
}
