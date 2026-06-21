//! Bladewing's Thrall — `{2}{B}{B}` 3/3 black Zombie.
//! "This creature has flying as long as you control a Dragon." (GAP)
//! "When a Dragon enters, you may return this card from your graveyard to the
//! battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bladewing's Thrall");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    // Pre-intern the Dragon subtype so the trigger filter can reference it.
    let dragon_filter = script::subtype_filter(reg, "Dragon");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "has flying as long as you control a Dragon" — conditional static
    // keyword grant is not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: dragon_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: return_self_from_graveyard,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn return_self_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may return this card from your graveyard to the battlefield"
    // (the "may" is a resolution-time choice handled by the engine).
    vec![Effect::ReturnFromGraveyardToBattlefield {
        target: trig.source,
    }]
}
