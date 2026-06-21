//! Courser of Kruphix — `{1}{G}{G}` 2/4 Enchantment Creature — Centaur.
//!
//! Oracle:
//! * Play with the top card of your library revealed.  (static — GAP'd)
//! * You may play lands from the top of your library.  (static — GAP'd)
//! * Landfall — Whenever a land you control enters, you gain 1 life.
//!
//! "Landfall" is an ability word, not a keyword. The two static
//! library-access lines have no triggered/activated decomposition — GAP'd.
//! The landfall life-gain trigger is fully expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Courser of Kruphix");
    let centaur = reg.interner_mut().intern("Centaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);

    // "a land you control" entering the battlefield.
    let land_enter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Play with the top card of your library revealed" — no primitive.
    // GAP: static "You may play lands from the top of your library" — no primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: land_enter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn landfall_gain(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 1 }]
}
