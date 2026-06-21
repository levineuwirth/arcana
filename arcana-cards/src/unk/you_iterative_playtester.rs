//! You, Iterative Playtester — `{1}{W}` 1/3 Legendary Human Gamer.
//!
//! Oracle:
//! * {T}: Choose another target playtest card. Increase or decrease a number of
//!   your choice on it by 1 for as long as it remains on the stack or
//!   battlefield.
//! * {W}{U}{B}{R}{G}: Reveal the top eight cards of your library. Put each
//!   playtest card among them into your hand. Put the rest on the bottom of
//!   your library in a random order.
//!
//! Both activated abilities are GAP'd (effects only — costs are emitted):
//! "playtest card" is an Un-set / Mystery Booster playtest-card attribute that
//! is not modeled, so neither the numeral-mutation ability nor the
//! reveal-eight-take-playtest-cards ability can be expressed. The first
//! ability's target ("another playtest card") also has no filter, so it is
//! emitted without target requirements.

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
    let name = reg.interner_mut().intern("You, Iterative Playtester");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Choose another target playtest card. Increase or decrease a number of your choice on it by 1.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mutate_numeral,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}{B}{R}{G}: Reveal the top eight cards of your library. Put each playtest card among them into your hand. Put the rest on the bottom of your library in a random order.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reveal_eight_playtest,
            }),
    )
}

fn mutate_numeral(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: playtest-card numeral manipulation is an Un-set mechanic with no
    // modeling (no playtest-card attribute, no numeral state).
    Vec::new()
}

fn reveal_eight_playtest(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "playtest card" is not a modeled attribute, so the filtered
    // reveal-eight / take-playtest-cards selection cannot be expressed
    // (DigTopN is single-take and has no playtest-card filter).
    Vec::new()
}
