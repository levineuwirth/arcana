//! Tectonic Hazard — `{R}` sorcery, "Tectonic Hazard deals 1 damage to each
//! opponent and each creature they control."
//!
//! GAP: dealing damage to each opponent (not all players, only opponents)
//! requires iterating over opponent PlayerIds, which is not directly
//! available. script::ids_matching can get opponent creatures for ForEach,
//! but dealing 1 to each opponent player has no helper. Best-effort:
//! ForEach on opponent creatures for 1 damage each; opponent player hits
//! are a GAP.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tectonic Hazard");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tectonic Hazard deals 1 damage to each opponent and each creature they control.".into(),
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
    let opponent_creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        entry.controller,
    );
    // GAP: dealing 1 damage to each opponent (player) not expressible without opponent PlayerIds
    vec![Effect::ForEach {
        targets: opponent_creatures,
        effect: Box::new(Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(arcana_core::objects::NULL_OBJECT_ID),
            amount: 1,
        }),
    }]
}
