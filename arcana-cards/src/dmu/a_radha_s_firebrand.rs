//! A-Radha's Firebrand — `{1}{R}` 3/1 Human Warrior. (Domain — not a usable
//! KeywordAbility variant; the Domain cost-reduction is noted as a gap.)
//! "Whenever Radha's Firebrand attacks, target creature defending player controls
//! with power less than or equal to Radha's Firebrand's power can't block this turn."
//! "Domain — {5}{R}: Radha's Firebrand gets +2/+2 until end of turn. This ability
//! costs {1} less to activate for each basic land type among lands you control.
//! Activate only once each turn."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Radha's Firebrand");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Domain is not an available KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // target creature defending player (opponent) controls with power <= 3.
    let blocker_filter = ObjectFilter::new()
        .with_types(TypeLine::CREATURE.into())
        .controlled_by(ControllerConstraint::Opponent)
        .with_max_power(3);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: forbid_block_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(blocker_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (cost): "costs {1} less per basic land type among lands you
                // control" is a dynamic cost reduction with no ActivationCost field;
                // the printed {5}{R} is modeled, plus once-per-turn.
                text: "Domain — {5}{R}: Radha's Firebrand gets +2/+2 until end of turn. Activate only once each turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{R}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn forbid_block_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ForbidBlocking {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
