//! Semester's End — `{3}{W}` instant. "Exile any number of target
//! creatures and/or planeswalkers you control. At the beginning of the
//! next end step, return each of them to the battlefield under its
//! owner's control. Each of them enters with an additional +1/+1
//! counter on it if it's a creature and an additional loyalty counter
//! on it if it's a planeswalker." We emit exile + delayed-return per
//! target and GAP the "enters with counter" rider.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Semester's End");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile any number of target creatures and/or planeswalkers you control. At the beginning of the next end step, return each of them to the battlefield under its owner's control. Each of them enters with an additional +1/+1 counter on it if it's a creature and an additional loyalty counter on it if it's a planeswalker.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types_any(TypeLine(
                            TypeLine::CREATURE | TypeLine::PLANESWALKER,
                        ))
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Any,
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
    let mut effects = Vec::new();
    for choice in &entry.targets.targets {
        if let TargetChoice::Object(id) = choice {
            effects.push(Effect::ExilePermanent { target: *id });
            effects.push(Effect::DelayedAction {
                source: *id,
                controller: entry.controller,
                when: DelayedWhen::NextEndStep,
                action: DelayedAction::ReturnFromExileToBattlefield,
            });
            // GAP: cannot attach +1/+1 or loyalty counters to delayed return.
        }
    }
    effects
}
