//! Calix, Guided by Fate — `{1}{G}{W}` 2/2 Legendary Enchantment Creature —
//! Human Druid.
//! Constellation — Whenever Calix or another enchantment you control enters,
//! put a +1/+1 counter on target creature.
//! Whenever Calix or an enchanted creature you control deals combat damage to
//! a player, you may create a token that's a copy of a nonlegendary enchantment
//! you control. Do this only once each turn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Calix, Guided by Fate");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let enchantment_you_control = ObjectFilter::permanent()
        .with_types(TypeLine::ENCHANTMENT.into())
        .controlled_by(ControllerConstraint::You);

    let nonleg_enchantment = ObjectFilter::permanent()
        .with_types(TypeLine::ENCHANTMENT.into())
        .controlled_by(ControllerConstraint::You)
        .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: enchantment_you_control,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: constellation_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: source filter should be "Calix OR an enchanted creature
                // you control"; engine DamageDealt source_filter can't express
                // "enchanted creature", so this fires on any creature you
                // control dealing combat damage to a player.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: copy_enchantment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(nonleg_enchantment),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn constellation_counter(
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
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn copy_enchantment(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::CopyPermanent { target: *id }]
}
