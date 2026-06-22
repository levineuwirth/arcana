//! Meloku the Clouded Mirror — `{4}{U}` 2/4 Legendary Moonfolk Wizard (U).
//! Flying.
//! `{1}, Return a land you control to its owner's hand: Create a 1/1 blue
//! Illusion creature token with flying.`
//!
//! Flying is a base keyword. The activation pays {1}; the "Return a land you
//! control to its owner's hand" additional cost has no ActivationCost field
//! (there is no bounce-as-cost), so that cost piece is GAP'd. The effect mints
//! the flying Illusion token.

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
    let name = reg.interner_mut().intern("Meloku the Clouded Mirror");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let _illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Return a land you control to its owner's hand" additional
                // cost — no bounce-as-cost field on ActivationCost.
                text: "{1}, Return a land you control to its owner's hand: Create \
                       a 1/1 blue Illusion creature token with flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_illusion,
            }),
    )
}

fn make_illusion(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let illusion = match reg.interner().lookup("Illusion") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: illusion,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
