//! Tezzeret, Master of Metal — `{4}{U}{B}` Legendary Planeswalker —
//! Tezzeret, starting loyalty 5.
//!
//! * `+1`: Reveal cards from the top of your library until you reveal an
//!   artifact card; put it into your hand and the rest on the bottom in a
//!   random order. Expressed via `Effect::RevealUntil`.
//! * `−3`: Target opponent loses life equal to the number of artifacts you
//!   control. GAP'd (dynamic resolution-time life-loss amount).
//! * `−8`: Gain control of all artifacts and creatures target opponent
//!   controls. GAP'd (multi-object "gain control of all …" is not
//!   expressible from the activated-ability surface).

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Master of Metal");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal cards from the top of your library until you \
                       reveal an artifact card. Put that card into your hand \
                       and the rest on the bottom of your library in a random \
                       order.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_reveal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Target opponent loses life equal to the number of \
                       artifacts you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_drain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Gain control of all artifacts and creatures target \
                       opponent controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_control,
            }),
    )
}

fn plus_one_reveal(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::RevealUntil {
        player: ctx.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
        found_dest: RevealDest::Hand,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}

fn minus_three_drain(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic life-loss amount (= artifacts you control) at resolution.
    Vec::new()
}

fn minus_eight_control(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gain control of all artifacts and creatures target opponent
    // controls" — a multi-object board-wide control grab not expressible.
    Vec::new()
}
