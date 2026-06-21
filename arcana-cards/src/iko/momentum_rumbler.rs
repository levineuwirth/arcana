//! Momentum Rumbler — `{3}{R}` 3/3 Dinosaur.
//! "Whenever this creature attacks, if it doesn't have first strike, put a first
//!  strike counter on it."
//! "Whenever this creature attacks, if it has first strike, it gains double
//!  strike until end of turn."
//!
//! Both triggers gate on a keyword-presence intervening-if ("if it doesn't /
//! does have first strike"), which has no expressible condition (no
//! source-has-keyword predicate). Firing either effect unconditionally would be
//! materially wrong (the two are mutually exclusive), so both effects are GAP'd
//! while the SelfAttacks triggers are recorded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Momentum Rumbler");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: add_first_strike_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: gain_double_strike,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_first_strike_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it doesn't have first strike, put a first strike counter on it" —
    // no expressible source-has-keyword intervening-if; effect omitted to avoid
    // unconditional firing.
    Vec::new()
}

fn gain_double_strike(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it has first strike, it gains double strike until end of turn" —
    // no expressible source-has-keyword intervening-if; effect omitted to avoid
    // unconditional firing.
    Vec::new()
}
