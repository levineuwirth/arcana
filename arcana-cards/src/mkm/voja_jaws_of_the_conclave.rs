//! Voja, Jaws of the Conclave — `{2}{R}{G}{W}` 5/5 Legendary Creature — Wolf.
//! Vigilance, trample, ward {3}.
//! Whenever Voja attacks, put X +1/+1 counters on each creature you control,
//! where X is the number of Elves you control; draw a card for each Wolf you
//! control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voja, Jaws of the Conclave");
    let wolf = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{3}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let elves = script::count_matching(state, &script::subtype_filter(reg, "Elf"), trig.controller);
    let wolves =
        script::count_matching(state, &script::subtype_filter(reg, "Wolf"), trig.controller);
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::ForEach {
            targets: creatures,
            effect: Box::new(Effect::AddCounters {
                target: NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: elves,
            }),
        },
        Effect::DrawCards {
            player: trig.controller,
            count: wolves,
        },
    ]
}
