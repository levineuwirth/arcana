//! Kefnet the Mindful — `{2}{U}` Legendary 5/5 God with Flying and
//! Indestructible.
//! "Kefnet can't attack or block unless you have seven or more cards in hand."
//! "{3}{U}: Draw a card, then you may return a land you control to its owner's
//!  hand."

use arcana_core::effects::{Effect, KeywordAbility, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP (static): "Kefnet can't attack or block unless you have seven or more
// cards in hand." — a conditional can't-attack/can't-block restriction is not
// expressible for this shape.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kefnet the Mindful");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Indestructible],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Draw a card, then you may return a land you control to its owner's hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_then_bounce_land,
            }),
    )
}

fn draw_then_bounce_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): "you may return A land" is modeled as the player-chosen
    // any-number bounce, so the player could return more than one land.
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::ChooseAnyNumberFromZone {
            chooser: ctx.controller,
            zone: Zone::Battlefield,
            filter: ObjectFilter::permanent()
                .with_types(TypeLine::LAND.into())
                .controlled_by(ControllerConstraint::You),
            action: PickAction::ReturnToHand,
        },
    ])]
}
