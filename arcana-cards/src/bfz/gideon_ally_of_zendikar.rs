//! Gideon, Ally of Zendikar — `{2}{W}{W}` Legendary Planeswalker — Gideon.
//! Starting loyalty 4 (oracle).
//!
//! +1: Until end of turn, Gideon becomes a 5/5 Human Soldier Ally creature
//!     with indestructible that's still a planeswalker. Prevent all damage
//!     that would be dealt to him this turn.
//!     GAP: animate-self-as-a-typed-creature-while-still-a-planeswalker
//!     (becomes 5/5 Human Soldier Ally + indestructible + still PW + prevent
//!     all damage to him) is not expressible as a single loyalty effect.
//! 0: Create a 2/2 white Knight Ally creature token.
//! −4: You get an emblem with "Creatures you control get +1/+1."
//!     GAP: emblem creation not modeled.

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
    let name = reg.interner_mut().intern("Gideon, Ally of Zendikar");
    let gideon = reg.interner_mut().intern("Gideon");
    // Intern token subtypes up front so the resolver's lookup() succeeds.
    let _knight = reg.interner_mut().intern("Knight");
    let _ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gideon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until end of turn, Gideon becomes a 5/5 Human \
                       Soldier Ally creature with indestructible that's still \
                       a planeswalker. Prevent all damage to him this turn.".into(),
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
                effect: plus_one_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create a 2/2 white Knight Ally creature token.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: You get an emblem with \"Creatures you control get \
                       +1/+1.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_emblem,
            }),
    )
}

fn plus_one_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: animate self as a 5/5 Human Soldier Ally + indestructible while
    //      still a planeswalker, plus prevent all damage to him this turn.
    Vec::new()
}

fn zero_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg
        .interner()
        .lookup("Knight")
        .expect("Knight interned during register()");
    let ally = reg
        .interner()
        .lookup("Ally")
        .expect("Ally interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(knight);
    token_subtypes.0.insert(ally);
    let token = TokenDefinition {
        name: knight,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_four_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with "Creatures you control get +1/+1" — emblem creation
    //      not modeled.
    Vec::new()
}
