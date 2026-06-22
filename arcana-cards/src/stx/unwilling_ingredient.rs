//! Unwilling Ingredient — `{B}` 1/1 Creature — Frog.
//! Menace.
//! "{2}{B}, Exile this card from your graveyard: You draw a card and you
//! lose 1 life."
//!
//! Keyword line → `keywords: vec![Menace]`. The graveyard-activated
//! ability costs {2}{B} and exiles this card from the graveyard
//! (`discard_self`/`exile_self` is the self-exile-from-graveyard cost;
//! activation_zone = Graveyard). Effect: draw a card + lose 1 life,
//! wrapped in a Sequence.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unwilling Ingredient");
    let frog = reg.interner_mut().intern("Frog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{B}, Exile this card from your graveyard: You draw a card and you lose 1 life."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_and_lose,
        }),
    )
}

fn draw_and_lose(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::LoseLife { player: ctx.controller, amount: 1 },
    ])]
}
