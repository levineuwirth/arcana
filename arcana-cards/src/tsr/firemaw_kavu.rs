//! Firemaw Kavu — `{5}{R}` 4/2 red Creature — Kavu.
//!
//! Echo {5}{R}. When this creature enters, it deals 2 damage to target
//! creature. When this creature leaves the battlefield, it deals 4 damage to
//! target creature.
//!
//! Echo is a casting/upkeep-cost keyword not in the usable `KeywordAbility`
//! surface (no `KeywordAbility::Echo`), so `keywords` is empty and Echo is
//! GAP'd. The ETB damage trigger is emitted faithfully. The "leaves the
//! battlefield" trigger has no exact `TriggerCondition`; the closest available
//! variant is `SelfDies`, used here with a GAP note (it covers dying to the
//! graveyard but not bounce/exile leaves).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Firemaw Kavu");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Echo {5}{R} has no KeywordAbility::Echo variant.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_deal_2,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            // GAP: trigger — "leaves the battlefield" has no exact
            // TriggerCondition; SelfDies is the closest available variant.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: ltb_deal_4,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn etb_deal_2(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(*id),
        amount: 2,
    }]
}

fn ltb_deal_4(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Object(*id),
        amount: 4,
    }]
}
