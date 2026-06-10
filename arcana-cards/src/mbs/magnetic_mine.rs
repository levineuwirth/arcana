//! Magnetic Mine — `{4}` artifact.
//! "Whenever another artifact is put into a graveyard from the
//! battlefield, this artifact deals 2 damage to that artifact's
//! controller." A ZoneChange trigger (battlefield → graveyard,
//! artifact filter); the dead artifact is read via
//! `trig.entering_object()` and its controller via
//! `script::target_controller`. The "another" self-exclusion is a
//! GAP (no exclude-source filter predicate).

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
    let name = reg.interner_mut().intern("Magnetic Mine");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "another artifact" — the trigger filter cannot
                // exclude this card itself; the mine's own death may
                // also fire the trigger.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    },
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: zap_controller,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn zap_controller(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.entering_object() else {
        return Vec::new();
    };
    let player = script::target_controller(state, dead, trig.controller);
    vec![Effect::DealDamage {
        target: DamageTarget::Player(player),
        amount: 2,
        source: trig.source,
    }]
}
