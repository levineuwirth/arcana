//! Swarm of Locus — `{1}{B}` 1/1 Insect Locus with Flying.
//!
//! * Flying.
//! * Whenever Swarm of Locus attacks, it gets +1/+0 until end of turn for
//!   each Locus you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Swarm of Locus");
    let insect = reg.interner_mut().intern("Insect");
    let locus = reg.interner_mut().intern("Locus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(locus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_per_locus,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_per_locus(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Locus").controlled_by(ControllerConstraint::You),
        trig.controller,
    ) as i32;
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
