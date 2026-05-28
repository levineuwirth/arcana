//! Ellie and Alan, Paleontologists — `{2}{G}{W}{U}` 2/5 Legendary
//! green/white/blue Human Scientist.
//! `{T}, Exile a creature card from your graveyard: Discover X, where
//! X is the mana value of the exiled card. Activate only as a sorcery.`
//!
//! GAP: ExileFromGraveyard as an activation cost component is not
//! modeled (only sacrifice/tap/mana/life are wired); the ability
//! is registered with just the tap cost and the Discover effect uses
//! a fixed value since the exiled card's mana value is not available
//! at resolve time. Full fidelity requires a new activation-cost field
//! and a dynamic-X mechanism for the exiled card's CMC.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ellie and Alan, Paleontologists");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Exile a creature card from your graveyard: Discover X, where X is the mana value of the exiled card. Activate only as a sorcery.".into(),
                // GAP: activation cost cannot express "exile a creature card from graveyard";
                // registering tap-only as best approximation.
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
                effect: discover_exiled_cmc,
            }),
    )
}

fn discover_exiled_cmc(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic X (mana value of exiled card) is not accessible;
    // x_value is set by the engine from dynamic_x but no dynamic_x
    // is wired here. Using x_value if available, otherwise 0.
    let mv = ctx.x_value.unwrap_or(0);
    vec![Effect::Discover { player: ctx.controller, mana_value: mv }]
}
