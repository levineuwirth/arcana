//! Archpriest of Iona — `{W}` */2 Creature — Human Cleric (white).
//!
//! * "Archpriest of Iona's power is equal to the number of creatures in your
//!   party." — a power-defining CDA. GAP: party is "up to one each of
//!   Cleric, Rogue, Warrior, Wizard you control" (0..=4, each subtype caps
//!   at 1). `self_pt_from_match` is a single symmetric filter that cannot
//!   enforce the one-per-subtype cap across four distinct subtypes, and
//!   `self_pt_cda`'s compute has no registry to resolve those subtype names,
//!   so neither constructor expresses it. Recorded as */2 via PtValue::Star.
//! * "At the beginning of combat on your turn, if you have a full party,
//!   target creature gets +1/+1 and gains flying until end of turn." —
//!   PhaseBegins(Combat, You) trigger pumping a target creature +1/+1 with
//!   Flying. GAP: the intervening-if "if you have a full party" has no
//!   condition predicate, so it is left as None.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archpriest of Iona");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    // GAP: power CDA (creatures in your party) — party's one-per-subtype cap
    // across four subtypes is inexpressible with the self-CDA constructors;
    // recorded as */2 via Star.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::PhaseBegins {
            phase: Phase::Combat,
            whose: ControllerConstraint::You,
        },
        // GAP: intervening-if "if you have a full party" has no predicate.
        intervening_if: None,
        effect: pump_flying,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![TargetRequirement::target_creature()],
    }))
}

fn pump_flying(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Flying],
    }]
}
