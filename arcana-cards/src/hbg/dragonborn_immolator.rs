//! Dragonborn Immolator — `{3}{R}` 2/4 Dragon Shaman.
//! "{2}{R}: This creature gets +1/+0 until end of turn.
//!  Gift of Tiamat — When this creature dies, if its power is greater than 0,
//!  note its power. You get a one-time boon with 'When you cast a creature
//!  spell, it perpetually gets +X/+0, where X is the noted number.'"

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonborn Immolator");
    let dragon = reg.interner_mut().intern("Dragon");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP keyword: "Gift of Tiamat" is not a usable KeywordAbility variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "{2}{R}: This creature gets +1/+0 until end of turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: This creature gets +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
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
            // "Gift of Tiamat — When this creature dies, ..."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: gift_of_tiamat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn gift_of_tiamat(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "note its power. You get a one-time boon with 'When you cast a
    // creature spell, it perpetually gets +X/+0 ...'" — the noted-power one-time
    // boon and the perpetual-buff machinery are not expressible.
    Vec::new()
}
