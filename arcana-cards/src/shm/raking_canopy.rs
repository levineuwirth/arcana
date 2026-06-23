//! Raking Canopy — `{1}{G}{G}` enchantment.
//! "Whenever a creature with flying attacks you, this enchantment deals 4
//! damage to it."
//!
//! A `CreatureAttacks` trigger filtered to flying attackers attacking this
//! card's controller (`attacking_you_only()`); the attacking creature is
//! read via `trig.attacking_creature()` for "it".

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
    let name = reg.interner_mut().intern("Raking Canopy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                        .with_keyword(KeywordAbility::Flying),
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

/// "…this enchantment deals 4 damage to it."
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
        amount: 4,
        source: trig.source,
    }]
}
