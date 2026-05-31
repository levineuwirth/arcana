//! Deadly Embrace — `{3}{B}{B}` sorcery. "Destroy target creature an
//! opponent controls. Then draw a card for each creature that died this
//! turn."
//!
//! The destroy half is expressed; the rider draws a card for each
//! creature that died this turn, but the script helper surface exposes
//! no "creatures that died this turn" counter, so that DYNAMIC amount
//! cannot be computed and the draw is GAP-ed rather than hardcoded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deadly Embrace");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy target creature an opponent controls. Then draw a card for each creature that died this turn.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "draw a card for each creature that died this turn" — no script
    // helper exposes a count of creatures that died this turn, so the
    // dynamic draw amount cannot be computed. Only the destroy is emitted.
    vec![Effect::DestroyPermanent { target: *id }]
}
