//! Centaur Vinecrasher — `{3}{G}` Plant Centaur with Trample.
//! Enters with +1/+1 counters equal to land cards in all graveyards (dynamic
//! amount not computable here — GAP).
//! Whenever a land card is put into a graveyard from anywhere, you may pay
//! {G}{G}; if you do, return this card from your graveyard to your hand.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Centaur Vinecrasher");
    let plant = reg.interner_mut().intern("Plant");
    let centaur = reg.interner_mut().intern("Centaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(centaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let land_filter = ObjectFilter::permanent().with_types(TypeLine::LAND.into());

    // GAP: "enters with a number of +1/+1 counters equal to the number of land
    // cards in all graveyards" — no script helper for land cards across all
    // graveyards.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: land_filter,
                from: None,
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: maybe_return,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn maybe_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{G}{G}").expect("valid cost")),
        then: Box::new(Effect::ReturnFromGraveyardToHand { target: trig.source }),
        else_effect: None,
    }]
}
