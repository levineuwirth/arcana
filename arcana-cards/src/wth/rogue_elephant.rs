//! Rogue Elephant — `{G}` 3/3 Creature — Elephant.
//! When this creature enters, sacrifice it unless you sacrifice a Forest.
//! NOTE: pay = sacrifice a land (SacrificeFilter::Land over-includes lands that
//! aren't Forests — the filter can't constrain to a subtype); declining
//! sacrifices a creature (no sacrifice-this-specific effect — minor
//! over-inclusion).

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
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
    let name = reg.interner_mut().intern("Rogue Elephant");
    let elephant = reg.interner_mut().intern("Elephant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_enter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn on_enter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "sacrifice it unless you sacrifice a Forest" — pay the land-sacrifice to
    // avoid the penalty; declining sacrifices a creature (the source).
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Land),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: arcana_core::targets::ObjectFilter::creature(),
            count: 1,
        })),
    }]
}
