//! Vodalian War Machine — `{1}{U}{U}` 0/4 Wall with Defender.
//! "Tap an untapped Merfolk you control: This creature can attack this
//! turn as though it didn't have defender."
//! "Tap an untapped Merfolk you control: This creature gets +2/+1 until
//! end of turn."
//! "When this creature dies, destroy all Merfolk tapped this turn to pay
//! for its abilities."
//!
//! The "can attack despite Defender" ability and the death trigger
//! (destroy Merfolk tapped for costs — no tracking of which permanents
//! paid which costs) are not expressible and are GAP'd. The +2/+1
//! pump ability is fully wired.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vodalian War Machine");
    let wall = reg.interner_mut().intern("Wall");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let merfolk_filter = ObjectFilter::permanent().with_subtype_sym(merfolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped Merfolk you control: This creature can attack this turn as though it didn't have defender.".into(),
                cost: ActivationCost {
                    tap_other: Some(merfolk_filter.clone()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: attack_despite_defender,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped Merfolk you control: This creature gets +2/+1 until end of turn.".into(),
                cost: ActivationCost {
                    tap_other: Some(merfolk_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_despite_defender(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can attack this turn as though it didn't have defender" — no
    // effect that lifts the Defender attack restriction.
    Vec::new()
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn on_death(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "destroy all Merfolk tapped this turn to pay for its abilities"
    // — no tracking of which permanents were tapped for which costs.
    Vec::new()
}
