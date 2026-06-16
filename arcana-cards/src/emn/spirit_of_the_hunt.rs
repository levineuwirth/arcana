//! Spirit of the Hunt — `{1}{G}{G}` 3/3 Wolf Spirit with Flash.
//! When this creature enters, each other creature you control that's a
//! Wolf or a Werewolf gets +0/+3 until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirit of the Hunt");
    let wolf = reg.interner_mut().intern("Wolf");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_pump_wolves,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_pump_wolves(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf = reg.interner().lookup("Wolf");
    let werewolf = reg.interner().lookup("Werewolf");
    let mut syms = Vec::new();
    if let Some(w) = wolf {
        syms.push(w);
    }
    if let Some(w) = werewolf {
        syms.push(w);
    }
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(syms);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let effects: Vec<Effect> = ids
        .into_iter()
        .filter(|id| *id != trig.source)
        .map(|id| Effect::Pump {
            target: id,
            power: 0,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
