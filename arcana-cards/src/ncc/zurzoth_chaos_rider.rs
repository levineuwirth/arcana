//! Zurzoth, Chaos Rider — `{2}{R}` 2/3 Legendary Devil.
//! "Whenever an opponent draws their first card each turn, if it's not their
//! turn, you create a 1/1 red Devil token with 'When this token dies, it deals
//! 1 damage to any target.'"
//! "Whenever one or more Devils you control attack one or more players, you and
//! those players each draw a card, then discard a card at random."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zurzoth, Chaos Rider");
    let devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let devil_attackers = script::subtype_filter(reg, "Devil")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever an opponent draws their first card each turn, if it's
            // not their turn, you create a 1/1 red Devil token …" GAP: neither
            // the "first card each turn" gate nor the "if it's not their turn"
            // intervening-if is expressible; firing on every opponent draw is
            // wrong.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: gap_opponent_first_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever one or more Devils you control attack one or more
            // players, you … draw a card, then discard a card at random."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: devil_attackers,
                },
                intervening_if: None,
                effect: devils_attack_wheel,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gap_opponent_first_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "first card each turn" + "if it's not their turn" — no per-turn
    // first-draw trigger and no opponent-turn intervening-if predicate.
    Vec::new()
}

fn devils_attack_wheel(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you … draw a card, then discard a card at random." GAP: the "and those
    // [attacked] players each" half is not expressible — the set of attacked
    // players isn't an accessor on this trigger.
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::Random,
        },
    ])]
}
