//! Radha's Firebrand — `{1}{R}` 3/1 red Human Warrior.
//! "Whenever this creature attacks, target creature defending player
//! controls with power less than this creature's power can't block this
//! turn.
//! Domain — {5}{R}: This creature gets +2/+2 until end of turn. This
//! ability costs {1} less to activate for each basic land type among
//! lands you control. Activate only once each turn."
//!
//! Abilities:
//! 1. SelfAttacks (target creature) → that creature can't block this
//!    turn. PARTIAL: the "defending player controls, with power less
//!    than this creature's power" restriction is a dynamic-power filter
//!    not expressible in the target filter; any creature is targetable.
//! 2. {5}{R}, once each turn: this creature gets +2/+2 until end of
//!    turn. PARTIAL: the Domain "{1} less per basic land type" cost
//!    reduction can't be expressed in a fixed ManaCost; GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Radha's Firebrand");
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
        // GAP: Domain is not an evergreen KeywordAbility; its only
        // mechanical effect (cost reduction) is on the activated
        // ability and is GAP'd there.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: forbid_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Domain — {5}{R}: This creature gets +2/+2 until end of turn. \
                       Activate only once each turn."
                    .into(),
                cost: ActivationCost {
                    // GAP: "{1} less per basic land type" cost reduction
                    // is not expressible in a fixed ManaCost.
                    mana_cost: ManaCost::parse("{5}{R}").unwrap(),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn forbid_block(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ForbidBlocking {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
