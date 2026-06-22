//! Talion's Messenger — `{2}{U}` 1/3 Creature — Faerie Noble.
//! Flying.
//! Whenever you attack with one or more Faeries, draw a card, then discard a card.
//! When you discard a card this way, put a +1/+1 counter on target Faerie you control.
//!   Modeled as one CreatureAttacks (Faerie you control) trigger that draws, discards,
//!   then adds a +1/+1 counter to a targeted Faerie you control. The discard always
//!   happens, so the reflexive counter trigger is folded in unconditionally.
//!   FIDELITY: CreatureAttacks fires once per attacking Faerie rather than once per
//!   "attack with one or more Faeries"; there is no "attack with one or more" variant.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Talion's Messenger");
    let faerie = reg.interner_mut().intern("Faerie");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(noble);

    let faerie_filter = script::subtype_filter(reg, "Faerie").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: faerie_filter.clone(),
                },
                intervening_if: None,
                effect: attack_loot_and_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(faerie_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn attack_loot_and_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ];
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    vec![Effect::Sequence(effects)]
}
