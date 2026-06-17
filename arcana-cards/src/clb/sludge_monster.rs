//! Sludge Monster — `{3}{U}{U}` 5/5 Horror.
//! "Whenever this creature enters or attacks, put a slime counter on up to one
//! other target creature."
//! "Non-Horror creatures with slime counters on them lose all abilities and
//! have base power and toughness 2/2." (global static — GAP'd)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sludge Monster");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let _slime = reg.interner_mut().intern("slime");

    // GAP (global static): "Non-Horror creatures with slime counters lose all
    // abilities and have base power and toughness 2/2" — counter-keyed global
    // ability-strip + base-PT static is not expressible here.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let slime_target = || TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::creature()),
        count: TargetCount::UpTo(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: put_slime,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![slime_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: put_slime,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![slime_target()],
            }),
    )
}

fn put_slime(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(slime) = reg.interner().lookup("slime") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(slime),
        count: 1,
    }]
}
