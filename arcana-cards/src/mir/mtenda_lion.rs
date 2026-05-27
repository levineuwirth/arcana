//! Mtenda Lion — `{G}` 2/1 green Cat.
//! "Whenever this creature attacks, defending player may pay {U}. If that player does,
//! prevent all combat damage that would be dealt by this creature this turn."
//! GAP: "defending player may pay" — OptionalPayment with chooser = defending player not fully
//! supported (defending_player() is available, but PreventDamage FROM source not modeled).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mtenda Lion");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(defending) = trig.defending_player() else { return Vec::new(); };
    // GAP: PreventDamage FROM source (this creature) not modeled — PreventDamage API targets recipients.
    vec![Effect::OptionalPayment {
        chooser: defending,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{U}").expect("valid cost")),
        then: Box::new(Effect::PreventDamage {
            target: DamageTarget::Object(trig.source),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        }),
        else_effect: None,
    }]
}
