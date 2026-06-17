//! Graven Dominator — `{4}{W}{W}` 4/4 Gargoyle with Flying.
//! "Flying. Haunt. When this creature enters or the creature it haunts dies,
//!  each other creature has base power and toughness 1/1 until end of turn."
//!
//! Flying is a base keyword. Haunt itself (exile-haunting on death + the
//! second trigger leg) is not modeled, so only the ETB leg of the combined
//! trigger is wired here.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Graven Dominator");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Haunt keyword (exile-it-haunting on death) — not a supported KeywordAbility variant.
    // The "or the creature it haunts dies" leg of the trigger depends on Haunt and is omitted.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: each_other_creature_becomes_1_1,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Each OTHER creature has base power and toughness 1/1 until end of turn.
fn each_other_creature_becomes_1_1(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    ids.into_iter()
        .filter(|id| *id != trig.source)
        .map(|id| Effect::SetBasePT {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
        })
        .collect()
}
