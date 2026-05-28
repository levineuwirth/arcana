//! Shrouded Serpent — `{4}{U}{U}{U}` 4/4 Creature — Serpent.
//! Whenever this creature attacks, defending player may pay {4}. If that player doesn't,
//! this creature can't be blocked this turn.
//! GAP: OptionalPayment polarity — "Z unless they pay" shape; "can't be blocked" not in catalog.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shrouded Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(defending) = trig.defending_player() else {
        return Vec::new();
    };
    // "Z unless they pay" polarity: punishment in else_effect; then is no-op
    // GAP: "can't be blocked" not in catalog
    vec![Effect::OptionalPayment {
        chooser: defending,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{4}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])), // paid → no effect
        else_effect: Some(Box::new(Effect::Sequence(vec![]))), // GAP: can't be blocked
    }]
}
