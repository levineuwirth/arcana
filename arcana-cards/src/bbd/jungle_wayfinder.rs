//! Jungle Wayfinder — `{2}{G}` 3/3 green Elf Warrior. "When this
//! creature enters, each player may search their library for a basic
//! land card, reveal it, put it into their hand, then shuffle."
//!
//! # Gaps
//!
//! * The oracle's "may" is not expressible — the engine's
//!   `TutorToHand` has no optional flag, so the search is emitted
//!   as compulsory.
//! * `ObjectFilter` exposes no `basic`/supertype refinement, so the
//!   filter is widened to any LAND card (not just basics).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jungle Wayfinder");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_each_player_tutors_a_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: each player searches their library for a (basic)
/// land card, puts it into their hand, and shuffles. Implemented as
/// a `Sequence` of per-player `TutorToHand` effects.
fn etb_each_player_tutors_a_land(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: oracle says "may" (optional search) — engine's TutorToHand
    // has no optional flag, so the search is emitted as compulsory.
    // GAP: oracle says "basic" land — ObjectFilter has no basic /
    // supertype refinement, so the filter is widened to any land.
    let land_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let effects: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::TutorToHand {
            player: p,
            filter: land_filter.clone(),
            reveal: true,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
