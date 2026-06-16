//! Saheeli, the Gifted — `{2}{U}{R}` Legendary Planeswalker — Saheeli,
//! starting loyalty 5.
//!
//! +1: Create a 1/1 colorless Servo artifact creature token.
//! +1: The next spell you cast this turn has affinity for artifacts. GAP: a
//!     next-spell cost-reduction (affinity) rider is not expressible.
//! −7: For each artifact you control, create a token that's a copy of it.
//!     Those tokens gain haste. Exile them at the next end step. GAP: bespoke
//!     board-wide copy-and-exile.
//!
//! "Saheeli, the Gifted can be your commander." — reminder text only.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saheeli, the Gifted");
    let saheeli = reg.interner_mut().intern("Saheeli");
    let _servo = reg.interner_mut().intern("Servo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saheeli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 colorless Servo artifact creature token.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_servo,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: The next spell you cast this turn has affinity for \
                       artifacts.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_affinity,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: For each artifact you control, create a token that's a \
                       copy of it. Those tokens gain haste. Exile those tokens at \
                       the beginning of the next end step.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

fn plus_one_servo(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let servo = reg.interner().lookup("Servo")
        .expect("Servo interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(servo);
    let token = TokenDefinition {
        name: servo,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn plus_one_affinity(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: next-spell affinity-for-artifacts cost reduction.
    Vec::new()
}

fn minus_seven(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: copy each artifact you control, grant haste, exile at next end step.
    Vec::new()
}
