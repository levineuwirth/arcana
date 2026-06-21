//! Soratami Seer — `{4}{U}` 2/3 Moonfolk Wizard with Flying.
//! "{4}, Return two lands you control to their owner's hand: Discard all
//! the cards in your hand, then draw that many cards."
//!
//! Flying is an engine keyword. The activated ability's effect — discard
//! your whole hand, then draw that many cards — is modeled by counting
//! the hand at resolution, discarding that many, then drawing the same
//! number.
//!
//! GAP: the "Return two lands you control to their owner's hand"
//! additional cost cannot be expressed — `ActivationCost` has no
//! return-to-hand cost field (only mana/tap/sacrifice/discard/life/etc.),
//! so only the `{4}` mana portion of the cost is modeled.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soratami Seer");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}, Return two lands you control to their owner's hand: \
                       Discard all the cards in your hand, then draw that many cards."
                    .into(),
                // GAP: the "return two lands" additional cost is not expressible;
                // only the {4} mana portion is modeled.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: discard_hand_redraw,
            }),
    )
}

fn discard_hand_redraw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::hand_size(state, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![
        Effect::Discard {
            player: ctx.controller,
            count: n,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: ctx.controller, count: n },
    ]
}
