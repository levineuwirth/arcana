//! Rysorian Badger — `{2}{G}` 2/2 green Badger. "Whenever this
//! creature attacks and isn't blocked, you may exile up to two target
//! creature cards from defending player's graveyard. If you do, you
//! gain 1 life for each card exiled this way and this creature assigns
//! no combat damage this turn." Models the optional graveyard exile
//! (up to two creature cards) plus the per-card life gain. GAP: "this
//! creature assigns no combat damage this turn" has no catalog Effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rysorian Badger");
    let badger = reg.interner_mut().intern("Badger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(badger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacksUnblocked,
                intervening_if: None,
                effect: exile_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            }),
    )
}

fn exile_and_gain(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();
    let mut exiled = 0u32;
    for t in trig.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ExileFromGraveyard { target: *id });
            exiled += 1;
        }
    }
    if exiled > 0 {
        effects.push(Effect::GainLife { player: trig.controller, amount: exiled });
    }
    // GAP: "this creature assigns no combat damage this turn" has no catalog Effect.
    effects
}
