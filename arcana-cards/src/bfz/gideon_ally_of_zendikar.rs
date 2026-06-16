//! Gideon, Ally of Zendikar — `{2}{W}{W}` Legendary Planeswalker — Gideon, starting loyalty 4.
//! +1: Gideon becomes a 5/5 Human Soldier Ally with indestructible while still a PW + prevent all
//!     damage to him this turn. GAP — "becomes a creature while still a planeswalker" is not
//!     expressible in the demonstrated Effect surface.
//! 0: Create a 2/2 white Knight Ally creature token. IMPLEMENTED.
//! −4: You get an emblem with "Creatures you control get +1/+1." IMPLEMENTED (static anthem emblem).

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon, Ally of Zendikar");
    let sub = reg.interner_mut().intern("Gideon");
    let knight = reg.interner_mut().intern("Knight");
    let ally = reg.interner_mut().intern("Ally");
    let _emblem = reg.interner_mut().intern("Gideon, Ally of Zendikar emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

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

    let _ = (knight, ally);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until end of turn, Gideon, Ally of Zendikar becomes a 5/5 Human \
                       Soldier Ally creature with indestructible that's still a planeswalker. \
                       Prevent all damage that would be dealt to him this turn."
                    .into(),
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
                effect: plus_one_becomes_creature,
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
                effect: zero_make_knight,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: You get an emblem with \"Creatures you control get +1/+1.\"".into(),
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

/// `+1` — becomes a 5/5 creature while still a planeswalker + damage prevention.
fn plus_one_becomes_creature(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 5/5 creature while still a planeswalker" + "prevent all damage to him this
    // turn" are not expressible from the demonstrated Effect surface (no animate-self primitive).
    Vec::new()
}

/// `0` — create a 2/2 white Knight Ally creature token.
fn zero_make_knight(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").expect("Knight interned");
    let ally = reg.interner().lookup("Ally").expect("Ally interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(knight);
    token_subtypes.0.insert(ally);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−4` — static anthem emblem: creatures you control get +1/+1.
fn minus_four_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Gideon, Ally of Zendikar emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![ContinuousEffect::anthem(
                arcana_core::objects::NULL_OBJECT_ID,
                ctx.controller,
                1,
                1,
                Duration::Permanent,
            )],
            abilities: vec![],
        },
    }]
}
