//! Rhino's Rampage — `{R/G}` sorcery. "Target creature you control
//! gets +1/+0 until end of turn. It fights target creature an
//! opponent controls. When excess damage is dealt to the creature an
//! opponent controls this way, destroy up to one target noncreature
//! artifact with mana value 3 or less." We can express the pump and
//! the fight; the excess-damage trigger and chained artifact target
//! aren't expressible as a single-ability spell — GAP that rider.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Rhino's Rampage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    // GAP: 'when excess damage is dealt, destroy artifact' chained-target rider.
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Target creature you control gets +1/+0 until end of turn. It fights target creature an opponent controls. When excess damage is dealt to the creature an opponent controls this way, destroy up to one target noncreature artifact with mana value 3 or less.".into(),
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
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
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

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ts = &entry.targets.targets;
    if ts.len() < 2 { return Vec::new(); }
    let (TargetChoice::Object(a), TargetChoice::Object(b)) = (&ts[0], &ts[1]) else {
        return Vec::new();
    };
    vec![
        Effect::Pump {
            target: *a,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Fight { a: *a, b: *b },
    ]
}
