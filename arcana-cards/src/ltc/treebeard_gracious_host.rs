//! Treebeard, Gracious Host — `{2}{G}{W}` 0/5 Legendary Treefolk.
//! Trample, ward {2}. When Treebeard enters, create two Food tokens.
//! "Whenever you gain life, put that many +1/+1 counters on target
//! Halfling or Treefolk." — the "that many" count is the amount of
//! life gained, which has no PendingTrigger accessor → GAP'd.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Treebeard, Gracious Host");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let halfling = reg.interner_mut().intern("Halfling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    let halfling_or_treefolk = ObjectFilter::creature()
        .with_subtypes_any(vec![halfling, treefolk]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_two_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: lifegain_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(halfling_or_treefolk),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_two_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 2,
    }]
}

fn lifegain_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put that many +1/+1 counters" — the amount of life gained
    // has no PendingTrigger accessor (no life_gained_amount()), so the
    // dynamic count is unobtainable.
    Vec::new()
}
