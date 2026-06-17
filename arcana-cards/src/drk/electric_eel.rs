//! Electric Eel — `{U}` 1/1 Fish.
//! "When this creature enters, it deals 1 damage to you."
//! "{R}{R}: This creature gets +2/+0 until end of turn and deals 1 damage to you."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Electric Eel");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}: This creature gets +2/+0 until end of turn and deals 1 damage to you.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_damage,
            }),
    )
}

fn etb_damage(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(trig.controller),
        amount: 1,
    }]
}

fn pump_and_damage(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Pump {
            target: ctx.source,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
        },
    ]
}
