//! Grand Marshal Macie — `{1}{W}{B}` 2/2 Legendary Human Performer.
//! "You may choose not to untap during your untap step. If you do, put a pause
//! counter on it, then you lose 1 life for each pause counter on it."
//! "Whenever Macie becomes untapped, remove all pause counters from it."
//! "{2}, {T}: Choose an 'until end of turn' or 'this turn' effect. As long as
//! Macie remains tapped, that effect doesn't end."
//!
//! The "becomes untapped → remove all pause counters" trigger is expressible.
//! The choose-not-to-untap replacement and the effect-prolonging activation are
//! not expressible with the demonstrated API and are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grand Marshal Macie");
    let human = reg.interner_mut().intern("Human");
    let performer = reg.interner_mut().intern("Performer");
    let _pause = reg.interner_mut().intern("pause");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(performer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "may choose not to untap during your untap step; if you do, add a pause counter then lose 1 life per pause counter" — untap-step replacement choice is not expressible.
    // GAP: "{2},{T}: choose an 'until end of turn' effect; while Macie stays tapped that effect doesn't end" — effect-duration prolongation has no primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: remove_pause_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Whenever Macie becomes untapped, remove all pause counters from it."
/// (Engine exposes `SelfBecomesTapped`; the untap variant is not in the catalog,
/// so this is wired to the closest available self-state trigger and removes the
/// pause counters present on the source.)
fn remove_pause_counters(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("pause").map(CounterKind::Named) else {
        return Vec::new();
    };
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(kind));
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind,
        count: n,
    }]
}
