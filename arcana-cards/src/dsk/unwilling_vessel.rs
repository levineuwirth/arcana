//! Unwilling Vessel — `{2}{U}` 3/2 Human Wizard with Vigilance.
//! "Eerie — Whenever an enchantment you control enters and whenever you
//! fully unlock a Room, put a possession counter on this creature."
//! "When this creature dies, create an X/X blue Spirit creature token
//! with flying, where X is the number of counters on this creature."
//!
//! * Vigilance — base keyword (Eerie is reminder text, not a keyword).
//! * The enchantment-ETB half of the Eerie trigger is wired as a
//!   ZoneChange watching enchantments you control entering; it adds a
//!   possession counter to this creature.
//! * GAP (triggered): "whenever you fully unlock a Room" — no Room
//!   trigger condition exists, so that half of the Eerie trigger is
//!   omitted.
//! * The dies trigger mints an X/X flying blue Spirit, with X read at
//!   resolution as the possession-counter count on this creature.
//!   (Only possession counters ever land here, so counting that named
//!   kind matches "the number of counters on this creature.")

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unwilling Vessel");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _possession = reg.interner_mut().intern("possession");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    let enchantment_filter = ObjectFilter::new()
        .with_types(TypeLine::ENCHANTMENT.into())
        .controlled_by(ControllerConstraint::You);
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: enchantment_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: add_possession_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP (triggered): "whenever you fully unlock a Room" — no Room
            // trigger condition variant exists.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_spirit,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_possession_counter(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("possession").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}

fn dies_make_spirit(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let x = reg
        .interner()
        .lookup("possession")
        .map(CounterKind::Named)
        .and_then(|kind| state.objects.get(trig.source).map(|o| o.count_counters(kind)))
        .unwrap_or(0);
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spirit,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(x as i32)),
            toughness: Some(PtValue::Fixed(x as i32)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
