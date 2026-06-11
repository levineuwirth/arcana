//! Hissing Miasma — `{1}{B}{B}` enchantment (Guildpact).
//! "Whenever a creature attacks you, its controller loses 1 life."

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Hissing Miasma");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: drain_attacker_controller,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "Whenever a creature attacks YOU, its controller loses 1 life." The
/// attacks-you restriction is enforced via `trig.defending_player()`;
/// "its controller" is read with the documented 2-player opponents
/// read (GAP: no accessor for the attacking creature / its controller
/// on CreatureAttacks — in multiplayer this picks the first opponent).
fn drain_attacker_controller(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    if trig.defending_player() != Some(trig.controller) {
        return Vec::new();
    }
    let Some(p) = script::opponents(state, trig.controller).first().copied()
    else {
        return Vec::new();
    };
    vec![Effect::LoseLife {
        player: p,
        amount: 1,
    }]
}
