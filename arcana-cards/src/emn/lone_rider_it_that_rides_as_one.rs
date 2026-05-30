//! Lone Rider // It That Rides as One — `{1}{W}` white Creature — Human Knight 1/1.
//! Front: First strike, lifelink.
//! At the beginning of the end step, if you gained 3 or more life this turn,
//! transform this creature.
//! Back: It That Rides as One — Creature — Eldrazi Horror.
//! First strike, trample, lifelink.
//!
//! GAP: "if you gained 3 or more life this turn" — there is no
//!   script::life_gained_this_turn helper. The transform trigger fires
//!   unconditionally at end step (intervening_if cannot check cumulative
//!   life-gained this turn). Mark as GAP.
//! GAP: back-face-only triggered abilities not modeled (back face has no
//!   triggered abilities beyond its static keywords, so no additional GAP here).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lone Rider");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: It That Rides as One — Creature — Eldrazi Horror 4/4
    let back_name = reg.interner_mut().intern("It That Rides as One");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eldrazi_sub);
    back_subtypes.0.insert(horror_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![
                KeywordAbility::FirstStrike,
                KeywordAbility::Trample,
                KeywordAbility::Lifelink,
            ],
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // At beginning of end step, if you gained 3+ life this turn, transform.
            // GAP: intervening_if "3+ life gained this turn" not checkable — fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: transform_if_life_gained,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// GAP: should only transform if controller gained 3+ life this turn;
/// no script helper for cumulative life-gained-this-turn exists.
/// Fires unconditionally.
fn transform_if_life_gained(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
