//! Tezzeret the Schemer — `{2}{U}{B}` Legendary Planeswalker — Tezzeret,
//! starting loyalty 5.
//!
//! * `+1`: Create a colorless artifact token named Etherium Cell with
//!   "{T}, Sacrifice this token: Add one mana of any color." The token's
//!   activated mana ability is not expressible on `TokenDefinition`
//!   (which carries only triggered abilities), so the token is minted as
//!   a plain colorless artifact and the activated ability is GAP'd.
//! * `−2`: Target creature gets +X/-X until end of turn, where X is the
//!   number of artifacts you control. Dynamic-X pump is not expressible
//!   (`Effect::Pump` power/toughness are fixed `i32`), so GAP'd.
//! * `−7`: Emblem — GAP'd (emblem creation w/ bespoke combat trigger).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret the Schemer");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let _cell = reg.interner_mut().intern("Etherium Cell");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
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
                text: "+1: Create a colorless artifact token named Etherium \
                       Cell with \"{T}, Sacrifice this token: Add one mana of \
                       any color.\"".into(),
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
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target creature gets +X/-X until end of turn, where \
                       X is the number of artifacts you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"At the beginning of combat \
                       on your turn, target artifact you control becomes an \
                       artifact creature with base power and toughness 5/5.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cell = reg.interner().lookup("Etherium Cell")
        .expect("Etherium Cell interned during register()");
    // GAP: the token's "{T}, Sacrifice this token: Add one mana of any
    // color" activated mana ability is not expressible on TokenDefinition
    // (only triggered abilities are carried). Minted as a plain artifact.
    let token = TokenDefinition {
        name: cell,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: SubtypeSet::default(),
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_two_pump(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic +X/-X (X = artifacts you control) — Effect::Pump
    // power/toughness are fixed i32; resolution-time amount not expressible.
    Vec::new()
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a bespoke begin-combat targeted-becomes-creature trigger.
    Vec::new()
}
