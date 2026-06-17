//! Drizzt Do'Urden — `{3}{G}{W}` 3/3 legendary Elf Ranger with Double strike.
//! When Drizzt enters, create Guenhwyvar, a legendary 4/1 green Cat token
//! with trample.
//! Whenever a creature dies, if it had power greater than Drizzt's power,
//! put that many +1/+1 counters on Drizzt equal to the difference.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drizzt Do'Urden");
    let elf = reg.interner_mut().intern("Elf");
    let ranger = reg.interner_mut().intern("Ranger");
    let _cat = reg.interner_mut().intern("Cat");
    let _guen = reg.interner_mut().intern("Guenhwyvar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_guenhwyvar,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: arcana_core::targets::ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                // GAP: intervening-if "if it had power greater than Drizzt's
                // power" compares the dying object's power to the source's —
                // no conditions:: helper expresses that. The effect gates on
                // a positive difference instead (equivalent: zero counters
                // when not greater).
                intervening_if: None,
                effect: dies_counter_difference,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_guenhwyvar(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    let token_name = reg.interner().lookup("Guenhwyvar").unwrap_or(cat);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Trample],
            abilities: vec![],
        },
    }]
}

fn dies_counter_difference(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else { return Vec::new(); };
    let dead_power = script::power_of(state, dead);
    let drizzt_power = script::power_of(state, trig.source);
    let diff = dead_power - drizzt_power;
    if diff <= 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: diff as u32,
    }]
}
