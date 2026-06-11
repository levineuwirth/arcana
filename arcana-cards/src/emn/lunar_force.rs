//! Lunar Force — `{2}{U}` enchantment.
//! "When an opponent casts a spell, sacrifice this enchantment and
//! counter that spell."
//!
//! The opponent `SpellCast` trigger and the self-sacrifice (modeled as
//! a name-filtered `Effect::Sacrifice`) are wired; countering the
//! spell is a GAP — the effect catalog has no counter-spell variant.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lunar Force");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: sac_and_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…sacrifice this enchantment and counter that spell."
fn sac_and_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Self-sacrifice modeled as a sacrifice constrained to enchantments
    // with this card's name (the only battlefield match is this card or
    // another copy of it).
    let nm = reg.interner().lookup("Lunar Force");
    // GAP: "counter that spell" — the effect catalog has no
    // counter-spell variant.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter {
            name: nm,
            types: Some(TypeLine::ENCHANTMENT.into()),
            ..ObjectFilter::default()
        },
        count: 1,
    }]
}
