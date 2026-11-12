//! Rishkar, Peema Renegade — `{2}{G}` 2/2 Legendary Elf Druid.
//! When Rishkar enters, put a +1/+1 counter on each of up to two
//! target creatures.
//! Each creature you control with a counter on it has "{T}: Add {G}."
//! That is a printed static ability, which the engine installs as Rishkar
//! enters; the mana ability belongs to each creature that has a counter
//! of any kind, for as long as it has one.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rishkar, Peema Renegade");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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
                effect: etb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            })
            // "Each creature you control with a counter on it has
            // '{T}: Add {G}.'" Any kind of counter; Rishkar counts
            // himself when he has one.
            .with_static(vec![ContinuousEffect::filtered_grant_activated(
                NULL_OBJECT_ID,
                ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_any_counter(),
                ActivatedAbilityDef {
                    text: "{T}: Add {G}.".into(),
                    cost: ActivationCost::tap_only(),
                    target_requirements: Vec::new(),
                    is_mana_ability: true,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: true,
                    face_gate: None,
                    effect: add_green,
                },
                Duration::WhileSourceOnBattlefield,
            )]),
    )
}

/// The granted "{T}: Add {G}." `ctx.source` is the creature that has the
/// ability and `ctx.controller` the player who tapped it.
fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn etb_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
            _ => None,
        })
        .collect()
}
