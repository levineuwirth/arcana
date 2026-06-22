//! Kardur, Doomscourge — `{2}{B}{R}` 4/3 Legendary Demon Berserker.
//! "When Kardur enters, until your next turn, creatures your opponents
//! control attack each combat if able and attack a player other than you
//! if able."
//! "Whenever an attacking creature dies, each opponent loses 1 life and
//! you gain 1 life."
//!
//! No keywords. The ETB compulsory-attack effect is modeled by goading
//! every creature your opponents control (Goad forces it to attack each
//! combat and forbids attacking the goader = you), with the
//! UntilYourNextTurn duration matching the oracle. The dies trigger
//! cannot filter "attacking" (no such filter on a death ZoneChange), so
//! it fires on any creature death (GAP on the attacking restriction); the
//! life-swing payoff is wired fully.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kardur, Doomscourge");
    let demon = reg.interner_mut().intern("Demon");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: goad_opponent_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: cannot restrict to *attacking* creatures dying —
                // a death ZoneChange has no attacking filter; fires on any
                // creature death.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: drain_each_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn goad_opponent_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    let effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::Goad {
            target: id,
            goader: trig.controller,
            duration: Duration::UntilYourNextTurn(trig.controller),
        })
        .collect();
    vec![Effect::Sequence(effects)]
}

fn drain_each_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife {
            player: p,
            amount: 1,
        })
        .collect();
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: 1,
    });
    vec![Effect::Sequence(effects)]
}
