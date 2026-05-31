//! Joust — `{1}{R}` sorcery. "Choose target creature you control and
//! target creature you don't control. The creature you control gets
//! +2/+1 until end of turn if it's a Knight. Then those creatures
//! fight each other."
//!
//! Two creature targets and the fight are expressible. The
//! conditional "gets +2/+1 ... if it's a Knight" requires reading the
//! subtype of a specific target at resolution time, which no script
//! helper supports (subtype filters select board-wide sets, not a
//! single chosen object's type). That conditional pump is GAPed; the
//! fight is emitted faithfully.

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
    let name = reg.interner_mut().intern("Joust");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose target creature you control and target creature you don't control. The creature you control gets +2/+1 until end of turn if it's a Knight. Then those creatures fight each other.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::You),
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

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(mine)) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Object(theirs)) = entry.targets.targets.get(1) else { return Vec::new(); };
    // GAP: cannot conditionally grant +2/+1 only if the chosen
    // creature you control is a Knight — no script helper reads a
    // specific target's subtype at resolution time. The fight is
    // emitted unconditionally.
    vec![Effect::Fight { a: *mine, b: *theirs }]
}
