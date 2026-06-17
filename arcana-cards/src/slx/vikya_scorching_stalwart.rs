//! Vikya, Scorching Stalwart — `{2}{W}` 2/4 Legendary Human Warrior.
//! Training (whenever it attacks with another creature with greater power, put a
//! +1/+1 counter on it); `{4}{R}, {Q}, Discard a card: deals damage equal to its
//! power to any target; if excess damage to a creature, draw a card.`
//!
//! Training has no keyword variant; it is modeled as a SelfAttacks trigger that
//! adds a +1/+1 counter — the "with another creature with greater power"
//! precondition is GAP'd (no such intervening-if). The activated ability's `{Q}`
//! (untap) cost is GAP'd (no untap-self cost field); the discard-a-card cost is
//! wired. The damage is `equal to its power` (dynamic); the "excess → draw" rider
//! is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vikya, Scorching Stalwart");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: "with another creature with greater power" — no such gate.
                intervening_if: None,
                effect: training_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{R}, {Q}, Discard a card: Vikya deals damage equal to its power to any target."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{R}").expect("valid cost"),
                    // GAP: {Q} (untap-self) cost is not an ActivationCost field.
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_equal_to_power,
            }),
    )
}

fn training_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn damage_equal_to_power(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    let amount = script::power_of(state, ctx.source).max(0) as u32;
    // GAP: "If excess damage was dealt to a creature this way, draw a card."
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount,
    }]
}
