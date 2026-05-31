//! Tail Swipe — `{G}` instant. "Choose target creature you control and
//! target creature you don't control. If you cast this spell during your
//! main phase, the creature you control gets +1/+1 until end of turn.
//! Then those creatures fight each other."
//!
//! The fight is fully expressible via `Effect::Fight` over the two
//! targets. The conditional "+1/+1 if cast during your main phase" rider
//! is GAP'd: there is no cast-phase predicate available to the resolver.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Tail Swipe");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target creature you control and target creature you don't \
                   control. If you cast this spell during your main phase, the creature \
                   you control gets +1/+1 until end of turn. Then those creatures fight \
                   each other."
                .into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mine = match entry.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    let theirs = match entry.targets.targets.get(1) {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    // GAP: the "+1/+1 if you cast this spell during your main phase" rider is
    // not expressible — no cast-phase predicate is available to the resolver.
    vec![Effect::Fight { a: mine, b: theirs }]
}
