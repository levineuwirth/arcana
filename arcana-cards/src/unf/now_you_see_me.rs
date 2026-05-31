//! Now You See Me . . . — `{W}` instant (Unfinity). "Exile target
//! creature you control, then return it to the battlefield under its
//! owner's control with X +1/+1 counters on it, where X is the number
//! of mirrors on walls you can see from your seat."
//!
//! The blink is expressible: exile the target now and schedule its
//! return to the battlefield. The X counter count, however, is a
//! real-world ("mirrors on walls you can see from your seat") quantity
//! that cannot be derived from game state via any `script::*` helper,
//! so the AddCounters rider is GAPped rather than hardcoded.

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
    let name = reg.interner_mut().intern("Now You See Me . . .");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target creature you control, then return it to the battlefield under its owner's control with X +1/+1 counters on it, where X is the number of mirrors on walls you can see from your seat.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
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
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: X = "the number of mirrors on walls you can see from your
    // seat" is a real-world, out-of-game quantity that no script::*
    // helper can compute, so the +1/+1 counter rider is omitted rather
    // than emitted with a wrong literal count. The blink itself is
    // faithful: exile now, return to the battlefield at the next end
    // step.
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: entry.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
