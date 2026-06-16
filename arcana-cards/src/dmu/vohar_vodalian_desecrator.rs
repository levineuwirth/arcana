//! Vohar, Vodalian Desecrator — `{U}{B}` 1/2 Legendary Phyrexian
//! Merfolk Wizard.
//! "{T}: Draw a card, then discard a card. If you discarded an instant
//! or sorcery card this way, each opponent loses 1 life and you gain 1
//! life."
//! "{2}, Sacrifice Vohar: You may cast target instant or sorcery card
//! from your graveyard this turn. ... Activate only as a sorcery."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vohar, Vodalian Desecrator");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then discard a card.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_then_discard,
            })
            // GAP: "{2}, Sacrifice Vohar: cast target instant or sorcery from your
            // graveyard this turn ..." — no free/permission-to-cast-from-graveyard
            // primitive. Cost (mana + sacrifice self) IS expressible; effect is not.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Sacrifice Vohar: You may cast target instant or sorcery card from your graveyard this turn. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: noop,
            }),
    )
}

fn draw_then_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If you discarded an instant or sorcery card this way, each opponent
    // loses 1 life and you gain 1 life." — no post-discard conditional on the
    // discarded card's type is expressible. Draw + discard are emitted.
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}

fn noop(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
