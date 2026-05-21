//! Fateful Handoff — `{3}{B}` sorcery. "Draw cards equal to the mana
//! value of target artifact or creature you control. An opponent gains
//! control of that permanent." Reading a permanent's CMC and a permanent
//! change-of-control to an opponent are catalog primitives.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fateful Handoff");
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
                // GAP: cannot read a permanent's mana value to count draws (no script::cmc_of helper).
                // GAP: ChangeControl requires picking the new controller; cannot target an opponent here generically.
                text: "Draw cards equal to the mana value of target artifact or creature you control. An opponent gains control of that permanent.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                            .controlled_by(ControllerConstraint::You),
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
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cards-drawn computation needs CMC of the target — not in script::
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let opponents = script::opponents(state, entry.controller);
    let Some(opp) = opponents.into_iter().next() else { return Vec::new(); };
    vec![Effect::ChangeControl { target: *id, new_controller: opp }]
}
