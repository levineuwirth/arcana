//! Predatory Nightstalker — `{3}{B}{B}` 3/2 black Nightstalker.
//! "When this creature enters, you may have target opponent sacrifice a creature
//! of their choice."
//! GAP: "target opponent sacrifices a creature of their choice" — opponent-choice
//! sacrifice not in Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Predatory Nightstalker");
    let nightstalker = reg.interner_mut().intern("Nightstalker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightstalker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: gap_opponent_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gap_opponent_sacrifice(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target opponent sacrifices a creature of their choice" — opponent picks;
    // making each opponent sacrifice a creature as approximation
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|pid| Effect::Sacrifice {
            player: pid,
            filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            count: 1,
        })
        .collect()
}
