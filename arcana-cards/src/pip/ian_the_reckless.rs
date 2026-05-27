//! Ian the Reckless — `{1}{R}` 2/1 red Legendary Human Warrior.
//! "Whenever Ian the Reckless attacks, if it's modified, you may have it deal
//! damage equal to its power to you and any target."
//! GAP: "if it's modified" — intervening_if checking for Equipment, Auras, or
//! counters (modified status) not modeled; using None.
//! GAP: "you may have it deal damage" — optional player choice to activate the
//! damage not expressible with OptionalPaymentKind (no cost to pay); using
//! unconditional damage as approximation.
//! GAP: "deal damage equal to its power to you AND any target" — dealing
//! dynamic power damage to both controller and a chosen target requires two
//! DealDamage effects; dynamic power via script::power_of.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ian the Reckless");
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
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: intervening_if "if it's modified" not modeled
                intervening_if: None,
                effect: attacks_deal_power_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AnyTarget,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn attacks_deal_power_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let amount = script::power_of(state, trig.source).max(0) as u32;
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let target_effect = match target {
        TargetChoice::Object(id) => Effect::DealDamage {
            target: DamageTarget::Object(*id),
            amount,
            source: trig.source,
        },
        TargetChoice::Player(p) => Effect::DealDamage {
            target: DamageTarget::Player(*p),
            amount,
            source: trig.source,
        },
        _ => return Vec::new(),
    };
    // GAP: "you may have it" — optional (player choice without cost) not
    // expressible; emitting unconditionally.
    vec![
        Effect::DealDamage {
            target: DamageTarget::Player(trig.controller),
            amount,
            source: trig.source,
        },
        target_effect,
    ]
}
