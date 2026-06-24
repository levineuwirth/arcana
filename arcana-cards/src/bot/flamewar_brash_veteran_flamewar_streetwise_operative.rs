//! Flamewar, Brash Veteran // Flamewar, Streetwise Operative
//! Front: `{1}{B}{R}` Legendary Artifact Creature — Robot 3/2.
//!
//! Front face abilities:
//!   More Than Meets the Eye {B}{R} (GAP: alternate cast cost mechanic not modeled.)
//!   Sacrifice another artifact: Put a +1/+1 counter on Flamewar and convert it.
//!     Activate only as a sorcery. (GAP: sacrifice-other cost in ActivationCost not supported.)
//!   {1}, Discard your hand: Put all exiled cards you own with intel counters on them
//!     into your hand. (GAP: "intel counter" named counter on exiled cards lookup not modeled.)
//!
//! Back face (Flamewar, Streetwise Operative):
//!   Living metal (GAP: Living metal keyword not in enum — During your turn, this Vehicle
//!     is also a creature; but back face is already a creature so less relevant.)
//!   Menace, deathtouch
//!   Whenever Flamewar deals combat damage to a player, exile that many cards from the top
//!     of your library face down. Put an intel counter on each of them. Convert Flamewar.
//!   The convert (transform) half is wired (back-face only).
//!   (GAP: "exile that many cards face down + put an intel counter on each + play them while
//!     they have intel counters" — no exile-with-named-counter + counter-keyed play-permission
//!     primitive; ImpulseExile is fixed-count, this-turn-only and counter-less. Rider omitted.)
//!
//! GAP: More Than Meets the Eye alternate cast mechanic not modeled.
//! GAP: "Sacrifice another artifact" activation cost not modeled (sacrifice-other is GAP).
//! GAP: Discard your entire hand as activation cost not modeled.
//! GAP: Intel counters on exiled cards not modeled.
//! GAP: Convert keyword (same as Transform) recognized but day/night specifics deferred.
//! GAP: Living metal keyword not in KeywordAbility enum.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flamewar, Brash Veteran");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        // GAP: More Than Meets the Eye {B}{R} alternate cast not modeled
        // GAP: Sacrifice-other activation cost not modeled
        // GAP: "Discard your hand" activation cost not modeled
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Flamewar, Streetwise Operative");
    let back_robot_sub = reg.interner_mut().intern("Robot");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_robot_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black() | ColorSet::red(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Menace, KeywordAbility::Deathtouch],
            // GAP: Living metal keyword not in KeywordAbility enum
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: All activated abilities on front face involve unsupported costs
    //   (sacrifice-other, discard-hand) — omitted entirely.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back face only: whenever Flamewar deals combat damage to a player, convert it.
            // GAP: the "exile that many cards face down + intel counters + play them" rider is
            // not expressible; only the convert (transform) half is wired.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: back_damage_convert,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1), // back face only
    )
}

fn back_damage_convert(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile-that-many face down + intel counters + play-while-countered rider omitted.
    vec![Effect::Transform { target: trig.source }]
}
