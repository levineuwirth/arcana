//! Heliod, God of the Sun — `{3}{W}` 5/6 Legendary Enchantment Creature — God.
//! Indestructible
//! As long as your devotion to white is less than five, Heliod isn't a
//! creature.
//! Other creatures you control have vigilance.
//! {2}{W}{W}: Create a 2/1 white Cleric enchantment creature token.
//!
//! Indestructible is a base keyword. The devotion-gated "isn't a creature"
//! static and the "other creatures have vigilance" anthem are continuous
//! statics with no demonstrated primitive and are GAP'd. The {2}{W}{W} token
//! activated ability is wired.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heliod, God of the Sun");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    // Pre-intern the token's subtype.
    reg.interner_mut().intern("Cleric");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: "As long as your devotion to white is less than five, Heliod isn't
    // a creature." — devotion-gated type-removal static, no primitive.
    // GAP: "Other creatures you control have vigilance." — anthem static,
    // no primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{W}{W}: Create a 2/1 white Cleric enchantment creature token.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{W}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_cleric_token,
        }),
    )
}

fn make_cleric_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cleric = reg.interner().lookup("Cleric").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cleric);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: cleric,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
