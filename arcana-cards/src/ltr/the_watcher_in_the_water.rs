//! The Watcher in the Water — `{3}{U}{U}` 9/9 Legendary Kraken.
//! The Watcher in the Water enters tapped with nine stun counters on it.
//! Whenever you draw a card during an opponent's turn, create a 1/1 blue
//! Tentacle creature token.
//! Whenever a Tentacle you control dies, untap up to one target Kraken and put a
//! stun counter on up to one target nonland permanent.
//!
//! Abilities:
//!  - "enters tapped with nine stun counters" — an enters-with-state
//!    replacement (enter tapped + enter with N stun counters); there is no
//!    ETB-with-counters / enters-tapped effect primitive in the demonstrated
//!    surface → GAP'd.
//!  - "draw a card during an opponent's turn → create a 1/1 blue Tentacle"
//!    modeled with a CardDrawn{You} trigger creating the token. The "during an
//!    opponent's turn" restriction cannot be expressed on the trigger condition
//!    → documented partial (the token is created on any of your draws).
//!  - "a Tentacle you control dies → untap up to one target Kraken and put a
//!    stun counter on up to one target nonland permanent" — ZoneChange Tentacle
//!    → graveyard, two UpTo(1) target requirements; Untap the Kraken and
//!    AddCounters(Stun) on the nonland permanent.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: "The Watcher in the Water enters tapped with nine stun counters on it." —
// enters-tapped + enters-with-N-stun-counters is a replacement on entry; no
// ETB-with-counters / enters-tapped effect primitive is available.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Watcher in the Water");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    // Pre-intern the Tentacle token subtype.
    let _tentacle = reg.interner_mut().intern("Tentacle");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        ..Default::default()
    };

    // "a Tentacle you control" filter for the dies trigger.
    let tentacle_filter = script::subtype_filter(reg, "Tentacle")
        .controlled_by(ControllerConstraint::You);
    // "target Kraken" filter — built here where the interner is available.
    let kraken_filter = script::subtype_filter(reg, "Kraken");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Partial: "during an opponent's turn" restriction not expressible
                // on the condition; fires on any of your card draws.
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_tentacle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: tentacle_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: tentacle_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(kraken_filter),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            }),
    )
}

fn make_tentacle(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let tentacle = reg.interner().lookup("Tentacle").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tentacle);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: tentacle,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn tentacle_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Two independent "up to one" targets, in printed order:
    //   [0] = the Kraken to untap, [1] = the nonland permanent to stun.
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::Untap { target: *id });
    }
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.get(1) {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 1,
        });
    }
    effects
}
