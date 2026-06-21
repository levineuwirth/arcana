//! Oracle of Dust — `{4}{U}` 3/5 Eldrazi Processor (Devoid, colorless).
//!
//! Devoid (this card has no color)
//! {2}, Put a card an opponent owns from exile into that player's graveyard:
//! Draw a card, then discard a card.
//!
//! Devoid is not a `KeywordAbility` variant; it is realized by the colorless
//! color identity (the card has no colored pips). The activated ability's
//! mana cost ({2}) and effect (draw then discard) are wired; the additional
//! Processor cost ("put a card an opponent owns from exile into that
//! player's graveyard") has no `ActivationCost` field and is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oracle of Dust");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let processor = reg.interner_mut().intern("Processor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(processor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: cost — the Processor cost "put a card an opponent owns from
            //      exile into that player's graveyard" has no ActivationCost
            //      field; only the {2} mana portion is wired.
            text: "{2}, Put a card an opponent owns from exile into that player's graveyard: Draw a card, then discard a card.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_then_discard,
        }),
    )
}

fn draw_then_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}
