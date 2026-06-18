//! The Knight of Weeks — `{W}` 1/1 Legendary Human Knight.
//! Lifelink, vigilance, first strike.
//! At the beginning of your combat step and whenever another Knight enters the
//! battlefield under your control, put a day counter on The Knight of Weeks.
//! The Knight of Weeks gets +7/+7 for every seven day counters on it.
//!
//! The three keywords are base characteristics. The compound trigger line is
//! decomposed into two `TriggeredAbilityDef`s (combat-step-begins and
//! another-Knight-enters) that both put a "day" counter on this creature. The
//! final static "+7/+7 for every seven day counters" is a pure continuous
//! self-pump with no trigger or cost to decompose — GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: static "The Knight of Weeks gets +7/+7 for every seven day counters on
// it." — a continuous self-pump scaling on counters; pure static, no
// trigger/cost to express here.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Knight of Weeks");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let knight_subtype = reg.interner_mut().intern("Knight");
    // Pre-intern the named "day" counter so the resolver's lookup succeeds.
    let _day = reg.interner_mut().intern("day");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let knight_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![knight_subtype]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Lifelink,
            KeywordAbility::Vigilance,
            KeywordAbility::FirstStrike,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_day_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: knight_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: add_day_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_day_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let day = reg.interner().lookup("day").map(CounterKind::Named);
    let Some(kind) = day else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}
