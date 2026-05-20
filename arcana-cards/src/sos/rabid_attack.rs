//! Rabid Attack — `{1}{B}` instant. "Until end of turn, any number of
//! target creatures you control each get +1/+0 and gain 'When this
//! creature dies, draw a card.'"
//!
//! Any number of your creatures get +1/+0 until end of turn.
//!
//! GAP: granting a triggered ability ("when this creature dies, draw a
//! card") to a target via a spell has no catalog Effect; only the
//! +1/+0 is applied.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rabid Attack");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Until end of turn, any number of target creatures you control each get +1/+0 and gain \"When this creature dies, draw a card.\"".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Any,
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: granting "when this dies, draw a card" trigger to targets not expressible.
    let mut effects = Vec::new();
    for t in &entry.targets.targets {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::Pump {
                target: *id,
                power: 1,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            });
        }
    }
    effects
}
