//! Bloodline Keeper // Lord of Lineage — `{2}{B}{B}` 3/3 black Vampire with Flying.
//!
//! Front face (Bloodline Keeper):
//!   Flying
//!   {T}: Create a 2/2 black Vampire creature token with flying.
//!   {B}: Transform this creature. Activate only if you control five or more Vampires.
//!     (GAP: "activate only if you control five or more Vampires" condition not enforceable
//!      in ActivationCost; condition emitted as a comment and the activation fires freely.)
//!
//! Back face (Lord of Lineage):
//!   Flying
//!   Other Vampire creatures you control get +2/+2. (GAP: static pump aura not modeled.)
//!   {T}: Create a 2/2 black Vampire creature token with flying.
//!   (GAP: Back-face tap ability not auto-installed on transform.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodline Keeper");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Lord of Lineage");
    let back_vampire_sub = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_vampire_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            // GAP: "Other Vampire creatures you control get +2/+2" static pump not modeled
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern Vampire subtype for use in token creation
    let _vampire_tok = reg.interner_mut().intern("Vampire");

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}: Create a 2/2 black Vampire creature token with flying.
            // (face_gate: None — shared by both faces in spirit; back face GAP'd)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Create a 2/2 black Vampire creature token with flying.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: create_vampire_token,
            })
            // {B}: Transform this creature.
            // GAP: "Activate only if you control five or more Vampires" not enforced.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: Transform this creature. Activate only if you control five or more Vampires.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: do_transform,
            })
    )
}

fn create_vampire_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").expect("Vampire interned during register");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let token = TokenDefinition {
        name: vampire,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn do_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Activate only if you control five or more Vampires" condition not enforced
    vec![Effect::Transform { target: ctx.source }]
}
