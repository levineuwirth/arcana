//! Vazi, Keen Negotiator — `{2}{B}{R}{G}` Legendary 3/3 Human Advisor.
//!
//! Rules text:
//! * Haste
//! * {T}: Target opponent creates X Treasure tokens, where X is the number of
//!   Treasure tokens you created this turn.
//! * Whenever an opponent casts a spell or activates an ability, if mana from a
//!   Treasure was spent to cast it or activate it, put a +1/+1 counter on target
//!   creature, then draw a card.
//!
//! Haste is faithful. The {T} activated ability is GAP'd: X = "Treasure tokens
//! you created this turn" has no script helper, so the dynamic count can't be
//! computed (emitting a literal would be materially wrong). The opponent-cast
//! trigger is GAP'd: there is no intervening-if predicate for "mana from a
//! Treasure was spent", and "activates an ability" is not a SpellCast event.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vazi, Keen Negotiator");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Target opponent creates X Treasure tokens, where X is the number of Treasure tokens you created this turn.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: opponent_makes_treasures,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: treasure_spent_payoff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn opponent_makes_treasures(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = "Treasure tokens you created this turn" has no script helper, so the
    //       dynamic count cannot be computed; whole effect omitted.
    Vec::new()
}

fn treasure_spent_payoff(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if mana from a Treasure was spent" has no intervening-if predicate;
    //       also "or activates an ability" is not a SpellCast event. Effect omitted.
    Vec::new()
}
