//! Keeper of the Accord — `{3}{W}` 3/4 Human Soldier.
//! Two end-step catch-up triggers:
//! "At the beginning of each opponent's end step, if that player
//!  controls more creatures than you, create a 1/1 white Soldier."
//! "At the beginning of each opponent's end step, if that player
//!  controls more lands than you, you may search your library for a
//!  basic Plains card, put it onto the battlefield tapped, then shuffle."
//!
//! Both triggers carry a "more X than you" comparison gate for which
//! there is no intervening-if predicate; the comparison is GAP'd (the
//! triggers fire unconditionally). The bodies — token creation and a
//! basic-Plains tutor onto the battlefield tapped — are wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keeper of the Accord");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: intervening-if "that player controls more creatures than you" —
                //      no count-comparison predicate; fires unconditionally.
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: make_soldier,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: intervening-if "that player controls more lands than you" —
                //      no count-comparison predicate; fires unconditionally.
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: fetch_plains,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_soldier(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: soldier,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn fetch_plains(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // "you may search your library for a basic Plains card, put it onto the
    // battlefield tapped, then shuffle."
    let plains = reg.interner().lookup("Plains");
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter {
            types: Some(TypeLine::LAND.into()),
            supertypes: Some(SupertypeSet(SupertypeSet::BASIC)),
            subtypes: plains.map(|p| vec![p]),
            ..ObjectFilter::default()
        },
        tapped: true,
    }]
}
