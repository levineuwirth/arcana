//! Kavu Mauler — `{4}{G}{G}` 4/4 Creature — Kavu.
//! Trample.
//! "Whenever this creature attacks, it gets +1/+1 until end of turn for each
//! other attacking Kavu."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Kavu Mauler");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: pump_per_kavu,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_per_kavu(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Count attacking Kavu, then exclude this creature itself.
    let filter = script::subtype_filter(reg, "Kavu").attacking_only();
    let ids = script::ids_matching(state, &filter, trig.controller);
    let n = ids.into_iter().filter(|&id| id != trig.source).count() as i32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: n,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
