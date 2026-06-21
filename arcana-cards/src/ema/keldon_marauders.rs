//! Keldon Marauders — `{1}{R}` 3/3 Human Warrior.
//!
//! "Vanishing 2
//!  When this creature enters or leaves the battlefield, it deals 1
//!  damage to target player or planeswalker."
//!
//! Decomposition: Vanishing 2 keyword + the "enters or leaves" ping,
//! which has no single combined trigger condition, so it is split into
//! two triggered abilities (SelfEntersBattlefield and
//! SelfLeavesBattlefield). "Target player or planeswalker" is declared
//! via `any_target()` (a documented widening — it also admits
//! creatures; no narrower TargetFilter exists).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keldon Marauders");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vanishing(2)],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: ping_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target player or planeswalker" — any_target() also
                // admits creatures; no narrower TargetFilter exists.
                target_requirements: vec![TargetRequirement::any_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: ping_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target player or planeswalker" — any_target() also
                // admits creatures; no narrower TargetFilter exists.
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn ping_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        target: dt,
        amount: 1,
        source: trig.source,
    }]
}
