//! Gwyllion Hedge-Mage — `{2}{W/B}` 2/2 Hag Wizard.
//! "When this creature enters, if you control two or more Plains, you may
//! create a 1/1 white Kithkin Soldier creature token."
//! "When this creature enters, if you control two or more Swamps, you may put a
//! -1/-1 counter on target creature."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gwyllion Hedge-Mage");
    let hag = reg.interner_mut().intern("Hag");
    let wizard = reg.interner_mut().intern("Wizard");
    let _kithkin = reg.interner_mut().intern("Kithkin");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hag);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
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
                intervening_if: Some(if_two_plains),
                effect: make_kithkin_soldier,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(if_two_swamps),
                effect: minus_counter_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn if_two_plains(s: &GameState, _src: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_at_least(s, you, &script::subtype_filter(reg, "Plains"), 2)
}

fn if_two_swamps(s: &GameState, _src: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_at_least(s, you, &script::subtype_filter(reg, "Swamp"), 2)
}

fn make_kithkin_soldier(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(kithkin) = reg.interner().lookup("Kithkin") else { return Vec::new(); };
    let Some(soldier) = reg.interner().lookup("Soldier") else { return Vec::new(); };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: kithkin,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn minus_counter_target(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}
