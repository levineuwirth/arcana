//! Scrying Sheets — Snow Land.
//! "{T}: Add {C}." and "{1}{S}, {T}: Look at the top card of your
//! library. If that card is snow, you may reveal it and put it into
//! your hand." Snow supertype on the characteristics; the {S} snow
//! pip in the activation cost is approximated as generic mana (GAP);
//! the look-at-top is modeled with `Effect::DigTopN` filtered to snow
//! cards (the not-taken card going to the bottom instead of staying
//! on top is a documented fidelity GAP).

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scrying Sheets");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::SNOW),
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
                text: "{1}{S}, {T}: Look at the top card of your library. \
                       If that card is snow, you may reveal it and put it \
                       into your hand."
                    .into(),
                // GAP: the {S} snow-mana pip is not expressible in
                // ManaCost — approximated as generic ({1}{S} → {2}).
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_at_top,
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

fn look_at_top(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: if the top card is not snow (or is not taken) it should
    // stay on top of the library; DigTopN puts it on the bottom.
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 1,
        filter: Some(
            ObjectFilter::new()
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::SNOW)),
        ),
        rest: DigRest::BottomRandom,
    }]
}
