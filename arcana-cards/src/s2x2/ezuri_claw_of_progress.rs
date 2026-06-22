//! Ezuri, Claw of Progress — `{2}{G}{U}` 3/3 Legendary Phyrexian Elf
//! Warrior.
//! "Whenever a creature you control with power 2 or less enters, you get an
//!  experience counter.
//!  At the beginning of combat on your turn, put X +1/+1 counters on
//!  another target creature you control, where X is the number of
//!  experience counters you have."
//!
//! Both trigger conditions are wired, but both payloads are GAPs.
//! GAP: "you get an experience counter" — experience counters live on the
//!      PLAYER and no Effect places a counter on a player.
//! GAP: "put X +1/+1 counters ... where X is the number of experience
//!      counters you have" — no script helper reads a player's experience
//!      counters, so X is uncomputable; the whole effect is GAP'd (a literal
//!      would be a materially wrong card).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ezuri, Claw of Progress");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_power(2),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_experience,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: counters_for_experience,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn gain_experience(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you get an experience counter" — experience counters live on
    // the player; no Effect places a counter on a player.
    Vec::new()
}

fn counters_for_experience(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = "number of experience counters you have" is uncomputable
    // (no script accessor for a player's experience counters), so the
    // whole counter-placement effect is GAP'd rather than emitting a
    // wrong literal amount.
    Vec::new()
}
