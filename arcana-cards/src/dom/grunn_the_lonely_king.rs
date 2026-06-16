//! Grunn, the Lonely King — `{4}{G}{G}` 5/5 Legendary Ape Warrior.
//! Kicker {3}. If kicked, enters with five +1/+1 counters. Whenever Grunn
//! attacks alone, double its power and toughness until end of turn.
//!
//! GAP: Kicker is not in the usable KeywordAbility surface, and the
//! "if ~ was kicked" conditional enters-with-counters is not expressible.
//! The attacks-alone doubling IS expressed (Pump by current P/T, which
//! doubles them until end of turn).

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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grunn, the Lonely King");
    let ape = reg.interner_mut().intern("Ape");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacksAlone,
                intervening_if: None,
                effect: double_pt,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn double_pt(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Double its power and toughness" = add its current power/toughness.
    let power = script::power_of(state, trig.source).max(0);
    let toughness = script::toughness_of(state, trig.source).max(0);
    vec![Effect::Pump {
        target: trig.source,
        power,
        toughness,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
