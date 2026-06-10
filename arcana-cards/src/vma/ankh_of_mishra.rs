//! Ankh of Mishra — `{2}` artifact.
//! "Whenever a land enters, this artifact deals 2 damage to that
//! land's controller."
//! A `ZoneChange` trigger (any land entering the battlefield); the
//! effect reads the entering land and damages its controller.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ankh of Mishra");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: zap_land_controller,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn zap_land_controller(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let land = trig.entering_object().unwrap_or(trig.source);
    let controller = script::target_controller(state, land, trig.controller);
    vec![Effect::DealDamage {
        target: DamageTarget::Player(controller),
        amount: 2,
        source: trig.source,
    }]
}
