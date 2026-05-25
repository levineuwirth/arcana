//! Rootless Yew — `{3}{G}{G}` 5/4 green Treefolk creature. "When this
//! creature dies, search your library for a creature card with power
//! or toughness 6 or greater, reveal it, put it into your hand, then
//! shuffle." Modeled as a `SelfDies` trigger that invokes
//! `Effect::TutorToHand` with `reveal: true`. The "power OR toughness
//! 6 or greater" disjunction is not expressible with the available
//! `ObjectFilter` refinements (no `with_min_toughness`, and no
//! OR-of-numeric-bounds), so the filter is left as the broader
//! `ObjectFilter::creature()` — see GAP in the effect fn.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rootless Yew");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies_tutor_big_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Dies trigger resolution: search the controller's library for a
/// creature card, reveal it, put it into their hand, shuffle (shuffle
/// is automatic on `TutorToHand`).
fn on_dies_tutor_big_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the oracle restricts to creature cards with power OR
    // toughness 6 or greater. `ObjectFilter` exposes `with_min_power`
    // but no `with_min_toughness`, and no disjunction primitive, so
    // the "6+ power or toughness" constraint cannot be expressed.
    // Falling back to the broader `creature()` filter.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        reveal: true,
    }]
}
