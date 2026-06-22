//! Monstrosity of the Lake — `{4}{U}` 4/6 Legendary Creature — Kraken.
//! When this enters, you may pay {5}. If you do, tap all creatures your
//! opponents control, then put a stun counter on each of those creatures.
//! Islandcycling {2}.
//!
//! Islandcycling is a typecycling variant — per convention it is emitted
//! as the generic `Cycling` with its printed cost (the type-search of the
//! cycled card is not separately modeled).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monstrosity of the Lake");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_pay_then_tap_stun,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_pay_then_tap_stun(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opp_creatures =
        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &opp_creatures, trig.controller);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{5}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![
            Effect::ForEach {
                targets: ids.clone(),
                effect: Box::new(Effect::Tap { target: NULL_OBJECT_ID }),
            },
            Effect::ForEach {
                targets: ids,
                effect: Box::new(Effect::AddCounters {
                    target: NULL_OBJECT_ID,
                    kind: CounterKind::Stun,
                    count: 1,
                }),
            },
        ])),
        else_effect: None,
    }]
}
