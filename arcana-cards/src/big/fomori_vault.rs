//! Fomori Vault — nonbasic land (March of the Machine, 2023).
//! "{T}: Add {C}." and "{3}, {T}, Discard a card: Look at the top X cards
//! of your library, where X is the number of artifacts you control. Put
//! one of those cards into your hand and the rest on the bottom of your
//! library in a random order." The dig is `Effect::DigTopN` with a
//! dynamic count computed from the battlefield.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fomori Vault");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, {T}, Discard a card: Look at the top X cards of \
                       your library, where X is the number of artifacts you \
                       control. Put one of those cards into your hand and \
                       the rest on the bottom of your library in a random \
                       order."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: dig,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn dig(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: x,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
