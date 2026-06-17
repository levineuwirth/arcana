//! Lady Octopus, Inspired Inventor — `{U}` 0/2 Legendary Human Scientist Villain.
//! Whenever you draw your first or second card each turn, put an ingenuity counter on her.
//! {T}: You may cast an artifact spell from your hand with mana value <= the number of
//!   ingenuity counters on her without paying its mana cost. (GAP)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lady Octopus, Inspired Inventor");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let villain = reg.interner_mut().intern("Villain");
    let ingenuity = reg.interner_mut().intern("ingenuity");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);
    subtypes.0.insert(villain);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: arcana_core::types::SupertypeSet(arcana_core::types::SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    let _ = ingenuity;
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                // "first or second card each turn" — allow only while <= 2 cards drawn this turn.
                intervening_if: Some(if_first_or_second_draw),
                effect: add_ingenuity,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: You may cast an artifact spell from your hand with mana value less than or equal to the number of ingenuity counters on Lady Octopus without paying its mana cost."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                // GAP: "cast an artifact spell from hand without paying its mana cost, mv <= counter
                // count" — free-cast-from-hand with a dynamic mv gate is not an expressible Effect.
                effect: noop_act,
            }),
    )
}

fn if_first_or_second_draw(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // The drawing event has already been logged when the trigger goes on the stack, so the
    // count is 1 for the first draw and 2 for the second.
    script::cards_drawn_this_turn(s, you) <= 2
}

fn add_ingenuity(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("ingenuity").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}

fn noop_act(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
