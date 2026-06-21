//! Isu the Abominable — `{3}{U}{U}` 5/5 Legendary Snow Creature — Yeti.
//! "You may look at the top card of your library any time.
//!  You may play snow lands and cast snow spells from the top of your library.
//!  Whenever another snow permanent you control enters, you may pay {G}, {W},
//!  or {U}. If you do, put a +1/+1 counter on Isu."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Isu the Abominable");
    let yeti = reg.interner_mut().intern("Yeti");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(yeti);

    // GAP (static): "You may look at the top card of your library any time" —
    // no documented effect/static hook for revealing the top card.
    // GAP (static): "You may play snow lands and cast snow spells from the top
    // of your library" — no documented play-from-top static hook.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY | SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .controlled_by(ControllerConstraint::You)
                    .with_supertypes(SupertypeSet::new().with(SupertypeSet::SNOW)),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: snow_enters_pay,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn snow_enters_pay(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "{G}, {W}, or {U}" color CHOICE isn't expressible — OptionalPayment
    // takes a single ManaCost; using {U} (a color Isu can always tap for) as the
    // representative payment. If paid, put a +1/+1 counter on Isu.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{U}").expect("valid cost")),
        then: Box::new(Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
        else_effect: None,
    }]
}
