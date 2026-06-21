//! Sun Titan — `{4}{W}{W}` 6/6 Giant with Vigilance.
//! "Whenever this creature enters or attacks, you may return target
//! permanent card with mana value 3 or less from your graveyard to the
//! battlefield."
//!
//! Vigilance is a base keyword. The "enters or attacks" trigger is two
//! trigger events on one ability — modeled as two TriggeredAbilityDefs
//! (SelfEntersBattlefield + SelfAttacks) sharing one resolver that
//! returns a targeted permanent card (mv 3 or less) from the graveyard.

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

fn reanimate_req() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Card {
            zone: Zone::Graveyard(0),
            filter: ObjectFilter::permanent().with_max_cmc(3),
        },
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sun Titan");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: return_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![reanimate_req()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: return_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![reanimate_req()],
            }),
    )
}

fn return_permanent(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
