//! Adorned Pouncer — `{1}{W}` 1/1 Creature — Cat.
//! Double strike.
//! Eternalize {3}{W}{W} ({3}{W}{W}, Exile this card from your graveyard: Create
//!   a token that's a copy of it, except it's a 4/4 black Zombie Cat with no
//!   mana cost. Eternalize only as a sorcery.)
//!
//! Eternalize is not an expressible KeywordAbility variant; it is modeled as a
//! graveyard-activated ability that mints the modified token copy directly.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adorned Pouncer");
    let cat = reg.interner_mut().intern("Cat");
    // Pre-intern the Zombie subtype for the eternalize token.
    let _zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{W}{W}, Exile this card from your graveyard: Create a token that's a copy of it, except it's a 4/4 black Zombie Cat with no mana cost. Eternalize only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}{W}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: eternalize_token,
        }),
    )
}

fn eternalize_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let name = reg
        .interner()
        .lookup("Adorned Pouncer")
        .unwrap_or_default();
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cat);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::DoubleStrike],
            abilities: vec![],
        },
    }]
}
