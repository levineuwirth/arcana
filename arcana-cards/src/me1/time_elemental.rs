//! Time Elemental — `{2}{U}` 0/2 Elemental.
//! "When this creature attacks or blocks, at end of combat, sacrifice it
//! and it deals 5 damage to you."
//! "{2}{U}{U}, {T}: Return target permanent that isn't enchanted to its
//! owner's hand."
//!
//! The combat trigger's delayed "at end of combat" timing and the
//! "attacks OR blocks" disjunction are not directly expressible; we fire
//! the sacrifice + self-damage immediately on attack (best effort) and
//! GAP the block side / end-of-combat delay. The bounce ability is
//! implemented; the "isn't enchanted" restriction has no filter field
//! and is GAP'd (any permanent is targetable).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Time Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "attacks OR blocks" disjunction + "at end of combat"
                // delayed timing not expressible; firing on attack as best effort.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: sac_and_burn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{U}, {T}: Return target permanent that isn't enchanted to its owner's hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    // GAP: "that isn't enchanted" has no ObjectFilter field; any permanent targetable.
                    filter: arcana_core::targets::TargetFilter::Permanent(
                        arcana_core::targets::ObjectFilter::permanent(),
                    ),
                    count: arcana_core::targets::TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bounce_permanent,
            }),
    )
}

fn sac_and_burn(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(trig.controller),
            amount: 5,
        },
        Effect::Sacrifice {
            player: trig.controller,
            filter: arcana_core::targets::ObjectFilter::creature(),
            count: 1,
        },
    ]
}

fn bounce_permanent(
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
    vec![Effect::ReturnToHand { target: *id }]
}
