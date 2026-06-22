//! Choco, Seeker of Paradise — `{1}{G}{W}{U}` 3/5 Legendary Bird.
//! Whenever one or more Birds you control attack, look at that many cards from
//! the top of your library, you may put one into your hand, then put any number
//! of land cards from among them onto the battlefield tapped and the rest into
//! your graveyard.
//! Landfall — Whenever a land you control enters, Choco gets +1/+0 until end of
//! turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Choco, Seeker of Paradise");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let bird_filter = ObjectFilter::creature()
        .with_subtypes_any(vec![reg.interner_mut().intern("Bird")])
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks { filter: bird_filter },
                intervening_if: None,
                effect: birds_attack_dig,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn birds_attack_dig(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at THAT MANY cards (= number of attacking Birds), may put one
    // into hand, THEN put any number of land cards from among them onto the
    // battlefield tapped and the rest into your graveyard." The look-count is
    // dynamic off the attacking-Bird count (no accessor for that count), and the
    // 'put any number of lands from among the looked-at set onto the battlefield
    // tapped, rest to graveyard' multi-step over a transient revealed set is not
    // expressible (DigTopN is fixed-count, single-take, no land-to-battlefield
    // tail).
    Vec::new()
}

fn landfall_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
