//! Vivien's Arkbow — `{1}{G}` legendary artifact.
//! "{X}, {T}, Discard a card: Look at the top X cards of your library.
//! You may put a creature card with mana value X or less from among them
//! onto the battlefield. Put the rest on the bottom of your library in a
//! random order." Modeled with `Effect::RevealUntil` to the battlefield
//! capped at X reveals — a documented fidelity approximation (the first
//! matching card is taken deterministically; the printed 'look + may
//! choose' is a choice).

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivien's Arkbow");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{X}, {T}, Discard a card: Look at the top X cards \
                       of your library. You may put a creature card with \
                       mana value X or less from among them onto the \
                       battlefield. Put the rest on the bottom of your \
                       library in a random order."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
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
                effect: dig_creature_to_battlefield,
            },
        ),
    )
}

fn dig_creature_to_battlefield(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    if x == 0 {
        return Vec::new();
    }
    // GAP (fidelity): RevealUntil puts the FIRST creature card with mana
    // value <= X onto the battlefield; the printed effect lets the player
    // look at all X and choose (or decline).
    vec![Effect::RevealUntil {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_max_cmc(x),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(x),
    }]
}
