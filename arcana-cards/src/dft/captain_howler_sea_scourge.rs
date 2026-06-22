//! Captain Howler, Sea Scourge — `{2}{U}{R}` 5/4 Legendary Creature — Shark Pirate.
//! Ward—{2}, Pay 2 life.
//! Whenever you discard one or more cards, target creature gets +2/+0 until
//! end of turn for each card discarded this way. Whenever that creature deals
//! combat damage to a player this turn, you draw a card.
//!
//! Ward with a mixed mana+life cost ("{2}, Pay 2 life") is not expressible as
//! KeywordAbility::Ward(ManaCost) and is GAP'd. The discard trigger targets a
//! creature, pumps it +2/+0 per card discarded this turn, and grants it (for
//! the turn) a "deals combat damage to a player → you draw a card" rider via
//! GrantTriggeredAbility.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Captain Howler, Sea Scourge");
    let shark = reg.interner_mut().intern("Shark");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shark);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Ward—{2}, Pay 2 life." is a mixed mana+life ward cost, not
        // expressible as KeywordAbility::Ward(ManaCost).
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDiscarded {
                player: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: discard_pump_and_grant,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn discard_pump_and_grant(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // "+2/+0 until end of turn for each card discarded this way." Approximated
    // as cards discarded this turn by the controller.
    let discarded = script::cards_discarded_this_turn(state, trig.controller);
    let power = (2 * discarded) as i32;
    vec![
        Effect::Pump {
            target: *id,
            power,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::GrantTriggeredAbility {
            target: *id,
            ability: Box::new(TriggeredAbilityDef {
                id: GRANTED_TRIGGER_ID_BASE + 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: granted_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
            duration: Duration::EndOfTurn,
        },
    ]
}

fn granted_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
