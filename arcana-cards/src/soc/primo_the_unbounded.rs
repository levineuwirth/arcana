//! Primo, the Unbounded — `{X}{G}{G}{U}` 0/0 Legendary Fractal Wolf with Trample.
//! Primo enters with twice X +1/+1 counters on it.
//! Whenever one or more creatures you control with base power 0 deal combat
//! damage to a player, create a 0/0 green and blue Fractal creature token.
//! Put a number of +1/+1 counters on it equal to the damage dealt.
//!
//! "Enters with twice X +1/+1 counters" is GAP'd — there is no accessor for
//! the spell's X value inside a trigger fn. The combat-damage trigger mints
//! the Fractal token; the "put +1/+1 counters equal to the damage dealt on
//! it" rider is GAP'd because a freshly-created token's id can't be
//! referenced for a follow-up AddCounters.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Primo, the Unbounded");
    let fractal = reg.interner_mut().intern("Fractal");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    subtypes.0.insert(wolf);

    // GAP: "Primo enters with twice X +1/+1 counters on it" — no X accessor
    // available in a trigger fn.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_max_power(0),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: make_fractal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_fractal(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let fractal = reg.interner().lookup("Fractal").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    // GAP: "put +1/+1 counters equal to the damage dealt on it" — the new
    // token's id can't be referenced for a follow-up AddCounters.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: fractal,
            colors: ColorSet::green() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
