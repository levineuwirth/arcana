//! Guul Draz Overseer — `{4}{B}{B}` 3/4 Vampire with Flying.
//! Landfall — Whenever a land you control enters, other creatures you control
//! get +1/+0 until end of turn. If that land is a Swamp, those creatures get
//! +2/+0 until end of turn instead.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guul Draz Overseer");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: arcana_core::targets::ObjectFilter::permanent()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: pump_other_creatures,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_other_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "if that land is a Swamp, +2/+0 instead" upgrade is not
    // expressible (no single-id subtype check on the entering land). Only the base
    // +1/+0 to other creatures you control is emitted.
    let ids: Vec<_> = script::ids_matching(
        state,
        &arcana_core::targets::ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    )
    .into_iter()
    .filter(|&id| id != trig.source)
    .collect();
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
