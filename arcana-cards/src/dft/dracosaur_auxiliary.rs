//! Dracosaur Auxiliary — `{4}{R}{R}` 4/4 red Dinosaur Dragon Mount with
//! Flying and Haste.
//!
//! * Flying, Haste — keyword line. (Saddle is not an available KeywordAbility
//!   variant — see the Saddle ability below.)
//! * "Whenever this creature attacks while saddled, it deals 2 damage to any
//!   target." — a SelfAttacks trigger dealing 2 damage to any target.
//!     - GAP (intervening-if): "while saddled" has no `conditions::`
//!       predicate (no saddled-state condition), so the gate is omitted and
//!       the ability fires on every attack rather than only while saddled.
//! * "Saddle 3 (Tap any number of other creatures you control with total
//!   power 3 or more: This Mount becomes saddled ...)" — GAP: the Saddle
//!   cost (tap creatures totalling a POWER threshold, not a fixed count) and
//!   the "becomes saddled" state change are both inexpressible with the
//!   available activation-cost fields and effects.

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
    let name = reg.interner_mut().intern("Dracosaur Auxiliary");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let dragon = reg.interner_mut().intern("Dragon");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(dragon);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            // GAP (intervening-if): "while saddled" — no saddled-state condition.
            intervening_if: None,
            effect: attacks_deal_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::any_target()],
        }),
    )
}

fn attacks_deal_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 2,
    }]
}
