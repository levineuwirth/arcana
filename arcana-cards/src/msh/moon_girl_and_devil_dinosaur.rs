//! Moon Girl and Devil Dinosaur — `{1}{G}{U}` 2/2 Legendary Creature —
//! Human Dinosaur Hero.
//!
//! * Whenever you draw your second card each turn, until end of turn, this
//!   creature's base power and toughness become 6/6 and it gains trample.
//!   (Modeled as a `CardDrawn` trigger gated by an intervening-if that the
//!   draw is the second of the turn.)
//! * Whenever an artifact you control enters, draw a card. This ability
//!   triggers only once each turn (`OncePerTurn`).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moon Girl and Devil Dinosaur");
    let human = reg.interner_mut().intern("Human");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_draw),
                effect: become_six_six_trample,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

/// Intervening-if: the draw that just fired is the SECOND card drawn this turn.
fn if_second_draw(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::cards_drawn_this_turn(s, you) == 2
}

fn become_six_six_trample(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::SetBasePT {
            target: trig.source,
            power: 6,
            toughness: 6,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
