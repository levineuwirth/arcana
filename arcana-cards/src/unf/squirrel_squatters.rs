//! Squirrel Squatters — `{3}{G}{G}` 4/4 Squirrel.
//!
//! Oracle:
//! * When this creature enters, open an Attraction.
//! * Whenever this creature attacks, create a 1/1 green Squirrel creature
//!   token that's tapped and attacking for each Attraction you've visited
//!   this turn.
//!
//! "Open an Attraction" is an Un-set / Attraction-deck mechanic that is not
//! modeled (no Attraction deck or visit-tracking), so `keywords` is empty and
//! both abilities are GAP'd: the ETB has no Attraction-open Effect, and the
//! attack trigger scales by "Attractions you've visited this turn", a count
//! with no script helper. Both trigger frames are emitted so the abilities are
//! recorded.

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
    let name = reg.interner_mut().intern("Squirrel Squatters");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Open an Attraction" is an Attraction-deck mechanic, unmodeled.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: open_attraction,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: squirrels_per_attraction,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn open_attraction(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "open an Attraction" — no Attraction-deck modeling / Effect.
    Vec::new()
}

fn squirrels_per_attraction(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: count is "Attractions you've visited this turn" — no script helper
    // for Attraction visits, so the dynamic token count is uncomputable.
    Vec::new()
}
