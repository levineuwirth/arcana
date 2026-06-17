//! Svella, Ice Shaper — `{1}{R}{G}` 2/4 Legendary Snow Troll Warrior.
//! {3}, {T}: Create a colorless snow artifact token named Icy Manalith with
//!   "{T}: Add one mana of any color."
//! {6}{R}{G}, {T}: Look at the top four cards of your library; you may cast a
//!   spell from among them without paying its mana cost; bottom the rest.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Svella, Ice Shaper");
    let troll = reg.interner_mut().intern("Troll");
    let warrior = reg.interner_mut().intern("Warrior");
    let _manalith = reg.interner_mut().intern("Icy Manalith");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, {T}: Create a colorless snow artifact token named Icy Manalith with \"{T}: Add one mana of any color.\"".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_manalith,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{R}{G}, {T}: Look at the top four cards of your library. You may cast a spell from among them without paying its mana cost. Put the rest on the bottom of your library in a random order.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{R}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: cast_from_top,
            }),
    )
}

fn make_manalith(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let nm = reg.interner().lookup("Icy Manalith").unwrap_or_default();
    // GAP: the token's "{T}: Add one mana of any color" activated ability can't
    // be authored on a TokenDefinition; the token is minted with bare bones.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: nm,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn cast_from_top(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top four, you may cast one for free, bottom the rest"
    // has no Effect — DigTopN goes to hand, not a free cast.
    Vec::new()
}
