//! Raving Visionary — `{1}{U}` 1/1 Creature — Merfolk Wizard.
//!
//! Oracle:
//! * `{U}, {T}: Draw a card, then discard a card.`
//! * Delirium — `{2}{U}, {T}: Draw a card.` Activate only if there are four
//!   or more card types among cards in your graveyard.

use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raving Visionary");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Draw a card, then discard a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, {T}: Draw a card. Activate only if there are four or more card types among cards in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                    tap: true,
                    activation_condition: Some(if_delirium),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            }),
    )
}

fn loot(
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

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn if_delirium(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::delirium(s, you)
}
