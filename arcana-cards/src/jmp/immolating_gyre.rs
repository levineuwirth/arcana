//! Immolating Gyre — `{4}{R}{R}` sorcery. "Immolating Gyre deals X damage to
//! each creature and planeswalker you don't control, where X is the number of
//! instant and sorcery cards in your graveyard."
//!
//! Note: planeswalkers are not a TypeLine in the current engine; targeting
//! creatures you don't control. GAP: damage to planeswalkers opponent controls.
//! GAP: graveyard_matching with instant+sorcery type filter (no
//! ObjectFilter::with_types_any for graveyard_matching).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;
use arcana_core::objects::NULL_OBJECT_ID;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Immolating Gyre");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Immolating Gyre deals X damage to each creature and planeswalker you don't control, where X is the number of instant and sorcery cards in your graveyard.".into(),
                target_requirements: vec![],
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
    // GAP: count instant+sorcery cards in graveyard (graveyard_matching needs type filter)
    // GAP: also deal damage to planeswalkers
    let x = script::graveyard_size(state, entry.controller);
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: x,
        }),
    }]
}
