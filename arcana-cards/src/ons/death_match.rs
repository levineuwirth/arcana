//! Death Match — `{3}{B}` enchantment.
//! "Whenever a creature enters, that creature's controller may have target
//! creature of their choice get -3/-3 until end of turn."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death Match");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: shrink_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: the target should be chosen by the ENTERING
                // creature's controller ("of their choice"), and the whole
                // effect is optional for that player ("may"); the engine's
                // trigger targeting is by this enchantment's controller.
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

/// "…target creature … get[s] -3/-3 until end of turn."
fn shrink_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -3,
        toughness: -3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
