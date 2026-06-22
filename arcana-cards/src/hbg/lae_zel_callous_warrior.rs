//! Lae'zel, Callous Warrior — `{3}{W}{B}` 3/6 Legendary Gith Warrior.
//!
//! Double strike
//! When this creature enters or specializes, return up to two target
//! creature cards with total mana value 3 or less from your graveyard to
//! the battlefield.
//!
//! Decomposed as: a keyword line (Double strike) plus two triggers sharing
//! one resolver — one for "enters", one for "specializes". Each returns up
//! to two target creature cards from your graveyard to the battlefield.
//! The "total mana value 3 or less" aggregate constraint across the chosen
//! pair is not expressible via TargetCount (a per-target filter can't bound
//! a *combined* mana value), so it is GAP'd; the count and zone are faithful.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lae'zel, Callous Warrior");
    let gith = reg.interner_mut().intern("Gith");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gith);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: reanimate_pair,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![graveyard_creature_pair()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: reanimate_pair,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![graveyard_creature_pair()],
            }),
    )
}

fn graveyard_creature_pair() -> TargetRequirement {
    // GAP: "with total mana value 3 or less" — TargetCount cannot bound the
    // aggregate mana value of the chosen pair. Count and zone are faithful.
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::creature(),
        },
        count: TargetCount::UpTo(2),
        controller: None,
    }
}

fn reanimate_pair(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
            }
            _ => None,
        })
        .collect()
}
