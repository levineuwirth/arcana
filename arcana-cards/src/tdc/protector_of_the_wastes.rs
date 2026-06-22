//! Protector of the Wastes — `{4}{W}{W}` 5/5 white Dragon.
//!
//! Flying.
//! When this creature enters or becomes monstrous, exile up to two
//! target artifacts and/or enchantments controlled by different
//! players. (Only the enters half is modeled; the becomes-monstrous
//! event has no engine hook.)
//! `{4}{W}`: Monstrosity 3. (Put three +1/+1 counters on it and it
//! becomes monstrous — only the counter placement is modeled.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Protector of the Wastes");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When this creature enters or becomes monstrous, exile up to
            // two target artifacts and/or enchantments controlled by
            // different players."
            // GAP: the "or becomes monstrous" half — no monstrous-state event;
            // GAP: the "controlled by different players" constraint is not
            // expressible. Only the enters-half exile of up to two
            // artifact/enchantment permanents is modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_up_to_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
                        )),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            })
            // "{4}{W}: Monstrosity 3."
            // GAP: the "and it becomes monstrous" state flag + the "if it
            // isn't monstrous" gate are unmodeled; only the three +1/+1
            // counters are placed.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}: Monstrosity 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: monstrosity_3,
            }),
    )
}

fn exile_up_to_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ExilePermanent { target: *id }),
            _ => None,
        })
        .collect()
}

fn monstrosity_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}
