//! District Guide — `{2}{G}` 2/2 Elf Scout. "When this creature
//! enters, you may search your library for a basic land card or
//! Gate card, reveal it, put it into your hand, then shuffle."
//!
//! GAPs:
//!   * The "may" optionality isn't expressible — the engine resolves
//!     `TutorToHand` unconditionally; faithful enough for most plays.
//!   * `ObjectFilter` has no "basic supertype" predicate and no
//!     "subtype Gate OR basic" disjunction. We fall back to a plain
//!     land filter; the chosen card will be any land card rather than
//!     restricted to basics-or-Gates. The `reveal: true` flag matches
//!     the printed reveal step.

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
    let name = reg.interner_mut().intern("District Guide");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: tutor for a land card (best-effort — see GAP note on
/// the basic-or-Gate refinement).
fn etb_tutor_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should be restricted to "basic land card or Gate card";
    // ObjectFilter exposes no basic-supertype predicate and no
    // subtype-OR-supertype disjunction, so we tutor any land.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        reveal: true,
    }]
}
