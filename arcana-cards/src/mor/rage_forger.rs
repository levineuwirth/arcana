//! Rage Forger — `{2}{R}` 2/2 Elemental Shaman (red).
//!
//! Oracle:
//! * "When this creature enters, put a +1/+1 counter on each other Shaman
//!   creature you control." — ETB trigger; ForEach over Shaman creatures you
//!   control adds a +1/+1 counter to each. ("Other" is approximated by the
//!   subtype set — there is no self-exclude; this creature is itself a Shaman,
//!   so it may also receive a counter — a minor fidelity GAP.)
//! * "Whenever a creature you control with a +1/+1 counter on it attacks, you
//!   may have that creature deal 1 damage to target player or planeswalker." —
//!   CreatureAttacks trigger filtered to your creatures bearing a +1/+1 counter;
//!   the attacking creature deals 1 damage to the chosen target. There is no
//!   "player or planeswalker" TargetFilter — `any_target()` is the closest
//!   superset (its creature inclusion is a minor fidelity GAP). The "you may" is
//!   a resolution-time choice.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rage Forger");
    let elemental = reg.interner_mut().intern("Elemental");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counter_shamans,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter {
                        has_counter: Some(CounterKind::PlusOnePlusOne),
                        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
                    },
                },
                intervening_if: None,
                effect: attacker_pings,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn etb_counter_shamans(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Shaman")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}

fn attacker_pings(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(attacker) = trig.attacking_creature() else { return Vec::new(); };
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: attacker,
        target: dt,
        amount: 1,
    }]
}
