//! Cemetery Prowler — `{1}{G}{G}` 3/4 Wolf.
//! Vigilance.
//! Whenever this creature enters or attacks, exile a card from a graveyard.
//! Spells you cast cost {1} less to cast for each card type they share with
//! cards exiled with this creature.
//!
//! Vigilance is a base keyword. The compound "enters or attacks" trigger is
//! decomposed into two triggers, each exiling a target card from a
//! graveyard. The static graveyard-keyed cost reduction is GAP'd (no
//! cost-reduction primitive keyed on shared card types of exiled cards).

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cemetery Prowler");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "Spells you cast cost {1} less for each card type they
    // share with cards exiled with this creature" — no such cost-reduction
    // primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![graveyard_card_req()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![graveyard_card_req()],
            }),
    )
}

fn graveyard_card_req() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::default(),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

fn exile_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExileFromGraveyard { target: *id }]
}
