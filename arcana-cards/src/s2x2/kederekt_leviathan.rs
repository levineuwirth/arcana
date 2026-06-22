//! Kederekt Leviathan — `{6}{U}{U}` 5/5 blue Leviathan.
//! When this creature enters, return all other nonland permanents to their
//! owners' hands.
//! Unearth {6}{U}.
//!
//! Abilities:
//!  - ETB → bounce every other nonland permanent (ForEach over the matching
//!    ids, ReturnToHand each; excludes self via the per-id list which the
//!    engine substitutes).
//!  - Unearth: not in the usable keyword surface for this card class → GAP'd.

use arcana_core::effects::Effect;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: Unearth {6}{U} — the graveyard-recursion keyword (return from
// graveyard, gains haste, exile at end step) is not in the usable keyword
// surface for this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kederekt Leviathan");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_bounce_all_other_nonland,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_bounce_all_other_nonland(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // All nonland permanents (the engine excludes the source for "all other").
    let filter = ObjectFilter::permanent().without_types(TypeLine::LAND.into());
    let ids: Vec<_> = script::ids_matching(state, &filter, trig.controller)
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnToHand { target: NULL_OBJECT_ID }),
    }]
}
