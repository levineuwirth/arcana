//! Bloated Processor — `{2}{B}` 3/2 Phyrexian. "Sacrifice another
//! Phyrexian: Put a +1/+1 counter on this creature." "When this creature
//! dies, incubate X, where X is its power."
//!
//! The Scryfall keyword tags Incubate / Transform are mechanic tags, not
//! members of the usable `KeywordAbility` surface, so the keyword line is
//! empty; the incubate is wired as the dies-trigger effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloated Processor");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let phyrexian_filter = script::subtype_filter(reg, "Phyrexian");

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another Phyrexian: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(phyrexian_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter_to_self,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_incubate_power,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_counter_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn dies_incubate_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let n = script::power_of(state, id).max(0) as u32;
    vec![Effect::Incubate { controller: trig.controller, n }]
}
