//! Goblin Snowman — `{3}{R}` 1/1 Goblin.
//!
//! Oracle:
//! * Whenever this creature blocks, prevent all combat damage that would
//!   be dealt to and dealt by it this turn. (The "dealt to it" half is
//!   wired via `Effect::PreventDamage` on the source; the "dealt by it"
//!   half is GAP'd — `PreventDamageFrom` takes an `ObjectFilter`, not a
//!   single object id, so the source-relative prevention isn't
//!   expressible here.)
//! * {T}: This creature deals 1 damage to target creature it's blocking.
//!   (Targeted via `TargetFilter::CreatureBlockingOrBlockedBySource`.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Snowman");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: prevent_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: This creature deals 1 damage to target creature it's blocking.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::CreatureBlockingOrBlockedBySource,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_blocked_creature,
            }),
    )
}

fn prevent_combat_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Prevent all damage that would be dealt TO Goblin Snowman this turn.
    // GAP: the "dealt BY it" half — preventing the damage this creature
    // deals to its blocker — isn't expressible: `PreventDamageFrom` keys
    // on an `ObjectFilter`, not a single source id.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(trig.source),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn ping_blocked_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 1,
    }]
}
