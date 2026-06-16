//! Svega, the Unconventional — `{1}{G}{W}{U}` Legendary Planeswalker — Svega, loyalty 5.
//!
//! Landfall — Whenever a land enters under your control, put a loyalty counter
//!   on target planeswalker.
//! −2: For each planeswalker type among planeswalkers you control, create a 1/1
//!   Attendee creature token that's all colors.
//! −X: Choose an Elspeth, Teferi, Liliana, Chandra, or Garruk planeswalker card
//!   name with mana value X. Create a copy of the card with the chosen name. You
//!   may cast the copy without paying its mana cost.
//!
//! # Scope
//! GAP: the −2 "for each planeswalker type among planeswalkers you control"
//!   count needs distinct-subtype enumeration that the demonstrated surface
//!   can't express; the ability shell is declared with its −2 cost, effect empty.
//! GAP: the −X ability uses a dynamic-X loyalty cost (`remove_self_counter` is a
//!   fixed u32) — not expressible. The ability is OMITTED rather than mis-cost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Svega, the Unconventional");
    let svega = reg.interner_mut().intern("Svega");
    let _attendee = reg.interner_mut().intern("Attendee");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(svega);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::PLANESWALKER.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: For each planeswalker type among planeswalkers you control, create a 1/1 Attendee creature token that's all colors.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            }),
    )
}

fn landfall_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

fn minus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each planeswalker type among planeswalkers you control" requires
    // distinct-subtype enumeration not expressible in the demonstrated surface.
    Vec::new()
}
