//! Oketra the True — `{3}{W}` 3/6 legendary God with Double strike and
//! Indestructible.
//! "Oketra can't attack or block unless you control at least three
//! other creatures. {3}{W}: Create a 1/1 white Warrior creature token
//! with vigilance."
//!
//! Abilities:
//! 1. Double strike, Indestructible (keywords).
//! 2. Static attack/block restriction — pure static, not expressible
//!    as a triggered/activated ability; GAP'd.
//! 3. {3}{W}: Create a 1/1 white Warrior token with vigilance.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oketra the True");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::DoubleStrike, KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: "Oketra can't attack or block unless you control at least
    // three other creatures" is a pure static restriction — not
    // expressible as a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{W}: Create a 1/1 white Warrior creature token with vigilance.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{W}").unwrap(),
                ..ActivationCost::default()
            },
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_warrior_token,
        }),
    )
}

fn make_warrior_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: warrior,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Vigilance],
            abilities: vec![],
        },
    }]
}
