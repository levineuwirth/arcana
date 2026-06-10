//! Eden, Seat of the Sanctum — nonbasic land, subtype Town.
//! "{T}: Add {C}." and "{5}, {T}: Mill two cards. Then you may
//! sacrifice this land. When you do, return another target permanent
//! card from your graveyard to your hand."
//!
//! GAP: the reflexive "Then you may sacrifice this land. When you do,
//! return another target permanent card from your graveyard to your
//! hand." is not expressible (optional self-sacrifice + reflexive
//! trigger with its own target); only the Mill 2 resolves.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eden, Seat of the Sanctum");
    let town = reg.interner_mut().intern("Town");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(town);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
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
                text: "{5}, {T}: Mill two cards. Then you may sacrifice \
                       this land. When you do, return another target \
                       permanent card from your graveyard to your hand."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mill_two,
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

fn mill_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Then you may sacrifice this land. When you do, return
    // another target permanent card from your graveyard to your hand."
    // — optional self-sacrifice with a reflexive targeted trigger is
    // not expressible; only the mill is modeled.
    vec![Effect::Mill { player: ctx.controller, count: 2 }]
}
