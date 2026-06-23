//! Circle of Flame — `{1}{R}` enchantment.
//! "Whenever a creature without flying attacks you or a planeswalker you
//! control, this enchantment deals 1 damage to that creature."
//!
//! A `CreatureAttacks` trigger filtered to non-flying attackers attacking
//! this card's controller (`attacking_you_only()`); the attacking
//! creature is read via `trig.attacking_creature()` for "that creature".
//!
//! Fidelity note: "or a planeswalker you control" is approximated by the
//! `attacking_you_only()` defending-player check (planeswalker-attack
//! redirection to its controller is not separately modeled).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Circle of Flame");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .attacking_you_only()
                        .without_keyword(KeywordAbility::Flying),
                },
                intervening_if: None,
                effect: ping_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…this enchantment deals 1 damage to that creature."
fn ping_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(attacker) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Object(attacker),
        amount: 1,
        source: trig.source,
    }]
}
