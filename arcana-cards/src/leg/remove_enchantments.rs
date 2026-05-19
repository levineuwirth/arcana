//! Remove Enchantments — `{W}` instant, "Return to your hand all enchantments
//! you both own and control, all Auras you own attached to permanents you
//! control, and all Auras you own attached to attacking creatures your
//! opponents control. Then destroy all other enchantments you control, all
//! other Auras attached to permanents you control, and all other Auras
//! attached to attacking creatures your opponents control."
//!
//! # GAP
//! - "Attached to permanents / attacking creatures" filter is not available.
//! - Owner-vs-controller distinction in filtering is not available.
//! - "Attacking creatures" state filter is not available.
//! Best effort: return all enchantments you control (both owned and not) to
//! hand. Owner-check and attached-to constraints are omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Remove Enchantments");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return to your hand all enchantments you both own and control, all Auras you own attached to permanents you control, and all Auras you own attached to attacking creatures your opponents control. Then destroy all other enchantments you control, all other Auras attached to permanents you control, and all other Auras attached to attacking creatures your opponents control.".into(),
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
    // GAP: owner filter, attached-to filter, and attacking-creature filter not available
    let ids = script::ids_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ENCHANTMENT.into())
            .controlled_by(ControllerConstraint::You),
        entry.controller,
    );
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnToHand {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
