//! MacCready, Lamplight Mayor — `{W}{B}` 1/3 Legendary Human Advisor.
//! Whenever a creature you control with power 2 or less attacks, it gains
//!   skulk until end of turn.
//! Whenever a creature with power 4 or greater attacks you, its controller
//!   loses 2 life and you gain 2 life.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("MacCready, Lamplight Mayor");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_power(2),
                },
                intervening_if: None,
                effect: grant_skulk,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().with_min_power(4),
                },
                intervening_if: None,
                effect: drain_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grant_skulk(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Skulk,
        duration: Duration::EndOfTurn,
    }]
}

fn drain_attacker(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "attacks you" — only fire when MacCready's controller is the defender.
    if trig.defending_player() != Some(trig.controller) {
        return Vec::new();
    }
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    let Some(controller) = state.objects.get(id).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: controller,
            amount: 2,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 2,
        },
    ]
}
