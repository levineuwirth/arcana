//! Civilized Scholar // Homicidal Brute — `{2}{U}` blue Human Advisor 0/1 (front) /
//! Human Mutant (back). Transform creature.
//!
//! Front face (Civilized Scholar):
//!   {T}: Draw a card, then discard a card. If a creature card is discarded this way,
//!        untap this creature, then transform it.
//!
//! Back face (Homicidal Brute):
//!   At the beginning of your end step, if this creature didn't attack this turn,
//!   tap this creature, then transform it.
//!
//! GAP: Front-face activation "if a creature card is discarded this way, untap then transform"
//!   — checking whether the specifically discarded card was a creature card is not expressible
//!   in the Effect catalog (no conditional-on-discarded-card-type variant). Modeled as:
//!   draw + discard, then unconditionally untap + transform. (The transform fires always,
//!   not only when a creature card is discarded — verify will flag this.)
//! GAP: Back-face triggered ability "if this creature didn't attack this turn" — the
//!   "didn't attack" intervening condition is not expressible as TriggerCondition or
//!   intervening_if. Back-face-only triggered ability not auto-installed on transform.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Civilized Scholar");
    let human_sub = reg.interner_mut().intern("Human");
    let advisor_sub = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(advisor_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Homicidal Brute
    let back_name = reg.interner_mut().intern("Homicidal Brute");
    let human_sub2 = reg.interner_mut().intern("Human");
    let mutant_sub = reg.interner_mut().intern("Mutant");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub2);
    back_subtypes.0.insert(mutant_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(1)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {T}: draw a card, discard a card, then (unconditionally in this
            // approximation) untap and transform.
            // GAP: should only untap+transform if discarded card was a creature card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then discard a card. If a creature card is discarded this way, untap this creature, then transform it.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: scholar_tap,
            }),
            // GAP: back-face "At the beginning of your end step, if this creature didn't
            // attack this turn, tap then transform" — back-face-only triggered ability
            // not modeled.
    )
}

fn scholar_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: unconditional transform; should only fire when a creature card is discarded.
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard { player: ctx.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::Untap { target: ctx.source },
        Effect::Transform { target: ctx.source },
    ]
}
