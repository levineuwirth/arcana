//! Marcus, Mutant Mayor — `{3}{G}{U}` 4/4 Legendary Mutant Advisor.
//! Vigilance, trample.
//! Whenever a creature you control deals combat damage to a player, draw a
//! card if that creature has a +1/+1 counter on it. If it doesn't, put a
//! +1/+1 counter on it.
//!
//! Vigilance and Trample are base keywords. The combat-damage trigger is
//! wired to the correct condition (a creature you control deals combat
//! damage to a player). Its effect is GAP'd: it branches on whether the
//! DEALING creature has a +1/+1 counter, but the source creature of a
//! DamageDealt trigger has no accessor and Effect::Conditional has no
//! documented counter-presence condition.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marcus, Mutant Mayor");
    let mutant = reg.interner_mut().intern("Mutant");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: counter_or_draw_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_or_draw_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: branches on whether the DEALING creature has a +1/+1 counter,
    // but a DamageDealt trigger exposes no accessor for the source creature
    // and there is no counter-presence Conditional condition available.
    Vec::new()
}
