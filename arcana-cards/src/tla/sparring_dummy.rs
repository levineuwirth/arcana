//! Sparring Dummy — `{1}{G}` 1/3 green Artifact Creature — Scarecrow
//! with Defender.
//!
//! Rules text:
//! * Defender.
//! * "{T}: Mill a card. You may put a land card milled this way into
//!   your hand. You gain 2 life if a Lesson card is milled this way."
//!   — the base mill is expressible; the milled-card-dependent riders
//!   (return a land milled this way / gain 2 life if a Lesson milled
//!   this way) have no primitive and are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sparring Dummy");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Mill a card. You may put a land card milled this way into your hand. You gain 2 life if a Lesson card is milled this way."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_mill,
            }),
    )
}

fn tap_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "You may put a land card milled this way into your hand." and
    // "You gain 2 life if a Lesson card is milled this way." — milled-card
    // -dependent riders have no expressible primitive.
    vec![Effect::Mill {
        player: ctx.controller,
        count: 1,
    }]
}
