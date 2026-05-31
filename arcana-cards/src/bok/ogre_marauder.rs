//! Ogre Marauder — `{1}{B}{B}` 3/1 black Ogre Warrior. "Whenever this
//! creature attacks, it gains 'This creature can't be blocked' until end
//! of turn unless defending player sacrifices a creature of their choice."
//!
//! Attack trigger via `TriggerCondition::SelfAttacks`. The "unless the
//! defending player sacrifices a creature" gate is a SACRIFICE-cost
//! optional payment levied on the DEFENDING player — `OptionalPaymentKind`
//! only supports Mana / Life in v1, so the conditional gate is GAP-ed.
//! We emit the (no-cost-paid) branch: the creature gains can't-be-blocked
//! until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ogre Marauder");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "unless defending player sacrifices a creature of their choice" —
    // the unless-gate is a SACRIFICE-cost optional payment levied on the
    // defending player; OptionalPaymentKind only supports Mana/Life, so the
    // conditional cannot be modeled. Emit the not-paid branch: this creature
    // can't be blocked until end of turn.
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::EndOfTurn,
    }]
}
