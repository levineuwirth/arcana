//! Miles "Tails" Prower — `{1}{W}{U}` 3/1 Legendary Fox Artificer.
//! Flash, Flying.
//! "Whenever a Vehicle you control enters, draw a card if it has flying.
//! Otherwise, put a flying counter on it." — modeled as: put a flying
//! counter on the entering Vehicle (the conditional draw branch is a GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miles \"Tails\" Prower");
    let fox = reg.interner_mut().intern("Fox");
    let artificer = reg.interner_mut().intern("Artificer");
    let _flying_counter = reg.interner_mut().intern("flying");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: arcana_core::script::subtype_filter(reg, "Vehicle")
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: vehicle_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn vehicle_enters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "draw a card if it has flying, otherwise..." conditional on the
    // entering object's flying status is not expressible with available
    // conditions; emit the default branch (put a flying counter on it).
    let Some(id) = trig.entering_object() else { return Vec::new(); };
    let Some(flying) = reg.interner().lookup("flying").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: id,
        kind: flying,
        count: 1,
    }]
}
