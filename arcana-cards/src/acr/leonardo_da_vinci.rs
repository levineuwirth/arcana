//! Leonardo da Vinci — `{2}{U}` 3/3 Legendary Human Artificer.
//!
//! {3}{U}{U}: Until end of turn, Thopters you control have base power and
//! toughness X/X, where X is the number of cards in your hand.
//! {2}{U}, {T}: Draw a card, then discard a card. If the discarded card
//! was an artifact card, exile it from your graveyard. If you do, create a
//! token that's a copy of it, except it's a 0/2 Thopter artifact creature
//! with flying in addition to its other types.
//!
//! Decomposed as: two activated abilities. The first sets each Thopter you
//! control to base X/X until end of turn (X = your hand size). The second
//! draws then discards; the conditional "if the discarded card was an
//! artifact, exile it and create a modified copy" depends on which card was
//! discarded mid-resolution and applies a custom token override, neither of
//! which is expressible, so that rider is GAP'd while the draw+discard is wired.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leonardo da Vinci");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let _thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}{U}: Until end of turn, Thopters you control have base power and toughness X/X, where X is the number of cards in your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: thopters_become_xx,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, {T}: Draw a card, then discard a card. If the discarded card was an artifact card, exile it from your graveyard. If you do, create a token that's a copy of it, except it's a 0/2 Thopter artifact creature with flying in addition to its other types.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot_and_copy,
            }),
    )
}

fn thopters_become_xx(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::hand_size(state, ctx.controller) as i32;
    let ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Thopter")
            .controlled_by(arcana_core::targets::ControllerConstraint::You),
        ctx.controller,
    );
    ids.into_iter()
        .map(|id| Effect::SetBasePT {
            target: id,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
        })
        .collect()
}

fn loot_and_copy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If the discarded card was an artifact, exile it and create a
    // 0/2 Thopter artifact-creature copy of it with flying" requires keying
    // off which card was discarded mid-resolution and applying a custom
    // token override on a copy — neither is expressible. Draw+discard wired.
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
