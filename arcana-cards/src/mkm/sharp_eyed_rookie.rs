//! Sharp-Eyed Rookie — `{1}{G}` 2/2 Human Detective with Vigilance.
//! Whenever a creature you control enters, if its power is greater than this
//! creature's power or its toughness is greater than this creature's toughness,
//! put a +1/+1 counter on this creature and investigate.
//!
//! The comparison gates on the ENTERING creature's stats versus this creature's
//! stats — data only available from the trigger event — so it's evaluated in the
//! effect body (intervening_if can't read the triggering object), then the
//! counter + Clue (investigate) are emitted.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sharp-Eyed Rookie");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: on_bigger_creature_enters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_bigger_creature_enters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let entered = match trig.entering_object() {
        Some(id) => id,
        None => return Vec::new(),
    };
    let entered_power = script::power_of(state, entered);
    let entered_toughness = script::toughness_of(state, entered);
    let my_power = script::power_of(state, trig.source);
    let my_toughness = script::toughness_of(state, trig.source);
    if entered_power > my_power || entered_toughness > my_toughness {
        vec![
            Effect::AddCounters {
                target: trig.source,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            },
            Effect::CreateCommodityToken {
                controller: trig.controller,
                kind: CommodityToken::Clue,
                count: 1,
            },
        ]
    } else {
        Vec::new()
    }
}
