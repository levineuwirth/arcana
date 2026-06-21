//! Myojin of Towering Might — `{5}{G}{G}{G}` 8/8 Legendary Spirit.
//!
//! Oracle:
//! * Enters with an indestructible counter on it if you cast it from
//!   your hand. (Modeled as an ETB add-counters of a named
//!   "indestructible" counter; GAP: the "if you cast it from your hand"
//!   condition is not expressible — the counter is always added.)
//! * Remove an indestructible counter from Myojin of Towering Might:
//!   Distribute eight +1/+1 counters among any number of target
//!   creatures you control. They gain trample until end of turn.
//!   GAP: distributing N counters among any-number-of-targets is not
//!   expressible; the trample grant + the remove-counter cost ARE.

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myojin of Towering Might");
    let spirit = reg.interner_mut().intern("Spirit");
    let _indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    let indestructible_kind =
        CounterKind::Named(reg.interner().lookup("indestructible").expect("interned"));

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_indestructible_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove an indestructible counter from Myojin of Towering Might: \
                       Distribute eight +1/+1 counters among any number of target creatures \
                       you control. They gain trample until end of turn."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((indestructible_kind, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Any,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: distribute_and_trample,
            }),
    )
}

fn etb_indestructible_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if you cast it from your hand" condition not expressible.
    let Some(kind) = reg.interner().lookup("indestructible").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}

fn distribute_and_trample(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: distributing eight +1/+1 counters among any number of targets
    // is not expressible (no divided-counter primitive). The trample grant
    // on the chosen creatures IS expressible.
    let mut out = Vec::new();
    for t in &ctx.targets.targets {
        if let TargetChoice::Object(id) = t {
            out.push(Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::Trample,
                duration: Duration::EndOfTurn,
            });
        }
    }
    out
}
