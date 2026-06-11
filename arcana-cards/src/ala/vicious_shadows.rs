//! Vicious Shadows — `{6}{R}` enchantment.
//! "Whenever a creature dies, you may have this enchantment deal damage
//! to target player equal to the number of cards in that player's hand."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vicious Shadows");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: damage_equal_to_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // "you may have this enchantment deal damage to target
                // player" — UpTo(1) lets the controller decline by
                // choosing no target.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…deal damage to target player equal to the number of cards in that
/// player's hand."
fn damage_equal_to_hand(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let amount = script::hand_size(state, *p);
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*p),
        amount,
        source: trig.source,
    }]
}
