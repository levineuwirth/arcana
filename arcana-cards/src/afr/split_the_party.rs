//! Split the Party — `{3}{U}{U}` sorcery. "Choose target player.
//! Return half the creatures they control to their owner's hand,
//! rounded up."
//!
//! GAP: "half, rounded up, player chooses which" isn't expressible —
//! bounce all the target player's creatures as a best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Split the Party");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Choose target player. Return half the creatures they control to their owner's hand, rounded up.".into(),
            target_requirements: vec![TargetRequirement::target_player()],
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
    let Some(t) = entry.targets.targets.first() else { return Vec::new(); };
    let p = match t {
        TargetChoice::Player(p) => *p,
        _ => return Vec::new(),
    };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        p,
    );
    let half = ids.len().div_ceil(2);
    // GAP: 'half rounded up, player picks which' — approximate by
    // bouncing the first half.
    ids.into_iter()
        .take(half)
        .map(|id| Effect::ReturnToHand { target: id })
        .collect()
}
