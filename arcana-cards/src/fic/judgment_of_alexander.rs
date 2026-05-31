//! Judgment of Alexander — `{2}{W}` instant. "Prevent all damage that
//! would be dealt to you this turn by sources your opponents control.
//! Whenever damage from a creature is prevented this way, each commander
//! creature you control deals damage equal to its power to that creature."
//!
//! The prevention shield is expressed with `Effect::PreventDamageFrom`
//! (source = permanents your opponents control, target = you). The
//! prevented-damage rider (each commander creature you control deals
//! damage to the prevented source) is a delayed reflexive trigger keyed
//! on a prevention event, which the catalog cannot express.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Judgment of Alexander");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Prevent all damage that would be dealt to you this turn by sources your opponents control. Whenever damage from a creature is prevented this way, each commander creature you control deals damage equal to its power to that creature.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the prevented-damage rider (a reflexive trigger that fires when
    // damage is prevented this way, having each commander creature you
    // control deal damage equal to its power to the prevented source) has
    // no catalog primitive. The prevention shield itself is expressible.
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::permanent().controlled_by(ControllerConstraint::Opponent),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
