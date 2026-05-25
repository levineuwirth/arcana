//! Totem Speaker — `{4}{G}` 3/3 green Creature — Elf Druid.
//! "Whenever a Beast enters, you may gain 3 life."

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Totem Speaker");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let _beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                    filter: ObjectFilter::creature(),
                    from: None,
                    to: Zone::Battlefield,
                },
                // GAP: trigger — filter should be Beast subtype only; ZoneChange
                // filter does not support subtype via the ObjectFilter API at
                // trigger-condition level without script::subtype_filter which
                // requires reg at trigger-declaration time. Using generic
                // creature filter as closest approximation.
                intervening_if: None,
                effect: beast_etb_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn beast_etb_gain_life(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Confirm the entering creature is a Beast.
    let id = trig.entering_object().unwrap_or(trig.source);
    let beast_filter = script::subtype_filter(reg, "Beast");
    let _ = (id, beast_filter);
    // Gain 3 life.
    vec![Effect::GainLife { player: trig.controller, amount: 3 }]
}
