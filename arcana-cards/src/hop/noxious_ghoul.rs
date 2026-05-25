//! Noxious Ghoul — `{3}{B}{B}` 3/3 black Zombie. "Whenever this creature or
//! another Zombie enters, all non-Zombie creatures get -1/-1 until end of turn."
//! ZoneChange trigger on any Zombie ETB; debuff all non-Zombie creatures.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Noxious Ghoul");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: script::subtype_filter(reg, "Zombie"),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_zombie_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_zombie_etb(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // All non-Zombie creatures get -1/-1.
    let zombie_filter = script::subtype_filter(reg, "Zombie");
    let all_creatures = arcana_core::targets::ObjectFilter::creature();
    // Use ids_matching for all creatures, then filter out Zombies.
    let all_ids = script::ids_matching(state, &all_creatures, trig.controller);
    let zombie_ids: std::collections::HashSet<_> = script::ids_matching(state, &zombie_filter, trig.controller).into_iter().collect();
    let non_zombie_ids: Vec<_> = all_ids.into_iter().filter(|id| !zombie_ids.contains(id)).collect();
    let inner = Effect::Pump {
        target: arcana_core::objects::NULL_OBJECT_ID,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    };
    vec![Effect::ForEach { targets: non_zombie_ids, effect: Box::new(inner) }]
}
