//! Titans' Vanguard — `{3}{R}{G}` 5/5 Eldrazi.
//! Devoid (this card has no color — modeled as colorless; the Devoid
//! keyword itself has no supported variant).
//! When you cast this spell and whenever this creature attacks, put a
//! +1/+1 counter on each colorless creature you control.
//! Trample.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Titans' Vanguard");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let self_filter = ObjectFilter { name: Some(name), ..ObjectFilter::default() };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        // Devoid: colorless despite the red/green pips in the cost.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(self_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: counter_each_colorless,
                trigger_zones: vec![Zone::Battlefield, Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: counter_each_colorless,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_each_colorless(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "put a +1/+1 counter on each colorless creature you control."
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_colors(ColorSet::colorless());
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect()
}
