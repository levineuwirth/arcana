//! Charge Across the Araba — `{4}{W}` instant (Arcane). Sweep —
//! "Return any number of Plains you control to their owner's hand.
//! Creatures you control get +1/+1 until end of turn for each Plains
//! returned this way."
//!
//! Modeled as: return every Plains you control (the Sweep "any number"
//! choice collapses to all, since more returned = bigger pump), then
//! pump each creature you control +N/+N where N is the number of Plains
//! returned. The two halves are coupled, so we compute the count first.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charge Across the Araba");
    let _plains = reg.interner_mut().intern("Plains");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Sweep — Return any number of Plains you control to their owner's hand. \
                   Creatures you control get +1/+1 until end of turn for each Plains returned this way."
                .into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // Plains you control.
    let plains_filter = ObjectFilter {
        name: reg.interner().lookup("Plains"),
        ..ObjectFilter::default()
    }
    .controlled_by(ControllerConstraint::You);
    let plains_ids = script::ids_matching(state, &plains_filter, entry.controller);
    let n = plains_ids.len() as i32;

    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        entry.controller,
    );

    let mut effects: Vec<Effect> = Vec::new();
    // Return all Plains to hand.
    effects.push(Effect::ForEach {
        targets: plains_ids,
        effect: Box::new(Effect::ReturnToHand {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    });
    // Pump each creature you control +N/+N.
    if n > 0 {
        effects.push(Effect::ForEach {
            targets: creature_ids,
            effect: Box::new(Effect::Pump {
                target: arcana_core::objects::NULL_OBJECT_ID,
                power: n,
                toughness: n,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        });
    }
    effects
}
