//! Arni Metalbrow — `{2}{R}` 3/3 Legendary Creature — Human Berserker.
//! Whenever a creature you control attacks or enters attacking, you may pay {1}{R}. If you do,
//! you may put a creature card with mana value less than that creature's mana value from your
//! hand onto the battlefield tapped and attacking.
//! GAP: "put a creature card from hand onto battlefield tapped and attacking" with mana-value
//! comparison vs. triggering creature — no Effect for ETB-attacking from hand with dynamic
//! mana-value filter. OptionalPayment can wrap the payment but the inner effect is a GAP.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Arni Metalbrow");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: arcana_core::targets::ObjectFilter::creature()
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                },
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
    // GAP: Inner effect "put creature from hand onto battlefield tapped and attacking with
    //      mana value < triggering creature" — no Effect variant for hand-to-battlefield
    //      with tapped+attacking state and dynamic mana-value filter.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{R}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])), // GAP: inner effect not expressible
        else_effect: None,
    }]
}
